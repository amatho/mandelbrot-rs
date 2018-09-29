use super::complex::Complex;

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
pub fn calculate(c: Complex<f64>, limit: u32) -> Membership {
    let mut z = Complex::<f64>::identity();

    for i in 0..limit {
        z = z * z + c;
        if z.abs_squared() > 4.0 {
            return Membership::No(i);
        }
    }

    Membership::Yes
}
