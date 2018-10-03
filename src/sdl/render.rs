use crate::member::{self, Membership};
use crate::Complex;
use crate::Complex64;
use crossbeam_utils::thread;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

const MAX_ITERATIONS: u32 = 256;

pub fn render_texture(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    bounds: (usize, usize),
    upper_left: Complex64,
    pixel_delta: f64,
) {
    let threads = sdl2::cpuinfo::cpu_count() as usize;

    let rows_per_band = bounds.1 / threads + 1;

    let mut memberships = vec![Membership::No(0); bounds.0 * bounds.1];
    let bands: Vec<&mut [Membership]> = memberships.chunks_mut(rows_per_band * bounds.0).collect();

    thread::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, pixel_delta);

            spawner.spawn(move || {
                map_membership(band, band_bounds, band_upper_left, pixel_delta);
            });
        }
    });

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for row in 0..bounds.1 {
                for column in 0..bounds.0 {
                    let offset = row * pitch + column * 3;
                    let membership = memberships[row * bounds.0 + column];

                    // Use the formula q = (x + y - 1) / y for dividing MAX_ITERATIONS with 2 and ceiling the output.
                    // We ceil the output since a floored number (which is the normal behaviour) would make
                    // `iterations % halfway` yield a 0, which produces some red color where it's supposed to be
                    // totally white ((255, 0, 0) instead of (255, 255, 255))
                    let halfway = (MAX_ITERATIONS + 2 - 1) / 2;
                    let color_factor = 255.0 / f64::from(halfway);

                    let color = match membership {
                        Membership::Yes => (0, 0, 0),
                        Membership::No(iterations) if iterations < halfway => {
                            let c = (f64::from(iterations) * color_factor).round() as u8;
                            (c, 0, 0)
                        }
                        Membership::No(iterations) => {
                            let iter_after_halfway = iterations % halfway;
                            let c = (f64::from(iter_after_halfway) * color_factor).round() as u8;
                            (255, c, c)
                        }
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

pub fn map_membership(
    memberships: &mut [Membership],
    bounds: (usize, usize),
    upper_left: Complex64,
    pixel_delta: f64,
) {
    assert!(memberships.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, pixel_delta);

            memberships[row * bounds.0 + column] = member::calculate(point, MAX_ITERATIONS);
        }
    }
}
