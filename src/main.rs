use mandelbrot;
use mandelbrot::complex::Complex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex::new(re, im)),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: mandelbrot PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!("Example: {} 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[1], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[2]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[3]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    mandelbrot::render(&mut pixels, bounds, upper_left, lower_right);
    sdl_setup(&pixels, bounds);
}

fn sdl_setup(pixels: &[u8], bounds: (usize, usize)) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Visualization", bounds.0 as u32, bounds.1 as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut rgb_pixels = vec![0; bounds.0 * bounds.1 * 4];

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let index = row * bounds.0 + column;
            let pixel = pixels[index];
            rgb_pixels[index * 4] = pixel;
            rgb_pixels[index * 4 + 1] = pixel;
            rgb_pixels[index * 4 + 2] = pixel;
            rgb_pixels[index * 4 + 3] = pixel;
        }
    }

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(
            Some(PixelFormatEnum::RGBA8888),
            bounds.0 as u32,
            bounds.1 as u32,
        ).unwrap();
    texture.update(None, &rgb_pixels, bounds.0 * 4).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}
