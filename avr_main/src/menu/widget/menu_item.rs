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
    optional::OptionsBuffer,
    sub_menu_render::{LcdLine, SubMenuRender},
    unsigned16_widget::{Content, Field, Format, StringBuffer},
    widget::Editable,
    widget::Widget,
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

pub struct NumericalParameterArgs<'a> {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    pub parameters: Format,
    pub child: Option<SubMenuHandle>,
    pub variable: &'a Cell<u16>,
}

pub struct OptionalParameterArgs<'a> {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    //pub variable: &'static mut Cursor,
    pub options_list: OptionsBuffer,
    pub child: Option<SubMenuHandle>,
    pub variable: &'a Cell<Cursor>,
}

pub enum MenuItemArgs<'a> {
    Numerical(NumericalParameterArgs<'a>),
    Optional(OptionalParameterArgs<'a>),
}

// -----------------------------------

pub struct MenuItemWidget<'a> {
    point_a: Point1d,
    caption: Caption,
    point_b: Point1d,
    field: Field<'a>,
    pub child: Option<SubMenuHandle>,
}

impl<'a> MenuItemWidget<'a> {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(
        point_a: Point1d,
        text: FlashString,
        point_b: Point1d,
        field: Field<'a>,
        child: Option<SubMenuHandle>,
    ) -> Self {
        Self {
            point_a,
            caption: Caption::new(text),
            point_b,
            field,
            child,
        }
    }

    pub fn from_numerical(args: NumericalParameterArgs<'a>) -> MenuItemWidget<'a> {
        let point1 = Point1d::new(args.point1_);
        let point2 = Point1d::new(args.point2_);
        let field = Field::from_numerical(args.variable, (args.parameters).clone());
        let mut menu_item = Self::new(point1, args.text, point2, field, args.child);
        menu_item
    }

    pub fn from_optional(args: OptionalParameterArgs<'a>) -> MenuItemWidget<'a> {
        let mut options_list_cloned = Vec::new();
        options_list_cloned.clone_from(&args.options_list);
        let point1 = Point1d::new(args.point1_);
        let point2 = Point1d::new(args.point2_);
        let initial_selection = Cursor::new(0, 2, 0); //(*args.variable).clone();
        let field = Field::from_optional(initial_selection, options_list_cloned);
        let mut menu_item = Self::new(point1, args.text, point2, field, args.child);
        menu_item
    }

    pub fn from_menu_args(args: MenuItemArgs<'a>) -> Self {
        match args {
            MenuItemArgs::Numerical(args) => Self::from_numerical(args),
            MenuItemArgs::Optional(args) => Self::from_optional(args),
        }
    }
}

impl MenuItemWidget<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    todo!(); //abort_edition();
                }

                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    todo!(); //save_edition();
                }

                //delegate everything else
                _ => self.field.send_key(key),
            };
        }
    }

    pub fn update(&mut self) {
        self.caption.update();
        self.field.update();
    }

    pub fn draw(&self, canvas: &mut Canvas, lcd_line: LcdLine) {
        let line = lcd_line as u8;
        let point1: Point<u8> = Point::new(self.point_a.pos, line);
        let point2: Point<u8> = Point::new(self.point_b.pos, line);
        self.caption.draw(canvas, point1);
        self.field.draw(canvas, point2);
    }
}

impl MenuItemWidget<'_> {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
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
