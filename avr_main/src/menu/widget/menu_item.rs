use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{
        accessor::Accessor,
        canvas::Canvas,
        flash::FlashString,
        point::{Point, Point1d},
    },
};

use super::{
    caption::Caption,
    optional::OptionsBuffer,
    sub_menu::{LcdLine, SubMenu},
    unsigned16_widget::{Content, Field, Format, StringBuffer},
    widget::Editable,
    widget::Widget,
};
use avr_progmem::string::PmString;
use core::{cell::Cell, ops::Range, str::FromStr};
use heapless::{String, Vec};
use lib_1::utils::cursor::Cursor;
use lib_1::{
    arena::arena::{Arena, ArenaId},
    utils::common::convert_u16_to_string_decimal,
};

// -----------------------------------

pub struct NumericalParameterArgs {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    pub variable: &'static mut u16,
    pub parameters: Format,
}

pub struct OptionalParameterArgs {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    pub variable: &'static mut Cursor,
    pub options_list: OptionsBuffer,
}

pub enum MenuItemArgs {
    Numerical(NumericalParameterArgs),
    Optional(OptionalParameterArgs),
}

// -----------------------------------

pub struct MenuItemWidget<'a> {
    point_a: Point1d,
    caption: Caption,
    point_b: Point1d,
    field: Field,
    sub_menu: Option<&'a mut SubMenu<'a>>,
}

impl<'a> MenuItemWidget<'a> {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(
        point_a: Point1d,
        text: FlashString,
        point_b: Point1d,
        field: Field,
        sub_menu: Option<&'a mut SubMenu<'a>>,
    ) -> Self {
        Self {
            point_a,
            caption: Caption::new(text),
            point_b,
            field,
            sub_menu,
        }
    }

    pub fn from_numerical(args: &mut NumericalParameterArgs) -> MenuItemWidget<'a> {
        let point1 = Point1d::new(args.point1_);
        let point2 = Point1d::new(args.point2_);
        let initial_value = (*args.variable).clone();
        let field = Field::from_numerical(initial_value, (args.parameters).clone());
        let mut menu_item = Self::new(point1, args.text, point2, field, None);
        menu_item
    }

    pub fn from_optional(args: &mut OptionalParameterArgs) -> MenuItemWidget<'a> {
        let mut options_list_cloned = Vec::new();
        options_list_cloned.clone_from(&args.options_list);
        let point1 = Point1d::new(args.point1_);
        let point2 = Point1d::new(args.point2_);
        let initial_selection = (*args.variable).clone();
        let field = Field::from_optional(initial_selection, options_list_cloned);
        let mut menu_item = Self::new(point1, args.text, point2, field, None);
        menu_item
    }

    pub fn from_menu_args(args: &mut MenuItemArgs) -> Self {
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

// =========================================================================
// BELLOW CODE: EXAMPLE OF CODE FOR INSTANTIATE MENUITEMS FROM TEMPLATE STRINGS
// example of template declaration content = "Posicao inicial     ${nnnnn} mm/s"
// CODE BELOW IS NOT ACTIVE YET, IT IS HERE TO SUGGEST FUTURE IMPLEMENTATION (REMOVE IT IF CONSIDERED NOT NECESSARY)
// NOTE: CODE BELOW WAS TESTED AND WORKS, BUT IS JUST A PROOF-OF-CONCEPT.

pub enum TemplateStringParsed {
    PureCaption(String<40>), // [Caption]
    ParameterWithOneFieldAndUnitOfMeasurement(String<40>, StringBuffer, String<10>), // [1st_caption, field_type, last_caption]
    ParameterWithOneField(String<40>, StringBuffer),
}

pub fn parse_menu_item_template_string(template_string: FlashString) -> TemplateStringParsed {
    // example of declaration content = "Posicao inicial     ${nnnnn} mm/s"
    let s: String<40> = template_string.to_string().unwrap();
    let begin_token: &[char] = &['$', '{'];
    let end_token: &[char] = &['}'];
    match s.find(begin_token) {
        Some(begin_index) => {
            //1st caption ends in begin_index
            let x = s.split_at(begin_index + begin_token.len());
            let first_caption_ = x.0;
            let first_caption = &first_caption_[0..first_caption_.len() - begin_token.len()];
            let remain = x.1;
            match remain.find(end_token) {
                Some(end_index) => {
                    let y = remain.split_at(end_index);
                    let field_type = y.0;
                    let last_caption_ = y.1;
                    let last_caption = &last_caption_[end_token.len()..last_caption_.len()];
                    TemplateStringParsed::ParameterWithOneFieldAndUnitOfMeasurement(
                        String::from_str(first_caption).unwrap(),
                        String::from_str(field_type).unwrap(),
                        String::from_str(last_caption).unwrap(),
                    )
                }
                None => {
                    //false open, everything is caption
                    let caption = s.as_str();
                    TemplateStringParsed::PureCaption(String::from_str(caption).unwrap())
                }
            }
        }

        None => {
            //caption entire string
            let caption = s.as_str();
            TemplateStringParsed::PureCaption(String::from_str(caption).unwrap())
        }
    }
}
