use std::sync::Arc;

mod flex;
mod text;
pub use flex::*;
pub use text::*;

pub struct Size {
    a: usize,
    b: usize,
}

struct WidgetPod<T, W> {
    data: T,
    inner: W,
}

pub trait Widget<T> {
    fn size(&self, bounds: &Size) -> Size;
    fn paint(&self, buf: &mut [&mut [char]]);
}

pub trait Data {}

impl<T> Data for Arc<T> {}
