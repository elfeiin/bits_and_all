use crate::{box_constraints::BoxConstraints, Data, DataWrapper, Size, Widget};

pub struct Text<T: Data> {
	text: Box<dyn Fn(&DataWrapper<T>) -> String>,
	needs_repaint: bool,
	buf: String,
}

impl<T: Data> Text<T> {
	pub fn new(text: Box<dyn Fn(&DataWrapper<T>) -> String>) -> Self {
		Self {
			text,
			needs_repaint: true,
			buf: String::new(),
		}
	}
}

impl<T: Data> Widget<T> for Text<T> {
	fn update(&mut self, data: &DataWrapper<T>) {
		self.buf = (*self.text)(data);
	}
	fn layout(&mut self, bc: &BoxConstraints) -> Size {
		let mut width = 0;
		let mut height = 1;
		let mut x = 0;
		for ch in self.buf.chars() {
			if ch == '\n' {
				height += 1;
				x = 0;
			}
			x += 1;
			if x > bc.max().width - 1 {
				x = 0;
				width = bc.max().width;
			}
			if x > width {
				width = x;
			}
		}
		Size::new(width, height).clamp(bc.min(), bc.max())
	}
	fn paint(&self, buf: &mut [char], size: &Size) {
		let mut the_chars = self.buf.chars();
		for y in 0..size.height {
			for x in 0..size.width {
				if let (Some(ch), Some(spot)) = (the_chars.next(), buf.get_mut(x + y * size.width)) {
					if ch == '\n' {
						break;
					} else {
						*spot = ch;
					}
				}
			}
		}
	}
	fn event(&mut self, data: &mut DataWrapper<T>) {}
}
