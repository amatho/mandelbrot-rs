use mandelbrot;
use mandelbrot::complex::Complex;
use mandelbrot::complex::Complexf64;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

fn parse_complex(s: &str) -> Option<Complexf64> {
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

    sdl_setup(bounds, upper_left, lower_right);
}

fn sdl_setup(bounds: (usize, usize), mut upper_left: Complexf64, mut lower_right: Complexf64) {
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

    render_texture(&mut canvas, bounds, upper_left, lower_right);

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
                    let (ul_transform, lr_transform) = match k {
                        Keycode::Up => (Complex::new(0.05, -0.05), Complex::new(-0.05, 0.05)),
                        Keycode::Down => (Complex::new(-0.05, 0.05), Complex::new(0.05, -0.05)),
                        Keycode::A => (Complex::new(-0.05, 0.0), Complex::new(-0.05, 0.0)),
                        _ => continue 'running,
                    };

                    upper_left += ul_transform;
                    lower_right += lr_transform;

                    render_texture(&mut canvas, bounds, upper_left, lower_right);
                }
                _ => {}
            }
        }
    }
}

fn render_texture(
    canvas: &mut Canvas<Window>,
    bounds: (usize, usize),
    upper_left: Complexf64,
    lower_right: Complexf64,
) {
    let mut pixels = vec![0; bounds.0 * bounds.1 * 4];
    mandelbrot::render(&mut pixels, bounds, upper_left, lower_right);

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_static(
            Some(PixelFormatEnum::RGBA8888),
            bounds.0 as u32,
            bounds.1 as u32,
        ).unwrap();
    texture.update(None, &pixels, bounds.0 * 4).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}
