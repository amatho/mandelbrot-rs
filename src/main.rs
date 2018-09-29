use mandelbrot;
use mandelbrot::complex::Complex;
use mandelbrot::complex::Complex64;

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

fn parse_complex(s: &str) -> Option<Complex64> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex::new(re, im)),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let (bounds, upper_left, lower_right) = if args.len() == 1 {
        println!("Running with default arguments: 800x800 -1.20,0.35 -1,0.20");
        let bounds = (800, 800);
        let upper_left = Complex::new(-1.20, 0.35);
        let lower_right = Complex::new(-1.0, 0.20);

        (bounds, upper_left, lower_right)
    } else if args.len() != 4 {
        eprintln!("Usage: mandelbrot PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!("Example: {} 800x800 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    } else {
        let bounds = parse_pair(&args[1], 'x').expect("error parsing image dimensions");
        let upper_left = parse_complex(&args[2]).expect("error parsing upper left corner point");
        let lower_right = parse_complex(&args[3]).expect("error parsing lower right corner point");

        (bounds, upper_left, lower_right)
    };

    mandelbrot::run(bounds, upper_left, lower_right);
}
