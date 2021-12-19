use std::ops::{Deref, DerefMut};

use crate::{box_constraints::BoxConstraints, Size};

pub trait Widget<T> {
    fn layout(&mut self, bc: &BoxConstraints) -> Size;
    fn paint(&self, buf: &mut [&mut [char]]);
}

impl<T> Widget<T> for Box<dyn Widget<T>> {
    fn layout(&mut self, bounds: &BoxConstraints) -> Size {
        self.deref_mut().layout(bounds)
    }

    fn paint(&self, buf: &mut [&mut [char]]) {
        self.deref().paint(buf)
    }
}
