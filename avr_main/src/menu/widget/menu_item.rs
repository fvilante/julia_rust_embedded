use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{canvas::Canvas, flash::FlashString, point::{Point, Point1d}},
};

use super::{caption::Caption, field::{Field, FieldBuffer, Getter, Setter}, widget::Editable, widget::Widget};

use heapless::String;
use lib_1::utils::common::convert_u16_to_string_decimal;
use core::{str::FromStr, ops::Range};

pub struct MenuItem {
    point_a: Point1d,
    caption: Caption,
    point_b: Point1d,
    field: Field,
}


impl MenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(point_a: Point1d, text: FlashString, point_b: Point1d, getter: Getter, setter: Setter, initial_cursor_position: usize, number_of_digits: usize, valid_range: Range<u16>) -> Self {
        Self {
            point_a,
            caption: Caption::new(text),
            point_b,
            field: Field::new(setter, getter, initial_cursor_position, number_of_digits, valid_range),
        }
    }

}

impl MenuItem {
    pub fn send_key(&mut self, key: KeyCode) {
        self.field.send_key(key);
    }

    pub fn update(&mut self) {
        self.caption.update();
        self.field.update();
    }

    // lcd_line: false = line_0 ; true = line_1
    pub fn draw(&self, canvas: &mut Canvas, lcd_line: bool) {
        let mut line = 0;
        if lcd_line {
            line = 1;
        }
        let point1 = Point::new(self.point_a.pos, line);
        let point2 = Point::new(self.point_b.pos, line);
        self.caption.draw(canvas, point1);
        self.field.draw(canvas, point2);
    }
}

impl MenuItem {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}


//


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

