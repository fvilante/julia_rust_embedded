use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{canvas::Canvas, flash::FlashString, point::{Point, Point1d}, accessor::{Accessor, ArenaId, Arena}},
};

use super::{caption::Caption, field::{Field, FieldBuffer, FieldEnum}, widget::Editable, widget::Widget, sub_menu::{LcdLine, SubMenu}, cursor::Cursor, optional::OptionsBuffer};

use avr_progmem::string::PmString;
use heapless::{String,Vec};
use lib_1::utils::common::convert_u16_to_string_decimal;
use core::{str::FromStr, ops::Range};

// -----------------------------------

pub struct NumericalParameterArgs {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    pub arena_id: ArenaId<u16>,
    pub initial_cursor_position: u8,
    pub number_of_digits: u8,
    pub valid_range: Range<u16>,
}

pub struct OptionalParameterArgs {
    pub point1_: u8,
    pub point2_: u8,
    pub text: FlashString,
    pub accessor_handler: ArenaId<Cursor>,
    pub options_list: OptionsBuffer,
}

pub enum MenuItemArgs {
    Numerical(NumericalParameterArgs),
    Optional(OptionalParameterArgs)
}

// -----------------------------------

pub struct MenuItem<'a> { 
    point_a: Point1d, 
    caption: Caption, 
    point_b: Point1d, 
    field: Field<'a>,   
    sub_menu: Option<&'a mut SubMenu<'a>>,
}


impl<'a> MenuItem<'a> {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(point_a: Point1d, text: FlashString, point_b: Point1d, field: Field<'a>, sub_menu: Option<&'a mut SubMenu<'a>>) -> Self {
        Self {
            point_a,
            caption: Caption::new(text),
            point_b,
            field,
            sub_menu,
        }
    }

    pub fn from_numerical<'b: 'a,const SIZE: usize>(
        controler: &'b mut Arena<u16, SIZE>, 
        args: NumericalParameterArgs,
    ) -> MenuItem<'a> {
        match args {
            NumericalParameterArgs {
                point1_,
                point2_,
                text,
                arena_id: accessor_handler,
                initial_cursor_position,
                number_of_digits,
                valid_range,
            } => {
                let point1 = Point1d::new(point1_);
                let point2 = Point1d::new(point2_);
                let accessor = Accessor::from_accessor_controler(controler, accessor_handler);
                let field = Field::from_numerical(
                    accessor,
                    initial_cursor_position as usize,
                    number_of_digits as usize,
                    valid_range,
                );
                let mut menu_item = Self::new(point1, text, point2, field, None);
                menu_item
            }
        }
    }

    pub fn from_optional<'b: 'a,const SIZE: usize>(
        controler: &'b mut Arena<Cursor, SIZE>, 
        args: OptionalParameterArgs,
    ) -> MenuItem<'a> {
        match args {
            OptionalParameterArgs {
                point1_,
                point2_,
                text,
                accessor_handler,
                options_list,
            } => {
                let point1 = Point1d::new(point1_);
                let point2 = Point1d::new(point2_);
                let accessor = Accessor::from_accessor_controler(controler, accessor_handler);
                let field = Field::from_optional(options_list, accessor);
                let mut menu_item = Self::new(point1, text, point2, field, None);
                menu_item
            }
        }
    }

    //pub fn from_menu_args<T,const SIZE: usize>( controler: &mut Accessor2Controler<T, SIZE>, args: MenuItemArgs) -> Self {
    //    match args {
    //        MenuItemArgs::Numerical(args) => Self::from_numerical(controler, args),
    //        MenuItemArgs::Optional(args) => Self::from_optional(controler, args),
    //    }
    //}

}

impl MenuItem<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        self.field.send_key(key);
    }

    pub fn update(&mut self) {
        self.caption.update();
        self.field.update();
    }

    pub fn draw(&self, canvas: &mut Canvas, lcd_line: LcdLine) {
        let line = lcd_line.as_u8();
        let point1: Point<u8> = Point::new(self.point_a.pos, line);
        let point2: Point<u8> = Point::new(self.point_b.pos, line);
        self.caption.draw(canvas, point1);
        self.field.draw(canvas, point2);
    }
}

impl MenuItem<'_> {
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

pub enum MenuItemParsed {
    PureCaption(String<40>), // [Caption]
    CaptionWithOneField(String<40>, FieldBuffer, String<10>), // [1st_caption, field_type, last_caption]
}

pub fn parse_menu_item_constructor_string(declaration: FlashString) -> MenuItemParsed {
    // example of declaration content = "Posicao inicial     ${nnnnn} mm/s"
    let s: String<40>  = declaration.to_string().unwrap_or(String::from_str("Error: Small container").unwrap());
    let begin_token: &[_] = &['$', '{'];
    let end_token: &[_] = &['}'];
    match s.find(begin_token) {
        Some(begin_index) =>  {
            //1st caption ends in begin_index
            let x = s.split_at(begin_index+begin_token.len());
            let first_caption_ = x.0;
            let first_caption = &first_caption_[0..first_caption_.len()-begin_token.len()];
            let remain = x.1;
            match remain.find(end_token) {
                Some(end_index) => {
                    let y = remain.split_at(end_index);
                    let field_type = y.0;
                    let last_caption_ = y.1;
                    let last_caption = &last_caption_[end_token.len()..last_caption_.len()];
                    MenuItemParsed::CaptionWithOneField(
                        String::from_str(first_caption).unwrap(), 
                        String::from_str(field_type).unwrap(), 
                        String::from_str(last_caption).unwrap(),
                    )
                }
                None => {
                    //false open, everything is caption
                    let caption = s.as_str();
                    MenuItemParsed::PureCaption(String::from_str(caption).unwrap())
                }
            }
        }

        None => {
            //caption entire string
            let caption = s.as_str();
            MenuItemParsed::PureCaption(String::from_str(caption).unwrap())
        }
    }
}

