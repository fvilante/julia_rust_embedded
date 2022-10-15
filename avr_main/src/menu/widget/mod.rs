pub mod widget;
pub mod edit_mode;
pub mod caption;
pub mod field;
mod cursor;
pub mod menu_item;
pub mod submenu;

use crate::board::keyboard::KeyCode;
use crate::alloc::borrow::ToOwned;

use self::{field::{Field}, cursor::Cursor};

use super::{
    canvas::Canvas, 
    flash::FlashString, 
    point::Point, 
    ratangular_wave::RectangularWave
};

use core::ops::Range;
use avr_progmem::progmem;
use heapless::{
    Vec,
    String,
};

//

pub use widget::Widget;
use widget::Editable;
use edit_mode::EditMode;
pub use caption::Caption;
pub use menu_item::MenuItem;
pub use submenu::SubMenu;

