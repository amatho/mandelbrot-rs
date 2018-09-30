use super::member::{self, Membership};
use super::Complex;
use super::Complex64;
use crossbeam_utils::thread;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

pub fn start(bounds: (usize, usize), mut upper_left: Complex64) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Visualization", bounds.0 as u32, bounds.1 as u32)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, bounds.0 as u32, bounds.1 as u32)
        .unwrap();

    // At this pixel_delta you can see approx. the entire set
    let mut pixel_delta = 0.003_138_428_376_721;
    render_texture(&mut canvas, &mut texture, bounds, upper_left, pixel_delta);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    let keyboard_mappings = [
                        Keycode::Up,
                        Keycode::Down,
                        Keycode::W,
                        Keycode::S,
                        Keycode::A,
                        Keycode::D,
                    ];

                    if !keyboard_mappings.contains(&k) {
                        continue 'running;
                    }

                    let shortest_bound = if bounds.0 < bounds.1 {
                        bounds.0
                    } else {
                        bounds.1
                    };

                    let step = pixel_delta * (shortest_bound / 10) as f64;
                    let zoom_factor = 1.1;

                    let mut transform_re = 0.0;
                    let mut transform_im = 0.0;

                    if k == Keycode::Up {
                        pixel_delta /= zoom_factor;
                        transform_re += pixel_delta * (bounds.0 / 18) as f64;
                        transform_im -= pixel_delta * (bounds.1 / 18) as f64;
                    }
                    if k == Keycode::Down {
                        pixel_delta *= zoom_factor;
                        transform_re -= pixel_delta * (bounds.0 / 18) as f64;
                        transform_im += pixel_delta * (bounds.1 / 18) as f64;
                    }
                    if k == Keycode::A {
                        transform_re -= step;
                    }
                    if k == Keycode::D {
                        transform_re += step;
                    }
                    if k == Keycode::S {
                        transform_im -= step;
                    }
                    if k == Keycode::W {
                        transform_im += step;
                    }

                    upper_left.transform(transform_re, transform_im);
                    println!("New pos: {}", upper_left);

                    render_texture(&mut canvas, &mut texture, bounds, upper_left, pixel_delta);
                }
                _ => {}
            }
        }
    }
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

fn render_texture(
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
