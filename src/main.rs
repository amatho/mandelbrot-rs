mod complex;

fn main() {
    let mut test = complex::Complex::<u8>::identity();
    test += complex::Complex { re: 1, im: 2 };

    println!("Value of test: {}", test);
}
