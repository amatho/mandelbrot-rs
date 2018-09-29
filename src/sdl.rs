use super::member::{self, Membership};
use super::Complex;
use super::Complexf64;
use crossbeam_utils::thread;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

pub fn start(bounds: (usize, usize), mut upper_left: Complexf64, mut lower_right: Complexf64) {
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

    render_texture(&mut canvas, &mut texture, bounds, upper_left, lower_right);

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
                    let step = 0.02;

                    let (ul_transform, lr_transform) = match k {
                        Keycode::Up => (Complex::new(0.05, -0.05), Complex::new(-0.05, 0.05)),
                        Keycode::Down => (Complex::new(-0.05, 0.05), Complex::new(0.05, -0.05)),
                        Keycode::A => (Complex::new(-step, 0.0), Complex::new(-step, 0.0)),
                        Keycode::D => (Complex::new(step, 0.0), Complex::new(step, 0.0)),
                        Keycode::S => (Complex::new(0.0, -step), Complex::new(0.0, -step)),
                        Keycode::W => (Complex::new(0.0, step), Complex::new(0.0, step)),
                        _ => continue 'running,
                    };

                    upper_left += ul_transform;
                    lower_right += lr_transform;

                    render_texture(&mut canvas, &mut texture, bounds, upper_left, lower_right);
                }
                _ => {}
            }
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

pub fn calculate_pixels(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            let pixel = match member::calculate(point, 255) {
                Membership::Yes => 0,
                Membership::No(count) => 255 - count as u8,
            };

            pixels[row * bounds.0 + column] = pixel;
        }
    }
}

fn render_texture(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    bounds: (usize, usize),
    upper_left: Complexf64,
    lower_right: Complexf64,
) {
    let threads = sdl2::cpuinfo::cpu_count() as usize;

    let rows_per_band = bounds.1 / threads + 1;

    let mut pixels = vec![0; bounds.0 * bounds.1];
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    thread::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move || {
                calculate_pixels(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    });

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for row in 0..bounds.1 {
                for column in 0..bounds.0 {
                    let offset = row * pitch + column * 3;
                    let pixel = pixels[row * bounds.0 + column];
                    buffer[offset] = pixel;
                    buffer[offset + 1] = pixel;
                    buffer[offset + 2] = pixel;
                }
            }
        })
        .unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}
