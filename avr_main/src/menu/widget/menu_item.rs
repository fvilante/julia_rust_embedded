use crate::{
    board::keyboard::KeyCode,
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

    pub fn set_caption(&mut self, text: FlashString) {
        self.caption.set_caption(text);
    }
}

impl Widget for MenuItem {
    fn send_key(&mut self, key: KeyCode) {
        self.field.send_key(key);
    }

    fn update(&mut self) {
        self.caption.update();
        self.field.update();
    }

    fn draw(&self, canvas: &mut Canvas) {
        self.caption.draw(canvas);
        self.field.draw(canvas);
    }
}

impl Editable for MenuItem {
    fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}


//



pub enum MenuItemParsed {
    PureCaption(String<40>), // [Caption]
    CaptionWithOneField(String<40>, String<10>, String<10>), // [1st_caption, field_type, last_caption]
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

// new implementation from a more complex constructor



trait Acessor<T> {
    fn set(value: T);
    fn get() -> T; 
}

pub struct ItemConstructor {
    string_constructor: FlashString
}

impl ItemConstructor {
     
}


pub struct MenuItem2 {

}

impl MenuItem2 {
    pub fn new(constructor_string: FlashString)  {

    }

    pub fn send_key(key: KeyCode) {

    }



}