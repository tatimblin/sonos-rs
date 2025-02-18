use std::io;
use std::sync::Arc;
use crossterm::event::KeyEvent;
use ratatui::{
  layout::{ Alignment, Constraint, Direction, Layout },
  text::Text,
  widgets::Paragraph,
  Frame,
};
use rand;
use rand::Rng;

use crate::state::store::Store;
use crate::widgets::{ logo::logo, util };

use super::View;

pub struct StartupView {
  store: Arc<Store>,
}

impl StartupView {
  pub fn new(store: Arc<Store>) -> Self {
    Self { store }
  }

  fn get_status_message(&self) -> String {
    self.store.with_state(|state| {
      let speaker_count = state.speakers.len();
      let mut rng = rand::thread_rng();
      match state.speakers.last() {
        Some(speaker) => format!(
          "({}/{}) {}",
          speaker_count,
          rng.gen_range(1..=100),
          speaker.get_info().get_name()
        ),
        None => format!(
          "{}",
          "Searching...",
        )
      }
    })
  }
}

impl View for StartupView {
  fn render(&mut self, frame: &mut Frame) {
    let logo = logo();
    let body = Text::from(self.get_status_message());

    let inner_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Length(5),
        Constraint::Length(1),
      ]);
    let (logo_area, text_area) = util::vertically_centered_layout(frame.area(), inner_layout);

    let logo_paragraph = Paragraph::new(logo).alignment(Alignment::Center);
    frame.render_widget(logo_paragraph, logo_area);

    let body_paragraph = Paragraph::new(body).alignment(Alignment::Center);
    frame.render_widget(body_paragraph, text_area);
  }

  fn handle_input(&mut self, _key_event: KeyEvent, _store: &Store) -> io::Result<()> {
    Ok(())
  }
}
