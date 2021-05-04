pub mod saves;
pub mod welcome;
pub mod main_menu;

pub use saves::Saves;
pub use welcome::Welcome;
pub use main_menu::MainMenu;

use std::{
    rc::Rc,
    cell::RefCell
};


#[derive(Default)]
pub struct Screens {
    pub saves: Option<Rc<RefCell<Saves>>>,
    pub welcome: Option<Rc<RefCell<Welcome>>>,
    pub main_menu: Option<Rc<RefCell<MainMenu>>>,
}
