mod flex;
mod menu;
pub use flex::*;
pub use menu::*;

/// All objects will try to take up as much space
/// as possible.

#[derive(Debug, Clone, Copy)]
pub struct Size {
	width: usize,
	height: usize,
}

impl Size {
	pub const ZERO: Size = Size::new(0, 0);

	#[inline]
	pub const fn new(width: usize, height: usize) -> Self {
		Size { width, height }
	}

	pub fn max_side(self) -> usize {
		self.width.max(self.height)
	}

	pub fn min_side(self) -> usize {
		self.width.min(self.height)
	}

	#[inline]
	pub fn area(self) -> usize {
		self.width * self.height
	}

	#[inline]
	pub fn is_empty(self) -> bool {
		self.area() == 0
	}

	pub fn clamp(self, min: Size, max: Size) -> Self {
		let width = self.width.max(min.width).min(max.width);
		let height = self.height.max(min.height).min(max.height);
		Size { width, height }
	}
}

impl core::ops::Mul<Size> for Size {
	type Output = Self;
	fn mul(self, s: Size) -> Self {
		Self {
			width: s.width * self.width,
			height: s.height * self.height,
		}
	}
}

trait RoundingDiv<Rhs = Self> {
    type Output;
    fn rdiv(self, rhs: Rhs) -> <Self as RoundingDiv<Rhs>>::Output;
}

impl RoundingDiv<usize> for usize {
    type Output = Self;
    fn rdiv(self, rhs: usize) -> Self {
        self / rhs + Self::from((self % rhs) >= (rhs / 2))
    }
}

pub trait Element<'a> {
	fn layout(&self, bc: &BoxConstraints) -> Size;
	fn paint(&self, out: &'a [&'a [u8]]);
}

pub struct WidgetPod<T> {
	inner: T,
}

impl<'a, T: Element<'a>> Element<'a> for WidgetPod<T> {
	fn layout(&self, constraints: Size) -> Size {
		self.inner.layout(bc)
	}
	fn paint(&self, out: &'a [&'a [u8]]) {
		self.inner.paint(out);
	}
}
