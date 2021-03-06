use crate::util::centered_rect;
use crate::localize::Localizor;
use crate::resources::ResourceHandler;
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
    text::Text,
    layout::Alignment,
    backend::CrosstermBackend,
    widgets::{
        Wrap,
        Block,
        Paragraph
    }
};

pub struct Welcome {
    pub logo: String,
    pub locale: Rc<RefCell<Localizor>>,
    pub events: Rc<RefCell<EventHandler>>,
    pub resources: Rc<RefCell<ResourceHandler>>,
}

impl Welcome {
    pub fn draw<W>(&self, frame: &mut Frame<CrosstermBackend<W>>) where W: Write {
        let locale = self.locale.borrow();

        let logo_text = Text::from(self.logo.to_owned());
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
        let resources = self.resources.borrow();

        self.logo = resources.random("logos").unwrap().ascii.to_owned();
    }
}
