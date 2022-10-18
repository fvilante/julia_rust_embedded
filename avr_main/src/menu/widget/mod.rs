pub mod widget;
pub mod edit_mode;
pub mod caption;
pub mod field;
mod cursor;
pub mod menu_item;
pub mod submenu;
pub mod splash;

use crate::board::keyboard::KeyCode;
use crate::alloc::borrow::ToOwned;

use self::{field::{Field}, cursor::Cursor};

use super::{
    canvas::Canvas, 
    flash::FlashString, 
    point::Point, 
    ratangular_wave::RectangularWave
};

