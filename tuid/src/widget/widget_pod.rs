use std::marker::PhantomData;

use crate::{box_constraints::BoxConstraints, Data, DataWrapper, Point, Size, Widget};

pub struct WidgetPod<T, W> {
	data: PhantomData<T>,
	inner: W,
	origin: Point,
}

impl<T, W: Widget<T>> WidgetPod<T, W> {
	pub fn new(inner: W) -> Self {
		Self {
			data: PhantomData,
			inner,
			origin: Point { x: 0, y: 0 },
		}
	}

	pub fn set_origin(&mut self, p: Point) {
		self.origin = p;
	}
}

impl<T: Data, W: Widget<T>> Widget<T> for WidgetPod<T, W> {
	fn update(&mut self, data: &DataWrapper<T>) {
		self.inner.update(data)
	}

	fn layout(&mut self, bounds: &BoxConstraints) -> Size {
		self.inner.layout(bounds)
	}

	fn paint(&self, buf: &mut [char], size: &Size) {
		self.inner.paint(buf, size)
	}
	fn event(&mut self, data: &mut DataWrapper<T>) {
		self.inner.event(data)
	}
}
