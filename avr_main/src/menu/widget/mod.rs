pub mod caption;
pub mod cursor;
pub mod execucao;
pub mod field;
pub mod main_menu;
pub mod manual_mode;
pub mod menu_item;
pub mod splash;
pub mod sub_menu_render;
pub mod widget;
pub mod widget_tests;

use crate::alloc::borrow::ToOwned;
use crate::board::keyboard::KeyCode;

use self::{cursor::Cursor, field::unsigned16_widget::Field};

use super::{canvas::Canvas, flash::FlashString, point::Point, ratangular_wave::RectangularWave};
