use core::cell::Cell;

use crate::geometry::point::Point;
use crate::{
    board::keypad::KeyCode,
    menu::{
        canvas::Canvas,
        widget::widget::{Editable, Saveble, Widget},
    },
};

use lib_1::utils::cursor::Cursor;

use super::{
    numerical::{Format, NumberInputEditorWidget},
    optional::{OptionEditorWidget, OptionsBuffer},
};

/// Makes possible to edit a position of memory using Lcd display and keyboard
/// esc abort edition, and enter confirm edition
///
/// Abstracts all kind of fields existent offering an equal interface for all of them (Note: New Fields
/// may be added in the future)
pub enum Field<'a> {
    Numerical(NumberInputEditorWidget<'a>),
    Optional(OptionEditorWidget<'a>),
}

impl<'a> Field<'a> {
    pub fn from_numerical(variable: &'a Cell<u16>, format: Format) -> Self {
        const INITIAL_EDITING_MODE: bool = false; // does not start in edit mode
        let numerical_field = NumberInputEditorWidget::new(variable, format, INITIAL_EDITING_MODE);
        Self::Numerical(numerical_field)
    }

    pub fn from_optional(variable: &'a Cell<Cursor>, options: OptionsBuffer) -> Self {
        const INITIAL_EDITING_MODE: bool = false; // does not start in edit mode
        let optional = OptionEditorWidget::new(variable, options, INITIAL_EDITING_MODE);
        Self::Optional(optional)
    }
}

impl Saveble for Field<'_> {
    fn restore_value(&mut self) {
        match self {
            Self::Numerical(x) => x.restore_value(),
            Self::Optional(x) => x.restore_value(),
        }
    }

    fn save_value(&mut self) {
        match self {
            Self::Numerical(x) => x.save_value(),
            Self::Optional(x) => x.save_value(),
        }
    }
}

impl Widget for Field<'_> {
    fn send_key(&mut self, key: KeyCode) {
        match self {
            Self::Numerical(x) => x.send_key(key),
            Self::Optional(x) => x.send_key(key),
        }
    }

    fn update(&mut self) {
        match self {
            Self::Numerical(x) => x.update(),
            Self::Optional(x) => x.update(),
        }
    }

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        match self {
            Self::Numerical(x) => x.draw(canvas, start_point),
            Self::Optional(x) => x.draw(canvas, start_point),
        }
    }
}

impl Editable for Field<'_> {
    fn set_edit_mode(&mut self, value: bool) {
        match self {
            Self::Numerical(x) => x.set_edit_mode(value),
            Self::Optional(x) => x.set_edit_mode(value),
        }
    }

    fn is_in_edit_mode(&self) -> bool {
        match self {
            Self::Numerical(x) => x.is_in_edit_mode(),
            Self::Optional(x) => x.is_in_edit_mode(),
        }
    }
}
