use super::*;

// Which way the children go :P
pub enum Direction {
	Row,
	RowReverse,
	Col,
	ColReverse,
}

// Allows the items to wrap
pub enum Wrap {
	NoWrap,
	Wrap,
	WrapReverse,
}

// How the children are sread out
// along the main axis
pub enum Justify {
	Start,
	End,
	Center,
	Between,
	Around,
	Evenly,
}

// Defines alignment along the cross axis
pub enum Alignment {
	Start,
	End,
	Center,
	Stretch,
	Baseline,
}

pub struct FlexPod<T> {
    flex: usize,
    alignment: Alignment,
    inner: WidgetPod<T>,
}

struct Flex<T> {
    direction: Direction,
    wrap: Wrap,
    justify: Justify,
	children: Vec<FlexPod<T>>,
}

impl<'a, T> Element<'a> for Flex<T> {
	fn layout(&self, constraints: Size) -> Size {
		Size::ZERO
	}
	fn paint(&self, out: &'a [&'a [u8]]) {}
}
