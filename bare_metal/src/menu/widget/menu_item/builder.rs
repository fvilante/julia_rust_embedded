use crate::geometry::point::Point1d;
use crate::menu::widget::submenu_programa::spec::MenuProgramaHandle;
use crate::string::flash::FlashString;

use super::{
    super::{field::field::Field, field::numerical::Format, field::optional::OptionsBuffer},
    menu_item::MenuItemWidget,
};
use avr_progmem::string::PmString;
use core::{cell::Cell, ops::Range};
use heapless::Vec;
use lib_1::utils::cursor::Cursor;

/// Base struct for menu_item builder. Contains usual building options (ie: `Title`, `Child`, etc)
struct Base {
    point1: u8,
    point2: Option<u8>, // only used if has numerical or optional field
    text: FlashString,
    child: Option<MenuProgramaHandle>,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            point1: Default::default(),
            point2: None,
            text: Default::default(),
            child: None,
        }
    }
}

/// only numerical
struct Numerical<'a> {
    format: Format,
    variable_u16: &'a Cell<u16>,
    /// define the position and text for display unit of measurement on screen
    unit_of_measurement: Option<(u8, FlashString)>,
}

/// only optional
struct Optional<'a> {
    variable_option: &'a Cell<Cursor>,
    options_list: OptionsBuffer,
}

/// TODO: This entire construction of the menusystem would be refactored when possible
/// This is a [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) to construct MenuItems.
pub struct MenuItemBuilder<'a> {
    base: Base,
    numerical: Option<Numerical<'a>>,
    optional: Option<Optional<'a>>,
}

impl<'a> MenuItemBuilder<'a> {
    //

    pub fn from_text<const N: usize>(val: &PmString<N>) -> Self {
        Self {
            base: Base {
                point1: 1,
                point2: None,
                text: FlashString::new(val),
                child: None,
            },
            numerical: None,
            optional: None,
        }
    }

    pub fn add_conection_to_submenu(&mut self, handle: MenuProgramaHandle) -> &mut Self {
        self.base.child = Some(handle);
        self
    }

    pub fn add_numerical_variable(
        &mut self,
        variable: &'a Cell<u16>,
        valid_range: Option<Range<u16>>,
        point2: u8,
        unit_of_measurement: Option<(u8, FlashString)>,
    ) -> &mut Self {
        let valid_range = if let Some(valid_range) = valid_range {
            valid_range
        } else {
            let full_range = 0..0xFFFF;
            full_range
        };
        self.base.point2 = Some(point2);
        self.numerical = Some(Numerical {
            format: Format {
                start: valid_range.start,
                end: valid_range.end,
                initial_cursor_position: 0,
            },
            variable_u16: variable,
            unit_of_measurement,
        });
        self
    }

    pub fn add_optional_variable(
        &mut self,
        variable: &'a Cell<Cursor>,
        options_list: OptionsBuffer,
        point2: u8,
    ) -> &mut Self {
        self.base.point2 = Some(point2);
        self.optional = Some(Optional {
            variable_option: variable,
            options_list,
        });
        self
    }

    pub fn build(&mut self) -> MenuItemWidget<'a> {
        const DEFAULT_POSITION_FOR_POINT_2: u8 = 30; // TODO: this value should be improved, to be more reasoned and less arbitrary, or eventually a panic with proper error message should be preferable

        // FIX: If client construct numerical and optional at same time the numerical will be taken and the
        // optional will be ignored. It's safe, but it's better to refactor the code so client cannot
        // compile this ambiguity.
        if let Some(numerical) = &mut self.numerical {
            let point1 = Point1d::new(self.base.point1);
            let point2 = Point1d::new(self.base.point2.unwrap_or(DEFAULT_POSITION_FOR_POINT_2));
            let field = Field::from_numerical(numerical.variable_u16, (numerical.format).clone());
            let (point3, unit_of_measurement) = numerical.unit_of_measurement.unwrap_or_default();
            let menu_item = MenuItemWidget::new(
                (point1, self.base.text),
                Some((point2, field)),
                self.base.child,
                Some((Point1d::new(point3), unit_of_measurement)),
            );
            menu_item
        } else if let Some(optional) = &mut self.optional {
            let mut options_list_cloned = Vec::new();
            options_list_cloned.clone_from(&optional.options_list);
            let point1 = Point1d::new(self.base.point1);
            let point2 = Point1d::new(self.base.point2.unwrap_or(DEFAULT_POSITION_FOR_POINT_2));
            const NUMBER_OF_OPTIONS_AVAILABLE: u8 = 2; // TODO: This is a simplification and not will be always the case in future. Make this avaliation more dynamic and automatic
            let _initial_selection = Cursor::new(0, NUMBER_OF_OPTIONS_AVAILABLE, 0); //(*args.variable).clone();
            let field = Field::from_optional(optional.variable_option, options_list_cloned);
            let menu_item = MenuItemWidget::new(
                (point1, self.base.text),
                Some((point2, field)),
                self.base.child,
                None,
            );
            menu_item
        } else {
            // it is submenu caller
            let point1 = Point1d::new(self.base.point1);
            let text = self.base.text;
            let child = self.base.child;
            let menu_item = MenuItemWidget::new((point1, text), None, child, None);
            menu_item
        }
    }
}
