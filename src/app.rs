use std::{
   fs::File,
   hint,
   io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom, Write},
   path::PathBuf,
};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use tui::{
   backend::Backend,
   widgets::{Block, Borders, Table},
   Terminal,
};

use crate::{readwriteseek::ReadWriteSeek, Res};

enum View {
   Base,
   Menu { selection: Vec<usize> },
   Window,
}

enum Buffer {
   Memory {
      buf: Vec<u8>,
   },
   File {
      file: File,
      pos: usize,
      buf: Vec<u8>,
   },
}

impl Default for Buffer {
   fn default() -> Self {
      Self::Memory { buf: vec![] }
   }
}

struct Window {
   buf: Box<dyn ReadWriteSeek>,
   columns: usize,
   show_text: bool,
   show_hex: bool,
   window_pos: usize,
   cursor_pos: usize,
}

impl Default for Window {
   fn default() -> Self {
      Self {
         columns: 32,
         ..Default::default()
      }
   }
}

impl TryFrom<PathBuf> for Window {
   type Error = io::Error;
   fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
      let mut file = File::open(p)?;
      file.seek(SeekFrom::Start(0))?;
      let window = Window {
         buf: Box::new(file),
         ..Default::default()
      };
      Ok(window)
   }
}

pub struct App<'a, B: Backend> {
   terminal: &'a mut Terminal<B>,
   view: View,
   windows: Vec<Window>,
   current_window: Option<usize>,
}

impl<'a, B: Backend> App<'a, B> {
   pub fn new(terminal: &'a mut Terminal<B>) -> Self {
      Self {
         terminal,
         view: View::Base,
         windows: vec![],
         current_window: None,
      }
   }

   fn handle_input(&mut self, ev: Event) {
      match self.view {
         View::Base => {
            if self.current_window.is_none() && !self.windows.is_empty() {
               self.current_window = Some(0);
            }
            match ev {
               Event::Key(k) => {}
               Event::Mouse(_) => {}
               Event::Resize(_, _) => {}
            }
         }
         View::Menu { ref selection } => todo!(),
         View::Window => todo!(),
      }
   }

   fn draw(&mut self) -> Res {
      match self.view {
         View::Base => todo!(),
         View::Menu { ref selection } => todo!(),
         View::Window => {
            if let Some(cw) = self.current_window {
               if let Some(window) = self.windows.get(cw) {
                  self.terminal.draw(|f| {
                     let size = f.size();
                     let bg = Block::default().title("Block").borders(Borders::NONE);
                     f.render_widget(bg, size);
                     // let rows =
                     // let hex_pane = Table::new(rows);
                  })?;
               }
            }
         }
      }
      Ok(())
   }

   pub fn run(mut self) -> Res {
      while let Ok(ev) = read() {
         if let Event::Key(k) = ev {
            if k.code == KeyCode::Char('q') && k.modifiers.contains(KeyModifiers::CONTROL) {
               return Ok(());
            }
         }
         self.handle_input(ev);
         self.draw()?;
      }
      Ok(())
   }
}
