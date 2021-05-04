use crate::localize::Localizor;
use crate::util::{
    offset_rect,
    centered_rect
};
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


pub struct Saves {
    pub locale: Rc<RefCell<Localizor>>,
    pub events: Rc<RefCell<EventHandler>>
}

impl Saves {
    pub fn draw<W>(&self, frame: &mut Frame<CrosstermBackend<W>>) where W: Write {
        let locale = self.locale.borrow();

        let base_rect = centered_rect(99, 99, frame.size())[1];
        let footer = offset_rect((99, 10), (1, 90), base_rect)[1];
        let header = offset_rect((99, -10), (1, -100), base_rect)[1];

        frame.render_widget(Block::default()
            .title("aaaaa")
            .borders(Borders::ALL), 
            base_rect
        );
        frame.render_widget(Block::default()
            .title("bbbbbbb")
            .borders(Borders::ALL), 
            footer
        );
        frame.render_widget(Block::default()
            .title("ccccc")
            .borders(Borders::ALL), 
            header
        );
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn once(&mut self) {
        
    }
}
