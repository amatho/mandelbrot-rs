use crate::Complex64;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod render;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn start(bounds: (usize, usize), mut upper_left: Complex64) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Visualization", bounds.0 as u32, bounds.1 as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    canvas.present();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, bounds.0 as u32, bounds.1 as u32)
        .unwrap();

    // At this pixel_delta you can see approx. the entire set when using the default bounds
    let mut pixel_delta = 0.003_141_5;
    render::render_texture(&mut canvas, &mut texture, bounds, upper_left, pixel_delta);

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
                        continue;
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

                    render::render_texture(
                        &mut canvas,
                        &mut texture,
                        bounds,
                        upper_left,
                        pixel_delta,
                    );
                }
                _ => {}
            }
        }
    }
}
