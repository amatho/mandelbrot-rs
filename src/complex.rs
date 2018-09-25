use std::fmt;
use std::ops::{Add, AddAssign};

pub struct Complex<T> {
	pub re: T,
	pub im: T,
}

impl<T> Add for Complex<T>
where
	T: Add<Output = T>,
{
	type Output = Complex<T>;

	fn add(self, other: Complex<T>) -> Complex<T> {
		let re = self.re + other.re;
		let im = self.im + other.im;

		Complex { re, im }
	}
}

impl<T> AddAssign for Complex<T>
where
	T: AddAssign,
{
	fn add_assign(&mut self, other: Complex<T>) {
		self.re += other.re;
		self.im += other.im;
	}
}

impl<T: Default> Complex<T> {
	pub fn identity() -> Complex<T> {
		Complex {
			re: Default::default(),
			im: Default::default(),
		}
	}
}

impl<T: fmt::Display> fmt::Display for Complex<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}+{}i", self.re, self.im)
	}
}
