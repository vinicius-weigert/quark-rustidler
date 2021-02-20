use log::*;

use crate::util::centered_rect;
use crate::localize::Localizor;
use crate::event::{
    Events,
    EventHandler
};

use crossterm::event::KeyEvent;

use std::{
    rc::Rc,
    io::Write,
    error::Error,
    cell::RefCell
};

use tui::{
    Frame,
    backend::CrosstermBackend,
    text::Text,
    widgets::{
        Wrap,
        Block,
        Borders,
        Paragraph
    },
    style::{
        Color,
        Style,
        Modifier
    },
    layout::{
        Layout,
        Alignment,
        Direction,
        Constraint,
    }
};

//Change it to ResourceHandler and then add more logos
const LOGO: &str = include_str!("../../res/ascii/logos/logo.txt");

pub struct Welcome {
    pub locale: Rc<RefCell<Localizor>>,
    pub events: Rc<RefCell<EventHandler>>,
}

impl Welcome {
    pub fn draw<W>(&self, frame: &mut Frame<CrosstermBackend<W>>) where W: Write {
        let locale = self.locale.borrow();

        let logo_text = Text::from(LOGO);
        let pktp_text = Text::from(locale.get("pktp", None).unwrap());
        let rect = centered_rect(90, 30, frame.size());

        let logo = Paragraph::new(logo_text)
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });
        let pktp = Paragraph::new(pktp_text)
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        frame.render_widget(logo, rect[1]);
        frame.render_widget(pktp, rect[2]);
    }

    pub fn on_key(&mut self, _key: KeyEvent) -> Result<(), Box<dyn Error>> {
        let mut events = self.events.borrow_mut();

        events.send(Events::SkipToScreen("main_menu".to_string()))
    }

    pub fn once(&mut self) {
        
    }
}
