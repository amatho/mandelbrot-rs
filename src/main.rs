use mandelbrot::complex::Complex;

fn main() {
    println!("Identity complex: {}", Complex::<i32>::identity());
    println!("Negative complex: {}", Complex { re: 1, im: -2 });
}
