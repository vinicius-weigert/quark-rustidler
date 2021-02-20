use log::*;

use crossterm::event::{
    self,
    Event
};

use std::{
    thread,
    error::Error,
    time::Duration,
    sync::mpsc::{
        channel,
        Sender,
        Receiver,
        RecvTimeoutError
    }
};


pub enum Events {
    SkipToScreen(String),
    CrosstermEvent(Event),
    CrosstermThreadError
}

pub struct EventHandler {
    pub tx: Sender<Events>,
    pub rx: Receiver<Events>
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let (tx, rx) = channel::<Events>();

        EventHandler { tx, rx }
    }

    pub fn send(&mut self, event: Events) -> Result<(), Box<dyn Error>> {
        self.tx.send(event)?;
        Ok(())
    }

    pub fn recv_timeout(&mut self, timeout: Duration) -> Option<Events> {
        match self.rx.recv_timeout(timeout) {
            Ok(event) => Some(event),
            Err(e) => {
                if e == RecvTimeoutError::Disconnected {
                    error!("Disconnected error in rx recv {:#?}", e);
                }
                None
            }
        }
    }

    pub fn run_crossterm(&self) {
        let tx = self.tx.clone();

        thread::spawn(move || {
            loop {
                let e = match event::read() {
                    Ok(event) => event,
                    Err(err) => {
                        warn!("Error while receiving crossterm event, may be recoverable {:#?}", err);
                        //Couldn't read for the crossterm events, so alert the main thread about it!
                        match tx.send(Events::CrosstermThreadError) {
                            Ok(_) => {},
                            Err(error) => error!("Couldn't send crossterm error to the main thread, this is a problem D: {:#?}", error)
                        };
                        continue;
                    }
                };

                match tx.send(Events::CrosstermEvent(e)) {
                    Ok(_) => {},
                    Err(err) => {
                        error!("Couldn't send crossterm event to the main thread. {:#?}", err);
                        break;
                    }
                }
            }
        });
    }
}
