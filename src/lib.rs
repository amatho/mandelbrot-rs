pub mod complex;

use self::complex::Complex;

/// Enum for representing a complex number's membership in the Mandelbrot set.
/// The `No(u32)` variant contains the number of tries before deciding that this
/// number is not part of the set.
pub enum Membership {
    Yes,
    No(u32),
}

/// Calculate whether the complex number `c` is part of the Mandelbrot set.
/// Use `limit` number of tries before deciding that `c` probably is part of the
/// set.
pub fn member(c: Complex<f64>, limit: u32) -> Membership {
    let mut z = Complex::<f64>::identity();

    for i in 0..limit {
        z = z * z + c;
        if z.abs_squared() > 4.0 {
            return Membership::No(i);
        }
    }

    Membership::Yes
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

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1 * 4);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            let pixel = match member(point, 255) {
                Membership::Yes => 0,
                Membership::No(count) => 255 - count as u8,
            };

            let index = row * bounds.0 + column;
            pixels[index * 4] = 255;
            pixels[index * 4 + 1] = pixel;
            pixels[index * 4 + 2] = pixel;
            pixels[index * 4 + 3] = pixel;
        }
    }
}

#[cfg(test)]
mod tests;
