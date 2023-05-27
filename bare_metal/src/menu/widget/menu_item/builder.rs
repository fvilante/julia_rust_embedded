use crate::geometry::point::Point1d;
use crate::menu::widget::submenu_programa::spec::MenuProgramaHandle;
use crate::string::flash::FlashString;

use super::{
    super::{field::field::Field, field::numerical::Format, field::optional::OptionsBuffer},
    menu_item::MenuItemWidget,
};
use avr_progmem::string::PmString;
use core::{cell::Cell, ops::Range};
use cross_platform::utils::cursor::Cursor;
use heapless::Vec;

pub struct SimpleMenu {
    pub parent_name: FlashString,
    pub child_menu: MenuProgramaHandle,
}

/// TODO: Improve this construction (ie: why (col, text) instead of an iterator of Captions and/or Fields ?)
/// TODO: Abstract string, use IntoIterator<Item = u8>.
/// TODO: Verify if is there there a way to avoid the Option<T> in the `unit_of_measurement` field
pub struct NumericalParameter<'a> {
    pub parameter_name: FlashString,
    pub variable: (u8, &'a Cell<u16>), // (collunm_position, text)
    pub valid_range: Range<u16>,
    pub unit_of_measurement_text: Option<(u8, FlashString)>, // (collunm_position, text)
}

pub struct OptionalParameter<'a> {
    pub parameter_name: FlashString,
    pub variable: (u8, &'a Cell<Cursor>), // (collunm_position, text)
    pub options_list: OptionsBuffer,
}

/// TODO: Make construction methods `const fn` this may improve flash code's size
pub struct MenuItemBuilder;

impl MenuItemBuilder {
    /// collum to start to print menu_item text in the lcd
    const POINT1: Point1d = Point1d::new(1);

    /// Wraps the menu into an Some value for convenience, because it will be used as the return
    /// of a next method of an Iterator
    fn wrap_value_for_convenience(menu_item: MenuItemWidget) -> Option<MenuItemWidget> {
        Some(menu_item)
    }

    pub fn make_simple_menu<'a>(ctor: SimpleMenu) -> Option<MenuItemWidget<'a>> {
        // prepare
        let point1 = Self::POINT1;
        let text = ctor.parent_name;
        let child = Some(ctor.child_menu);
        // build
        let menu_item = MenuItemWidget::new((point1, text), None, child, None);
        Self::wrap_value_for_convenience(menu_item)
    }

    pub fn make_numerical_parameter(ctor: NumericalParameter) -> Option<MenuItemWidget> {
        // prepare
        let point1 = Self::POINT1;
        let text = ctor.parameter_name;
        let point2 = ctor.variable.0.into();
        let format = Format {
            start: ctor.valid_range.start,
            end: ctor.valid_range.end,
            initial_cursor_position: 0,
        };
        let field = Field::from_numerical(ctor.variable.1, format);
        let unit_of_measurement_label = ctor
            .unit_of_measurement_text
            .map(|x| (Point1d::new(x.0), x.1));
        let child = None.into();
        // build
        let menu_item = MenuItemWidget::new(
            (point1, text),
            Some((point2, field)),
            child,
            unit_of_measurement_label,
        );
        Self::wrap_value_for_convenience(menu_item)
    }

    pub fn make_optional_parameter(ctor: OptionalParameter) -> Option<MenuItemWidget> {
        // prepare
        let point1 = Self::POINT1;
        let text = ctor.parameter_name;
        let options_list__ = {
            let mut options_list_cloned = Vec::new();
            options_list_cloned.clone_from(&ctor.options_list);
            options_list_cloned
        };
        let field = Field::from_optional(ctor.variable.1, options_list__);
        let point2 = ctor.variable.0.into();
        let child = None;
        // build
        let menu_item = MenuItemWidget::new((point1, text), Some((point2, field)), child, None);
        Self::wrap_value_for_convenience(menu_item)
    }
}
