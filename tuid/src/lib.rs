use crossterm::{
	cursor, queue,
	style::{self, Stylize},
	terminal::{size, Clear, ClearType},
	ExecutableCommand, QueueableCommand, Result as CTRes,
};
use std::{
	io::{stdout, Stdout, Write},
	ops::DerefMut,
};
use std::{ops::Deref, sync::Arc};

mod box_constraints;
mod point;
mod rect;
mod size;
mod theme;
mod vec2;
mod widget;
pub use box_constraints::*;
pub use point::*;
pub use rect::*;
pub use size::*;
pub use vec2::*;
pub use widget::*;

pub trait Data {}

impl<T> Data for Arc<T> {}

pub struct DataWrapper<T> {
	changed: bool,
	data: T,
}

impl<T: Data> DataWrapper<T> {
	pub fn new(data: T) -> Self {
		Self {
			changed: true,
			data,
		}
	}
}

impl<T> DataWrapper<T> {
	fn changed(&mut self) -> bool {
		if self.changed {
			self.changed = false;
			true
		} else {
			false
		}
	}
}

impl<T> Deref for DataWrapper<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T> DerefMut for DataWrapper<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.changed = true;
		&mut self.data
	}
}

pub struct Window<T: Data> {
	out: Stdout,
	buf: Vec<char>,
	data: DataWrapper<T>,
	root_widget: Box<dyn Widget<T>>,
}

impl<'a, T: Data> Window<T> {
	pub fn new<W>(data: T, root: W) -> Self
	where
		W: Widget<T> + 'static,
	{
		Self {
			out: stdout(),
			buf: vec![],
			data: DataWrapper::new(data),
			root_widget: Box::new(root),
		}
	}
	fn draw(&mut self) -> CTRes<()> {
		self.root_widget.event(&mut self.data);
		self.root_widget.update(&self.data);
		if self.data.changed() {
			queue![self.out, Clear(ClearType::All)]?;
			self.out.flush()?;
			let terminal_size = size()?;
			self.buf = vec![' '; terminal_size.0 as usize * terminal_size.1 as usize];
			self.root_widget.deref_mut().layout(&terminal_size.into());
			self
				.root_widget
				.deref_mut()
				.paint(&mut self.buf, &terminal_size.into());
			for ch in &self.buf {
				print!["{}", ch];
			}
			self.out.flush()?;
		}
		Ok(())
	}
	pub fn run(&mut self) -> CTRes<()> {
		loop {
			self.draw()?;
		}
		Ok(())
	}
}
