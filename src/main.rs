#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mandelbrot::{self, Complex, Complex64};

use std::env;
use std::str::FromStr;

// Parse a pair from a given string, using the given separator to determine which values to parse
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

// Parse a pair and create a new complex number from the parsed values
fn parse_complex(s: &str) -> Option<Complex64> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex::new(re, im)),
    }
}

// Prints program usage and exits with a non-zero exit code
fn usage_and_exit(program_path: &str) -> ! {
    eprintln!("Usage: mandelbrot PIXELS UPPERLEFT");
    eprintln!("Example: {} 800x800 -1.95,1.15", program_path);
    eprintln!("PIXELS and/or UPPERLEFT are optional, and will use default values if omitted.");
    std::process::exit(1);
}

fn main() {
    let mut args_iter = env::args();
    let program_path = args_iter.next().unwrap();
    let args: Vec<_> = args_iter.collect();

    // Check how many command line arguments were given.
    // If any arguments are submitted we use default values.
    let (bounds, upper_left) = match args.len() {
        0 => {
            println!("Running with default arguments: 800x800 -1.95,1.15");
            let bounds = (800, 800);
            let upper_left = Complex::new(-1.95, 1.15);

            (bounds, upper_left)
        }
        1 => match (parse_pair(&args[0], 'x'), parse_complex(&args[0])) {
            (Some(b), None) => (b, Complex::new(-1.95, 1.15)),
            (None, Some(c)) => ((800, 800), c),
            _ => usage_and_exit(&program_path),
        },
        2 => {
            let bounds = parse_pair(&args[0], 'x').expect("error parsing image dimensions");
            let upper_left =
                parse_complex(&args[1]).expect("error parsing upper left corner point");

            (bounds, upper_left)
        }
        _ => usage_and_exit(&program_path),
    };

    mandelbrot::run(bounds, upper_left);
}
