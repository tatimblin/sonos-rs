mod state;
mod hooks;
mod views;
mod types;
mod widgets;

use std::io;
use std::sync::Arc;
use crossterm::event::{ self, KeyCode, KeyEvent };
use ratatui::DefaultTerminal;

use crate::state::store::Store;
use crate::hooks::use_speakers::use_speakers;
use crate::views::{
  View,
  startup::StartupView,
  control::ControlView,
};

pub struct App {
  store: Arc<Store>,
  exit: bool,
  current_view: Box<dyn View>,
}

impl App {
  pub fn new() -> Self {
    let store = Arc::new(Store::new());
    Self {
      store: store.clone(),
      current_view: Box::new(StartupView::new(store.clone())),
      exit: false,
    }
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    use_speakers(&self.store, || {
      terminal
        .draw(|frame| self.current_view.render(frame))
        .map(|_| ())
    })?;

    self.current_view = Box::new(ControlView::new(&self.store));

    while !self.exit {
      terminal.draw(|frame| self.current_view.render(frame))?;

      if let event::Event::Key(key_event) = event::read()? {
        self.handle_input(key_event)?;
      }
    }
    Ok(())
  }

  fn handle_input(&mut self, key_event: KeyEvent) -> io::Result<()> {
    match key_event.code {
      KeyCode::Char('q') => {
        self.exit = true;
        return Ok(());
      }
      _ => {}
    }

    self.current_view.handle_input(key_event, &self.store)
  }
}

fn main() -> io::Result<()> {
  let mut terminal = ratatui::init();
  let app_result = App::new().run(&mut terminal);
  ratatui::restore();
  app_result
}
