use log::*;

use crate::localize::Localizor;
use crate::event::EventHandler;

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
    backend::CrosstermBackend,
    text::{
        Span,
        Spans,
    },
    style::{
        Color,
        Style
    },
    layout::{
        Layout,
        Direction,
        Constraint
    },
    widgets::{
        Tabs,
        Block,
        Borders,
        BorderType
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(8), Constraint::Percentage(92)].as_ref())
            .split(frame.size());

        let tabs = Tabs::new(self.buttons.iter().cloned().map(|s| {
            Spans::from(vec![
                Span::from(self.locale.borrow().get(s, None).unwrap())
            ])
        }).collect())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::TOP).border_type(BorderType::Rounded))
            .select(self.selected_btn)
            .highlight_style(Style::default().fg(Color::Gray).bg(Color::White))
            .divider("|");

        let border = Borders::LEFT | Borders::RIGHT | Borders::BOTTOM;

        let page = match self.selected_btn {
            0 => Block::default().title("Bibibii").borders(border),
            1 => Block::default().title("Babababab").borders(border),
            2 => Block::default().title("Bebebebe").borders(border),
            3 => Block::default().title("AhahshahshaAAAAAAAA").borders(border),
            4 => Block::default().title("Aehasuehuash").borders(border),
            _ => Block::default().title("d:").borders(border),
        };

        frame.render_widget(tabs, chunks[0]);
        frame.render_widget(page, chunks[1]);
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Result<(), Box<dyn Error>> {
        match key.code {
            KeyCode::Char('r') => {
                let mut locale = self.locale.borrow_mut();

                if locale.selected_lang == "pt-BR" {
                    locale.select_lang("en-US".to_string())?;
                } else {
                    locale.select_lang("pt-BR".to_string())?;
                }
            },
            KeyCode::Right => {
                self.selected_btn += 1;
                self.selected_btn %= self.buttons.len();
            },
            KeyCode::Left => {
                if self.selected_btn == 0 {
                    self.selected_btn = self.buttons.len()-1;
                } else {
                    self.selected_btn -= 1;
                }
            },
            _ => {}
        };

        Ok(())
    }

    pub fn once(&mut self) {
        
    }
}
