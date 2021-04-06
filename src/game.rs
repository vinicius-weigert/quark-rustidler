use log::*;

use std::{
    io,
    rc::Rc,
    path::Path,
    error::Error,
    cell::RefCell,
    time::Duration
};

use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode
    },
    event::{
        Event,
        KeyCode,
        KeyModifiers
    }
};

use tui::{
    Terminal,
    backend::CrosstermBackend,
};

use config::Config;

use crate::event::{
    Events,
    EventHandler
};
use crate::screens::{
    Screens,
    Welcome,
    MainMenu
};
use crate::logging;
use crate::config::init_cfg;
use crate::localize::Localizor;
use crate::resources::ResourceHandler;


pub struct Game {
    pub screens: Screens,
    pub settings: Config,
    pub current_screen: String,
    pub locale: Rc<RefCell<Localizor>>,
    pub event_handler: Rc<RefCell<EventHandler>>,
    pub resource_handler: Rc<RefCell<ResourceHandler>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            screens: Screens::default(),
            current_screen: String::from("uninitialized"),
            locale: Rc::new(RefCell::new(Localizor::new())),
            settings: init_cfg().expect("Couldn't init config"),
            event_handler: Rc::new(RefCell::new(EventHandler::new())),
            resource_handler: Rc::new(RefCell::new(ResourceHandler::new()))
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>>{
        logging::init(&self.settings)?;

        info!("Loading locales");
        self.load_locales()?;
        info!("Loading resources");
        self.load_resources()?;
        info!("Loading screens");
        self.load_screens();

        Ok(())
    }

    fn load_resources(&mut self) -> Result<(), Box<dyn Error>> {
        self.resource_handler.borrow_mut().load(Path::new("./res/ascii"))
    }

    fn load_locales(&mut self) -> Result<(), Box<dyn Error>> {
        self.locale.borrow_mut().load(Path::new("./locale"))
    }

    fn load_screens(&mut self) {
        let main_menu = MainMenu {
            selected_btn: 0,
            buttons: Vec::new(),
            locale: Rc::clone(&self.locale),
            events: Rc::clone(&self.event_handler),
        };
        let main_menu = Rc::new(RefCell::new(main_menu));

        info!("Created MainMenu screen");

        let welcome = Welcome {
            logo: String::from(""),
            locale: Rc::clone(&self.locale),
            events: Rc::clone(&self.event_handler),
            resources: Rc::clone(&self.resource_handler)
        };
        let welcome: Rc<RefCell<Welcome>> = Rc::new(RefCell::new(welcome));

        info!("Created Welcome screen");

        self.screens = Screens {
            welcome: Some(welcome),
            main_menu: Some(main_menu)
        };
    }

    pub fn run_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout();

        info!("Enabling raw mode");
        enable_raw_mode()?;

        //Start TUI's terminal with crosstermbackend
        info!("Creating terminal");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;


        //BORROWED SCREENS
        info!("Borrowing screens");
        let main_menu = self.screens.main_menu.as_ref().unwrap();
        let mut main_menu = main_menu.borrow_mut();

        let welcome = self.screens.welcome.as_ref().unwrap();
        let mut welcome = welcome.borrow_mut();


        //Run crossterm events
        {
            //Fucking borrow it onscope, so it don't fuck up other borrowinginwings
            let mut events = self.event_handler.borrow_mut();
            //Yes, we need to run it here
            info!("Running crossterm loop");
            events.run_crossterm();
            events.send(Events::SkipToScreen("welcome".to_string()))?;
        }

        //Set locale to the selected on the config, or just default to english
        {
            let mut locale = self.locale.borrow_mut();

            locale.select_lang(self.settings.get("main.locale").unwrap())?;
        }

        terminal.clear()?;

        info!("Entering the main loop");
        'main: loop {
            //DRAWING STUFF
            //Draw what you have to draw on the terminal
            match self.current_screen.as_str() {
                "welcome" => terminal.draw(|f| welcome.draw(f))?,
                "main_menu" => terminal.draw(|f| main_menu.draw(f))?,
                _ => {}
            }

            
            //EVENT HANDLING
            let event = {
                let mut events = self.event_handler.borrow_mut();
                let event = events.recv_timeout(Duration::from_millis(10));
    
                //If there's no event to handle, just skip the event section
                if event.is_none() {
                    continue 'main;
                }

                //So we move stupid event and drop events, with no problem on borrowingwinging it multiple times
                event.unwrap()
            };

            match event {
                Events::CrosstermEvent(Event::Key(key)) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('c') |
                            KeyCode::Char('q') |
                            KeyCode::Char('z') |
                            KeyCode::Char('x') => break 'main,
                            _ => {}
                        }
                    }

                    //Propagate the events to the proper active screen
                    match self.current_screen.as_str() {
                        "welcome" => welcome.on_key(key)?,
                        "main_menu" => main_menu.on_key(key)?,
                        _ => {}
                    }
                },
                Events::SkipToScreen(s) => {
                    info!("Changing from screen {} to {}!", self.current_screen, s);
                    self.current_screen = s;

                    match self.current_screen.as_str() {
                        "welcome" => welcome.once(),
                        "main_menu" => main_menu.once(),
                        _ => {}
                    };
                },
                Events::Quit(e) => {
                    info!("Exit code: {}", e);
                    break 'main;
                },
                _ => {}
            }
        }

        info!("Finishing the game");
        terminal.clear()?;
        terminal.set_cursor(0, 0)?;
        info!("Disabling raw mode");
        disable_raw_mode()?;

        info!("Bye bye :D");
        Ok(())
    }
}
