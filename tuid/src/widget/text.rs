use crate::{box_constraints::BoxConstraints, Data, Size, Widget};

pub struct Text<T: Data> {
    data: T,
    text: Box<dyn Fn(&T) -> String>,
}

impl<T: Data> Text<T> {
    pub fn new(data: T, text: Box<dyn Fn(&T) -> String>) -> Self {
        Self { data, text }
    }
    fn text(&self) -> String {
        (self.text)(&self.data)
    }
}

impl<T: Data> Widget<T> for Text<T> {
    fn layout(&mut self, _bounds: &BoxConstraints) -> Size {
        Size {
            width: self.text().chars().count(),
            height: self.text().chars().filter(|ch| *ch == '\n').count(),
        }
    }
    fn paint(&self, buf: &mut [&mut [char]]) {
        let the_text = self.text();
        let mut the_chars = the_text.chars();
        for line in buf.iter_mut() {
            for spot in line.iter_mut() {
                if let Some(ch) = the_chars.next() {
                    if ch == '\n' {
                        break;
                    } else {
                        *spot = ch;
                    }
                }
            }
        }
    }
}
