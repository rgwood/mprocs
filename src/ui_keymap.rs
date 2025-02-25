use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::{
  backend::CrosstermBackend,
  layout::{Margin, Rect},
  style::{Color, Style},
  text::{Span, Spans, Text},
  widgets::{Block, BorderType, Borders, Paragraph},
  Frame,
};

use crate::{
  encode_term::print_key,
  state::{Scope, State},
  theme::Theme,
};

type Backend = CrosstermBackend<io::Stdout>;

pub fn render_keymap(
  area: Rect,
  frame: &mut Frame<Backend>,
  state: &mut State,
) {
  let theme = Theme::default();

  let block = Block::default()
    .title(Span::styled("Help", theme.pane_title(false)))
    .borders(Borders::ALL)
    .border_style(theme.pane_border(false))
    .border_type(BorderType::Rounded)
    .style(Style::default().bg(Color::Black));
  frame.render_widget(block, area);

  let items = match state.scope {
    Scope::Procs => vec![
      (KeyCode::Char('a'), KeyModifiers::CONTROL, "Toggle focus"),
      (KeyCode::Char('q'), KeyModifiers::NONE, "Quit"),
      (KeyCode::Char('j'), KeyModifiers::NONE, "Next"),
      (KeyCode::Char('k'), KeyModifiers::NONE, "Prev"),
      (KeyCode::Char('s'), KeyModifiers::NONE, "Start"),
      (KeyCode::Char('x'), KeyModifiers::NONE, "Stop"),
    ],
    Scope::Term => {
      vec![(KeyCode::Char('a'), KeyModifiers::CONTROL, "Toggle focus")]
    }
  };
  let line = items
    .into_iter()
    .map(|(code, mods, desc)| (KeyEvent::new(code, mods), desc))
    .map(|(key, desc)| {
      vec![
        Span::raw(" <"),
        Span::styled(print_key(key), Style::default().fg(Color::Yellow)),
        Span::raw(": "),
        Span::raw(desc),
        Span::raw("> "),
      ]
    })
    .flatten()
    .collect::<Vec<_>>();

  let line = Spans::from(line);
  let line = Text::from(vec![line]);

  let p = Paragraph::new(line);
  frame.render_widget(
    p,
    area.inner(&Margin {
      vertical: 1,
      horizontal: 1,
    }),
  );
}
