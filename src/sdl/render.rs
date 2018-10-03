use crate::member::{self, Membership};
use crate::Complex;
use crate::Complex64;
use crossbeam_utils::thread;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

pub fn render_texture(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    bounds: (usize, usize),
    upper_left: Complex64,
    pixel_delta: f64,
) {
    let threads = sdl2::cpuinfo::cpu_count() as usize;

    let rows_per_band = bounds.1 / threads + 1;

    let mut escape_times = vec![0; bounds.0 * bounds.1];
    let bands: Vec<&mut [u8]> = escape_times.chunks_mut(rows_per_band * bounds.0).collect();

    thread::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, pixel_delta);

            spawner.spawn(move || {
                calculate_escape_times(band, band_bounds, band_upper_left, pixel_delta);
            });
        }
    });

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for row in 0..bounds.1 {
                for column in 0..bounds.0 {
                    let offset = row * pitch + column * 3;
                    let time = escape_times[row * bounds.0 + column];

                    let color = if time == 255 {
                        (0, 0, 0)
                    } else if time < 128 {
                        let c = time * 2;
                        (c, 0, 0)
                    } else {
                        let c = (time % 128) * 2;
                        (255, c, c)
                    };

                    buffer[offset] = color.0;
                    buffer[offset + 1] = color.1;
                    buffer[offset + 2] = color.2;
                }
            }
        })
        .unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex64,
    pixel_delta: f64,
) -> Complex64 {
    assert!(pixel.0 < bounds.0);
    assert!(pixel.1 < bounds.1);

    let re = upper_left.re + pixel_delta * pixel.0 as f64;
    let im = upper_left.im - pixel_delta * pixel.1 as f64;

    Complex::new(re, im)
}

pub fn calculate_escape_times(
    times: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex64,
    pixel_delta: f64,
) {
    assert!(times.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, pixel_delta);

            let time = match member::calculate(point, 254) {
                Membership::Yes => 255,
                Membership::No(count) => count as u8,
            };

            times[row * bounds.0 + column] = time;
        }
    }
}
