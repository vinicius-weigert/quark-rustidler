pub mod game;
pub mod util;
pub mod event;
pub mod config;
pub mod logging;
pub mod screens;
pub mod localize;
pub mod constants;

use game::Game;

use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();

    game.load()?;
    game.run_loop()?;

    Ok(())
}
