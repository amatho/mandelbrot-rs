use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Data representation of a complex number
#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub struct Complex<T> {
	/// The real component
	pub re: T,
	/// The imaginary component
	pub im: T,
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

impl<T> Complex<T> {
	pub fn new(re: T, im: T) -> Complex<T> {
		Complex { re, im }
	}
}

impl<T> Complex<T>
where
	T: Clone + Mul<Output = T> + Add<Output = T>,
{
	pub fn abs_squared(&self) -> T {
		self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
	}
}

impl Complex32 {
	pub fn abs(self) -> f32 {
		(self.re.powi(2) + self.im.powi(2)).sqrt()
	}
}

impl Complex64 {
	pub fn abs(self) -> f64 {
		(self.re.powi(2) + self.im.powi(2)).sqrt()
	}
}

impl<T> Complex<T>
where
	T: AddAssign,
{
	pub fn transform(&mut self, re: T, im: T) {
		self.re += re;
		self.im += im;
	}

	pub fn transform_re(&mut self, re: T) {
		self.re += re;
	}

	pub fn transform_im(&mut self, im: T) {
		self.im += im;
	}
}

// Macro for implementing Add and Sub.
// $trt is which of the two traits to implement, $fn_name is
// the function name required for the implementation.
// $op is the operator used in the impl.
macro_rules! addsub_impl {
	($trt:ident, $fn_name:ident, $op:tt) => {
		impl<T> $trt for Complex<T>
		where
			T: $trt<Output=T>,
		{
			type Output = Self;

			fn $fn_name(self, other: Self) -> Self {
				Complex {
					re: self.re $op other.re,
					im: self.im $op other.im,
				}
			}
		}
	};
}

// Macro for implementing AddAssign and SubAssign.
// $trt is which of the two traits to implement, $fn_name is
// the function name required for the implementation.
// $op is the operator used in the impl.
macro_rules! addsub_assign_impl {
	($trt:ident, $fn_name:ident, $op:tt) => {
		impl<T> $trt for Complex<T>
		where
			T: $trt,
		{
			fn $fn_name(&mut self, other: Self) {
				self.re $op other.re;
				self.im $op other.im;
			}
		}
	};
}

addsub_impl!(Add, add, +);
addsub_impl!(Sub, sub, -);

addsub_assign_impl!(AddAssign, add_assign, +=);
addsub_assign_impl!(SubAssign, sub_assign, -=);

impl<T> Mul for Complex<T>
where
	T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Complex {
			re: self.re * other.re - self.im * other.im,
			im: self.re * other.im + self.im * other.re,
		}
	}
}

impl<T> MulAssign for Complex<T>
where
	T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
	fn mul_assign(&mut self, other: Self) {
		*self = Complex {
			re: self.re * other.re - self.im * other.im,
			im: self.re * other.im + self.im * other.re,
		};
	}
}

impl<T> Div for Complex<T>
where
	T: Div<Output = T> + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
	type Output = Self;

	fn div(self, other: Self) -> Self {
		let a = self.re;
		let b = self.im;
		let c = other.re;
		let d = other.im;

		let divisor = c * c + d * d;

		let re = (a * c + b * d) / divisor;
		let im = (b * c - a * d) / divisor;

		Complex { re, im }
	}
}

impl<T> DivAssign for Complex<T>
where
	T: Div<Output = T> + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
	fn div_assign(&mut self, other: Self) {
		let a = self.re;
		let b = self.im;
		let c = other.re;
		let d = other.im;

		let divisor = c * c + d * d;

		self.re = (a * c + b * d) / divisor;
		self.im = (b * c - a * d) / divisor;
	}
}

impl<T> Mul<T> for Complex<T>
where
	T: Mul<Output = T> + Copy,
{
	type Output = Complex<T>;

	fn mul(self, other: T) -> Complex<T> {
		Complex {
			re: self.re * other,
			im: self.im * other,
		}
	}
}

impl<T: Default> Complex<T> {
	pub fn identity() -> Complex<T> {
		Complex::default()
	}
}

impl<T> fmt::Display for Complex<T>
where
	T: fmt::Display + PartialOrd + Default,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// May be a bad way to check if the number has a sign or not. There currently is no trait in the standard
		// library for checking signs of numbers. A possible solution would be to create a trait and implement it for
		// all number types that have the required sign checking method.
		let sign = if self.im >= Default::default() {
			"+"
		} else {
			""
		};

		write!(f, "{}{}{}i", self.re, sign, self.im)
	}
}
