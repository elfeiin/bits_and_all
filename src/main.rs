use crossterm::{
	cursor, queue,
	style::{self, Stylize},
	terminal::{Clear, ClearType},
	ExecutableCommand, QueueableCommand, Result as CTRes,
};
use std::io::{stdout, Error, Read, Seek, SeekFrom, Stdout, Write};
use std::{fs::File, path::PathBuf};

mod widget;

/// bitsandall
/// Controls:
///     Press Esc to bring up the Context
///     Menu.
/// Context Menu:
///     File -> Various commands for inte
///         racting with the file system.
///         Also has options for selectin
///         g a currently open window.
///     Edit -> Commands for choosing how
///         to interact with the current
///         window (and the app in genera
///         l).
///     Help -> A menu with information
///         about the program itself, suc
///         h as how to use it.
///

enum State {
	Base,
	Menu { selection: Vec<usize> },
}

struct Window {
	file: Option<File>,
	buf: Vec<u8>,
	window_pos: usize,
	cursor_pos: usize,
}

impl Window {
	pub fn new() -> Self {
		Self {
			file: None,
			buf: vec![],
			window_pos: 0,
			cursor_pos: 0,
		}
	}
}

impl TryFrom<PathBuf> for Window {
	type Error = Error;
	fn try_from(p: PathBuf) -> Result<Self, Error> {
		let mut file = File::open(p)?;
		let mut buf = vec![];
		file.read_to_end(&mut buf)?;
		file.seek(SeekFrom::Start(0))?;
		let file = Some(file);
		Ok(Self {
			file,
			buf,
			window_pos: 0,
			cursor_pos: 0,
		})
	}
}

#[derive(Default)]
struct Settings {
	columns: usize,
}

struct AppState<'a> {
	out: Stdout,
	state: State,
	windows: Vec<Window>,
	current_window: Option<&'a Window>,
	settings: Settings,
}

impl<'a> AppState<'a> {
	pub fn new() -> Self {
		Self {
			out: stdout(),
			state: State::Base,
			windows: vec![],
			current_window: None,
			settings: Settings::default(),
		}
	}
	fn open() {}
	pub fn draw(&mut self) -> CTRes<()> {
		queue![self.out, Clear(ClearType::All)]?;
		match &self.state {
			State::Base => {}
			State::Menu { selection } => {}
		}
		self.out.flush();
		Ok(())
	}
}

fn main() {
	// let mut app_state = AppState::new();

	// use widget::*;
	// let f = Flex::new::row();

	loop {
		// app_state.draw();
	}
}
