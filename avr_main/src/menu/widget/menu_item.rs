use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{
        accessor::Accessor,
        canvas::Canvas,
        flash::FlashString,
        point::{Point, Point1d},
        sub_menu_handle::SubMenuHandle,
    },
};

use super::{
    caption::Caption,
    field::optional::{make_options_buffer_from_array, OptionsBuffer},
    field::unsigned16_widget::{Content, Field, Format},
    sub_menu_render::{LcdLine, SubMenuRender},
    widget::Editable,
    widget::{Saveble, Widget},
};
use avr_progmem::string::PmString;
use core::{cell::Cell, ops::Range, str::FromStr};
use heapless::{String, Vec};
use lib_1::utils::{common::usize_to_u8_clamper, cursor::Cursor};
use lib_1::{
    arena::arena::{Arena, ArenaId},
    utils::common::convert_u16_to_string_decimal,
};

// -----------------------------------

/// Submenu Title and others...
struct Base {
    point1: u8,
    point2: Option<u8>, // only used if has numerical or optional field
    text: FlashString,
    child: Option<SubMenuHandle>,
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
}

/// only optional
struct Optional<'a> {
    variable_option: &'a Cell<Cursor>,
    options_list: OptionsBuffer,
}

/// This is a [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) to construct MenuItems.
pub struct MenuItemBuilder<'a> {
    base: Base,
    numerical: Option<Numerical<'a>>,
    optional: Option<Optional<'a>>,
}

impl<'a> MenuItemBuilder<'a> {
    //

    pub fn new_text<const N: usize>(val: &PmString<N>) -> Self {
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

    pub fn add_numerical_variable(
        &mut self,
        variable: &'a Cell<u16>,
        valid_range: Option<Range<u16>>,
        point2: u8,
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

    pub fn add_conection_to_submenu(&mut self, handle: SubMenuHandle) -> &mut Self {
        self.base.child = Some(handle);
        self
    }

    pub fn build(&mut self) -> MenuItemWidget<'a> {
        const default_position_for_point_2: u8 = 30; // TODO: this value should be improved, to be more reasoned and less arbitrary, or eventually a panic with proper error message should be preferable
        ///FIX: If client construct numerical and optional at same time the numerical will be taken and the
        /// optional will be ignored. It's safe, but it's better to refactor the code so client cannot
        /// compile this ambiguity.
        if let Some(numerical) = &mut self.numerical {
            let point1 = Point1d::new(self.base.point1);
            let point2 = Point1d::new(self.base.point2.unwrap_or(default_position_for_point_2));
            let field = Field::from_numerical(numerical.variable_u16, (numerical.format).clone());
            let mut menu_item = MenuItemWidget::new(
                (point1, self.base.text),
                Some((point2, field)),
                self.base.child,
            );
            menu_item
        } else if let Some(optional) = &mut self.optional {
            let mut options_list_cloned = Vec::new();
            options_list_cloned.clone_from(&optional.options_list);
            let point1 = Point1d::new(self.base.point1);
            let point2 = Point1d::new(self.base.point2.unwrap_or(default_position_for_point_2));
            const number_of_options_available: u8 = 2; // TODO: This is a simplification and not will be always the case in future. Make this avaliation more dynamic and automatic
            let initial_selection = Cursor::new(0, number_of_options_available, 0); //(*args.variable).clone();
            let field = Field::from_optional(optional.variable_option, options_list_cloned);
            let mut menu_item = MenuItemWidget::new(
                (point1, self.base.text),
                Some((point2, field)),
                self.base.child,
            );
            menu_item
        } else {
            // it is submenu caller
            let point1 = Point1d::new(self.base.point1);
            let text = self.base.text;
            let child = self.base.child;
            let mut menu_item = MenuItemWidget::new((point1, text), None, child);
            menu_item
        }
    }
}

//

pub struct MenuItemWidget<'a> {
    point_and_caption: (Point1d, Caption),
    point_and_field: Option<(Point1d, Field<'a>)>,
    child: Option<SubMenuHandle>,
}

impl<'a> MenuItemWidget<'a> {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(
        point_and_text: (Point1d, FlashString),
        point_and_field: Option<(Point1d, Field<'a>)>,
        child: Option<SubMenuHandle>,
    ) -> Self {
        let (point_a, text) = point_and_text;
        Self {
            point_and_caption: (point_a, Caption::new(text)),
            point_and_field,
            child,
        }
    }

    pub fn get_child(&self) -> Option<SubMenuHandle> {
        self.child
    }
}

impl Saveble for MenuItemWidget<'_> {
    fn restore_value(&mut self) {
        self.set_edit_mode(false); // terminate the edition
        let Some((_, field)) = &mut self.point_and_field else { return();};
        field.restore_value();
    }

    fn save_value(&mut self) {
        self.set_edit_mode(false); // terminate the edition
        let Some((_, field)) = &mut self.point_and_field else { return(); };
        field.save_value();
    }
}

impl MenuItemWidget<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.restore_value();
                }

                // saves edition
                KeyCode::KEY_ENTER => {
                    self.save_value();
                }

                //delegate everything else
                _ => {
                    if let Some((_, field)) = &mut self.point_and_field {
                        field.send_key(key);
                    };
                }
            };
        }
    }

    pub fn update(&mut self) {
        let (_, caption) = &mut self.point_and_caption;
        caption.update();
        if let Some((_, field)) = &mut self.point_and_field {
            field.update();
        };
    }

    pub fn draw(&self, canvas: &mut Canvas, lcd_line: LcdLine) {
        let line = lcd_line as u8;
        let (point1, caption) = &self.point_and_caption;
        let point1: Point<u8> = Point::new(point1.pos, line);
        caption.draw(canvas, point1);
        if let Some((point2, field)) = &self.point_and_field {
            let point2: Point<u8> = Point::new(point2.pos, line);
            field.draw(canvas, point2);
        };
    }
}

impl MenuItemWidget<'_> {
    pub fn set_edit_mode(&mut self, value: bool) {
        if let Some((_, field)) = &mut self.point_and_field {
            field.set_edit_mode(value);
        };
    }

    pub fn is_in_edit_mode(&self) -> bool {
        if let Some((_, field)) = &self.point_and_field {
            field.is_in_edit_mode()
        } else {
            false
        }
    }
}

/// Creates a parser for a menu_item template string
///
/// It parses the template string (for example: "Posicao inicial     ${nnnnn} mm/s") returning an interator
/// decomposing the parsed string
pub fn make_template_iterator(flash_string: FlashString) -> FlashTemplateIterator {
    FlashTemplateIterator {
        reminder: Some(flash_string),
        is_inside_token: false,
    }
}

pub enum TemplateKind {
    /// Pure caption
    Caption(FlashString),
    /// Pure field
    Field(FlashString),
    /// Represent not well formed template string.
    ///
    /// For example when you open a token but do not closes it before the end of the template string
    /// (ie: "Foo bar ${xxxxx  ").
    IllFormed(FlashString),
}

/// Flash template string parser
pub struct FlashTemplateIterator {
    /// contatins the string that still must to be parsed, at the end of iteration its value is None
    reminder: Option<FlashString>,
    is_inside_token: bool,
}

const BEGIN_TOKEN: &[char] = &['$', '{'];
const END_TOKEN: &[char] = &['}'];

// TODO: Improve readability of below code
impl Iterator for FlashTemplateIterator {
    type Item = TemplateKind;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(reminder) = self.reminder else {
            return None;
        };

        if self.is_inside_token {
            // If it is inside token we are waiting for an end token
            let Some(end_index) = reminder.find_index(&END_TOKEN) else {
                // Ill formed (open_token without end_token).
                self.is_inside_token = false;
                self.reminder = None;
                return Some(TemplateKind::IllFormed(reminder));
            };
            // Well formed token. (end_token located)
            self.is_inside_token = false;
            let field: FlashString = reminder.sub_string(0..end_index + 1);
            let new_reminder = reminder.sub_string(end_index + 1..reminder.len());
            self.reminder = if new_reminder.len() == 0 {
                None
            } else {
                Some(new_reminder)
            };
            // removed BEGIN_TOKEN and END_TOKEN from the field
            const BEGIN_TOKEN_LENGTH: u8 = usize_to_u8_clamper(BEGIN_TOKEN.len());
            const END_TOKEN_LENGTH: u8 = usize_to_u8_clamper(END_TOKEN.len());
            let field = field.sub_string(BEGIN_TOKEN_LENGTH..field.len() - (END_TOKEN_LENGTH));
            return Some(TemplateKind::Field(field));
            // NOTE: We will ignore the second Start_Token in the case of an Start_Token -> Start_Token -> End_Token
            // TODO: Maybe in future we should create escape code for the Tokens chars
        } else {
            // If  not is_inside_token then we are looking for begin_token
            let Some(begin_index) = reminder.find_index(&BEGIN_TOKEN) else {
                // but begin token does not exist then
                // this is a pure text (without token)
                self.is_inside_token = false;
                self.reminder = None;
                return Some(TemplateKind::Caption(reminder));
            };
            // begin_token exists
            self.is_inside_token = true;
            let caption: FlashString = reminder.sub_string(0..begin_index);
            self.reminder = Some(reminder.sub_string(begin_index..reminder.len()));
            return Some(TemplateKind::Caption(caption));
        }
    }
}
