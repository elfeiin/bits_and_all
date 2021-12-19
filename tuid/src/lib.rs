use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result as CTRes,
};
use std::sync::Arc;
use std::{
    io::{stdout, Stdout, Write},
    ops::DerefMut,
};

mod box_constraints;
mod point;
mod rect;
mod size;
mod theme;
mod vec2;
mod widget;
use box_constraints::*;
use point::*;
use rect::*;
use size::*;
use vec2::*;
use widget::*;

pub trait Data {}

impl<T> Data for Arc<T> {}

struct Window<T: Data> {
    out: Stdout,
    root_widget: Box<dyn Widget<T>>,
}

impl<'a, T: Data> Window<T> {
    pub fn new<W>(root: W) -> Self
    where
        W: Widget<T> + 'static,
    {
        Self {
            out: stdout(),
            root_widget: Box::new(root),
        }
    }
    pub fn draw(&mut self) -> CTRes<()> {
        queue![self.out, Clear(ClearType::All)]?;
        let terminal_size = size();
        self.root_widget.deref_mut().layout(&terminal_size?.into());
        self.out.flush();
        Ok(())
    }
}
