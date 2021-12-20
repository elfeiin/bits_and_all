use std::ops::Deref;
use std::sync::{Arc, Mutex};

use tuid::Data;
use tuid::*;

#[derive(Debug, Default)]
struct ExampleAppState {
	pub value: i32,
}

impl Data for ExampleAppState {}

fn make_ui() -> impl Widget<Arc<Mutex<ExampleAppState>>> {
	let t1 = Text::new(Box::new(
		|data: &DataWrapper<Arc<Mutex<ExampleAppState>>>| {
			let s = format!["Hello, {}!", data.deref().lock().unwrap().value];
			s
		},
	));

	// let flex = Flex::new();

	// flex

	t1
}

fn main() {
	let main_state = Arc::new(Mutex::new(ExampleAppState::default()));
	let mut window = Window::new(main_state, make_ui());
	window.run().unwrap();
}
