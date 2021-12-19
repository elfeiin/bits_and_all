use std::marker::PhantomData;

use crate::{Data, Point, Size, Widget, box_constraints::BoxConstraints};

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
    fn layout(&mut self, bounds: &BoxConstraints) -> Size {
        self.inner.layout(bounds)
    }

    fn paint(&self, buf: &mut [&mut [char]]) {
        self.inner.paint(buf)
    }
}
