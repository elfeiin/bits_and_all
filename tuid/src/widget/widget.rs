use std::ops::{Deref, DerefMut};

use crate::{box_constraints::BoxConstraints, DataWrapper, Size};

pub trait Widget<T> {
	fn update(&mut self, data: &DataWrapper<T>);
	fn layout(&mut self, bc: &BoxConstraints) -> Size;
	fn paint(&self, buf: &mut [char], size: &Size);
	fn event(&mut self, data: &mut DataWrapper<T>);
}

impl<T> Widget<T> for Box<dyn Widget<T>> {
	fn update(&mut self, data: &DataWrapper<T>) {
		self.deref_mut().update(data)
	}
	fn layout(&mut self, bounds: &BoxConstraints) -> Size {
		self.deref_mut().layout(bounds)
	}
	fn paint(&self, buf: &mut [char], size: &Size) {
		self.deref().paint(buf, size)
	}
	fn event(&mut self, data: &mut DataWrapper<T>) {
		self.deref_mut().event(data)
	}
}
