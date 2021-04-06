use crate::localize::Localizor;
use crate::util::centered_rect;
use crate::event::{
    Events,
    EventHandler
};

use crossterm::event::{
    KeyCode,
    KeyEvent
};

use std::{
    rc::Rc,
    io::Write,
    error::Error,
    cell::RefCell
};

use tui::{
    Frame,
    text::Text,
    backend::CrosstermBackend,
    style::{
        Color,
        Style
    },
    layout::{
        Rect,
        Layout,
        Alignment,
        Direction,
        Constraint,
    },
    widgets::{
        Wrap,
        Block,
        Borders,
        Paragraph,
    }
};


pub struct MainMenu {
    pub selected_btn: usize,
    pub buttons: Vec<&'static str>,
    pub locale: Rc<RefCell<Localizor>>,
    pub events: Rc<RefCell<EventHandler>>
}

impl MainMenu {
    pub fn draw<W>(&self, frame: &mut Frame<CrosstermBackend<W>>) where W: Write {
        let locale = self.locale.borrow();
        let buttons = self.buttons.iter();

        let rect = centered_rect(30, 30, frame.size());
        let rect = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                vec![Constraint::Percentage(100/buttons.len() as u16)]
                    .iter()
                    .cycle()
                    .take(buttons.len())
                    .map(|c| *c)
                    .collect::<Vec<Constraint>>()
            )
            .split(rect[1]);

        let btn_blocks: Vec<(Paragraph, Rect)> = buttons.enumerate()
            .map(|(i, btn)| {
                let label = locale.get(btn, None).unwrap();
                let style = if i == self.selected_btn {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                } else {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                };

                let paragraph = Paragraph::new(Text::from(label))
                    .style(style)
                    .wrap(Wrap { trim: false })
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                let r = rect[i];

                (paragraph, r)
            })
            .collect();

        for block in btn_blocks {
            frame.render_widget(block.0, block.1);
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Result<(), Box<dyn Error>> {
        let mut events = self.events.borrow_mut();

        match key.code {
            KeyCode::Down => {
                self.selected_btn += 1;
                self.selected_btn %= self.buttons.len();
            },
            KeyCode::Up => {
                if self.selected_btn == 0 {
                    self.selected_btn = self.buttons.len()-1;
                } else {
                    self.selected_btn -= 1;
                }
            },
            KeyCode::Left  | 
            KeyCode::Right | 
            KeyCode::Enter | 
            KeyCode::Char(' ') => {
                match self.selected_btn {
                    0 => events.send(Events::SkipToScreen("saves".to_string())),
                    1 => events.send(Events::SkipToScreen("settings".to_string())),
                    2 => events.send(Events::SkipToScreen("about".to_string())),
                    _ => events.send(Events::Quit(0))
                }?;
            },
            _ => {}
        }

        Ok(())
    }

    pub fn once(&mut self) {
        self.buttons = vec!["menu-btn-play", "menu-btn-settings", "menu-btn-about", "menu-btn-quit"];
    }
}
