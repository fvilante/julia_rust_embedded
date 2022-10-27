use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{canvas::Canvas, flash::FlashString, point::Point},
};

use super::{caption::Caption, field::{Field, FieldBuffer}, widget::Editable, widget::Widget};

use heapless::String;
use core::str::FromStr;

pub struct MenuItem {
    caption: Caption,
    field: Field,
}

impl MenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    pub fn new(point1: Point, text: FlashString, point2: Point, array: FieldBuffer) -> Self {
        Self {
            caption: Caption::new(point1, text),
            field: Field::new(point2, array),
        }
    }

    //pub fn from_string_constructor(declaration: FlashString) -> Self {
    //    match parse_menu_item_constructor_string(declaration) {
    //        MenuItemParsed::PureCaption(value) => {
    //            
    //        } 
    //        MenuItemParsed::CaptionWithOneField(c_first, c_field, c_first ) => {
    //            let x = 1; // initial x
    //            let point1 = Point(x,0);
    //            let point2 = point1.x as usize + c_field.len();
    //            Self::new(point1, text, point2, array)
    //        }
    //    }
    //}

    pub fn set_caption(&mut self, text: FlashString) {
        self.caption.set_caption(text);
    }

    pub fn get_value_if_it_has_changed(&mut self) -> Option<FieldBuffer> {
        self.field.get_value_if_it_has_changed()
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

    pub fn draw(&self, canvas: &mut Canvas) {
        self.caption.draw(canvas);
        self.field.draw(canvas);
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

