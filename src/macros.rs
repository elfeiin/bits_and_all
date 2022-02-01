#[macro_export]
macro_rules! app_start_boilerplate {
   ($stdout:expr) => {{
      enable_raw_mode()?;
      execute![$stdout, EnterAlternateScreen, EnableMouseCapture]?;
      Terminal::new(CrosstermBackend::new($stdout))?
   }};
}

#[macro_export]
macro_rules! app_end_boilerplate {
   ($terminal:expr) => {{
      disable_raw_mode()?;
      execute!(
         $terminal.backend_mut(),
         LeaveAlternateScreen,
         DisableMouseCapture
      )?;
      $terminal.show_cursor()?;
   }};
}
