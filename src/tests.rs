use super::complex;

mod complex_tests {
    use super::complex::Complex;

    #[test]
    fn add() {
        assert_eq!(
            Complex { re: 2, im: 3 } + Complex { re: 3, im: 2 },
            Complex { re: 5, im: 5 }
        );
    }

    #[test]
    fn add_assign() {
        let mut t = Complex { re: 1, im: 1 };
        t += Complex { re: 2, im: 1 };
        assert_eq!(t, Complex { re: 3, im: 2 });
    }

    #[test]
    fn sub() {
        assert_eq!(
            Complex { re: 5, im: 4 } - Complex { re: 2, im: 1 },
            Complex { re: 3, im: 3 }
        );
    }

    #[test]
    fn sub_assign() {
        let mut t = Complex { re: 1, im: 1 };
        t -= Complex { re: 2, im: 1 };
        assert_eq!(t, Complex { re: -1, im: 0 });
    }

    #[test]
    fn sign_formatting() {
        let negative_complex = Complex { re: 1, im: -2 };
        let negative_complex = format!("{}", negative_complex);

        assert_eq!(negative_complex, "1-2i".to_string());

        let identity = Complex::<i32>::identity();
        let identity = format!("{}", identity);

        assert_eq!(identity, "0+0i".to_string());
    }
}
