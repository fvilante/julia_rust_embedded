pub mod widget;
pub mod edit_mode;
pub mod caption;
pub mod field;
pub mod cursor;
pub mod menu_item;
pub mod submenu;
pub mod splash;
pub mod main_menu;
pub mod execucao;
pub mod manual_mode;

use crate::board::keyboard::KeyCode;
use crate::alloc::borrow::ToOwned;

use self::{field::{Field}, cursor::Cursor};

use super::{
    canvas::Canvas, 
    flash::FlashString, 
    point::Point, 
    ratangular_wave::RectangularWave
};

