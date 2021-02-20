pub mod welcome;
pub mod main_menu;

pub use welcome::Welcome;
pub use main_menu::MainMenu;

use std::{
    rc::Rc,
    cell::RefCell
};


#[derive(Default)]
pub struct Screens {
    pub welcome: Option<Rc<RefCell<Welcome>>>,
    pub main_menu: Option<Rc<RefCell<MainMenu>>>
}
