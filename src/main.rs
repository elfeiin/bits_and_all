use crossterm::{
   event::{DisableMouseCapture, EnableMouseCapture},
   execute,
   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod macros;
mod readwriteseek;

use app::*;

type Res = Result<(), io::Error>;

/// bitsandall
/// Controls:
///     Ctrl + Q to quit.
///     Press Esc to bring up the Menu.
/// Menu:
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
fn main() -> Res {
   let mut stdout = io::stdout();
   let mut terminal = app_start_boilerplate![stdout];

   {
      let app = App::new(&mut terminal);
      app.run()?;
   }

   app_end_boilerplate![terminal];

   Ok(())
}
