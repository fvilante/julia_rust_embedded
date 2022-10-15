use crate::board::keyboard::KeyCode;
use crate::alloc::borrow::ToOwned;

use super::{
    canvas::Canvas, 
    flash::FlashString, 
    point::Point, 
    ratangular_wave::RectangularWave
};

use core::ops::Range;
use avr_progmem::progmem;
use heapless::{
    Vec,
    String,
};

progmem! {
    static progmem string NOP1 = "nop1";
    static progmem string NOP2 = "nop2";
    static progmem string NAO_IDENTIFICADO = "Nao identificado";
}

pub trait Widget {
    fn send_key(&mut self, key: KeyCode);
    fn update(&mut self);
    fn draw(&self, canvas: &mut Canvas);
}


trait Editable {
    fn set_edit_mode(&mut self, value: bool);
    fn is_in_edit_mode(&self) -> bool;
    fn toggle_edit_mode(&mut self) {
        if self.is_in_edit_mode() {
            self.set_edit_mode(false);
        } else {
            self.set_edit_mode(true);
        }
    }
}

struct EditMode {
    is_in_edit_mode: bool,
}

impl EditMode {
    fn new(is_in_edit_mode: bool) -> Self {
        Self {
            is_in_edit_mode,
        }
    }
}

impl Editable for EditMode {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode = value;
    }
    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode
    }
}

struct Caption {
    text: FlashString,
    start_point: Point,
}

impl Caption {
    fn new(start_point: Point, text: FlashString) -> Self {
        Self {
            text,
            start_point,
        }
    }

    fn set_caption(&mut self, text: FlashString) {
        self.text = text;
    }
}

impl Widget for Caption {
    fn send_key(&mut self, _key: KeyCode) { 
        // ignore key
    }

    fn update(&mut self) {
        // do nothing
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_cursor(self.start_point);
        for byte in self.text.chars() {
            canvas.print_char(byte as char);
        }
    }
}

//

struct Cursor {
    current: usize,
    range: Range<usize>,
}

impl Cursor {
    fn new(range: Range<usize>) -> Self {
        Self {
            current: 0,
            range,
        }
    }

    fn get_current(&self) -> usize {
        self.current
    }

    /// returns true if has reached the upper bound
    fn next(&mut self) -> bool {
        let last_index = self.range.end-1;
        let current_index = self.current;
        let has_reached_upper_bound = current_index >= last_index;
        if has_reached_upper_bound == false  {
            self.current += 1;
        }
        has_reached_upper_bound
    }

    /// returns true if has reached the lower bound
    fn previous(&mut self) -> bool {
        let first_index = self.range.start;
        let current_index = self.current;
        let has_reached_lower_bound = current_index <= first_index;
        if has_reached_lower_bound == false {
            self.current -= 1;
        }
        has_reached_lower_bound
    }

    //fn end(&mut self) {
    //    self.current = self.range.end;
    //}
    //
    //fn begin(&mut self) {
    //    self.current = self.range.start;
    //}
}


struct BufferedCursor {
    buffer: String<10>,
    cursor: Cursor,
}

impl BufferedCursor {
    pub fn new(buffer: String<10>) -> Self {
        Self {
            cursor: Cursor::new(0..buffer.len()),
            buffer,
        }
    }


    pub fn change_cursor_item_to(&mut self, new_char: char) -> &mut Self {
        let current_cursor = self.cursor.get_current();
        let mut s: String<10> = String::new();
        for (index, current_char) in self.buffer.char_indices() {
            if index == current_cursor {
                s.push(new_char).unwrap();
            } else {
                s.push(current_char).unwrap();
            }
        }
        self.buffer = s.to_owned();
        self  
    }

    /// increment_cursor_safe
    pub fn move_cursor_right(&mut self) -> &mut Self {
        self.cursor.next();
        self
    }

    /// decrement_cursor_safe
    pub fn move_cursor_left(&mut self) -> &mut Self {
        self.cursor.previous();
        self
    }

    //pub fn move_cursor_begin(&mut self) -> &mut Self {
    //    self.cursor.begin();
    //    self
    //}
    //
    //pub fn move_cursor_end(&mut self) -> &mut Self {
    //   self.cursor.end();
    //    self
    //}

    pub fn addAndMoveRight(&mut self, item: char) -> &mut Self {
        self
            .change_cursor_item_to(item)
            .move_cursor_right()
    }

}

struct Field {
    buffer: BufferedCursor,
    blink: RectangularWave,
    start_point: Point,
    edit_mode: EditMode,
}

impl Field {
    fn new(start_point: Point, array: String<10>) -> Self {
        Self {
            buffer: BufferedCursor::new(array),
            blink: RectangularWave::new(400,700),
            start_point,
            edit_mode: EditMode::new(false),
        }
    }
}

impl Widget for Field {

    fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            let effect = match key {
                // navigation_key left and right
                KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.buffer.move_cursor_left(); Some(()) }, 
                KeyCode::KEY_SETA_BRANCA_DIREITA => { self.buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.buffer.move_cursor_left(); Some(()) },
                KeyCode::KEY_0 => { self.buffer.change_cursor_item_to('0').move_cursor_right(); Some(()) },
                KeyCode::KEY_1 => { self.buffer.change_cursor_item_to('1').move_cursor_right(); Some(()) },
                KeyCode::KEY_2 => { self.buffer.change_cursor_item_to('2').move_cursor_right(); Some(()) },
                KeyCode::KEY_3 => { self.buffer.change_cursor_item_to('3').move_cursor_right(); Some(()) },
                KeyCode::KEY_4 => { self.buffer.change_cursor_item_to('4').move_cursor_right(); Some(()) },
                KeyCode::KEY_5 => { self.buffer.change_cursor_item_to('5').move_cursor_right(); Some(()) },
                KeyCode::KEY_6 => { self.buffer.change_cursor_item_to('6').move_cursor_right(); Some(()) },
                KeyCode::KEY_7 => { self.buffer.change_cursor_item_to('7').move_cursor_right(); Some(()) },
                KeyCode::KEY_8 => { self.buffer.change_cursor_item_to('8').move_cursor_right(); Some(()) },
                KeyCode::KEY_9 => { self.buffer.change_cursor_item_to('9').move_cursor_right(); Some(()) },
                KeyCode::KEY_ESC => { self.set_edit_mode(false); Some(()) },
                _ => { None },
            };
    
            // reset the blinker when some key is pressed makes a better visual effect
            if let Some(_) = effect {
                self.blink.reset();
            }  
        } else {
            // ignore keys
        }
    }

    fn update(&mut self) {
        self.blink.update();
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_cursor(self.start_point);
        for (position,digit) in self.buffer.buffer.char_indices() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.buffer.cursor.get_current();
            let is_time_to_blink = self.blink.read() && self.is_in_edit_mode(); // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }
}

impl Editable for Field {
    fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode
    }
}


struct SubMenuItem {
    caption: Caption,
    field: Field,
}

impl SubMenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    fn new(point1: Point, text: FlashString, point2: Point, array: String<10>) -> Self {
        Self {
            caption: Caption::new(point1, text),
            field: Field::new(point2, array),
        }
    }

    fn set_caption(&mut self, text: FlashString) {
        self.caption.set_caption(text);
    }
}

impl Widget for SubMenuItem {
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

impl Editable for SubMenuItem {
    fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}

pub struct SubMenu {
    items: Vec<FlashString, 35>,
    item_cursor: Cursor,
    display_cursor: Cursor,
    is_in_edit_mode: bool,
    displayed_items: [SubMenuItem; 2],
}

impl SubMenu {
    pub fn new(items: Vec<FlashString, 35>) -> Self {
        let s1 = FlashString::new(&NOP1);
        let s2 = FlashString::new(&NOP2);
        let f1: String<10> = String::from("0000");
        let f2: String<10> = String::from("00000");
        Self {
            items: items.clone(),
            item_cursor: Cursor::new(0..items.len()), // number of items to show
            display_cursor: Cursor::new(0..2), // number of lines in the display 
            is_in_edit_mode: false,
            displayed_items: [
                SubMenuItem::new(Point::new(2,0), s1, Point::new(35,0), f1),
                SubMenuItem::new(Point::new(2,1), s2, Point::new(34,1), f2),
            ]
        }
    }
}

impl Editable for SubMenu {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode = value;
    }

    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode
    }
}

impl Widget for SubMenu {
    fn send_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                if self.is_in_edit_mode == false {
                    let overflow = self.display_cursor.previous();
                    if overflow {
                        self.item_cursor.previous();
                    }
                    
                };
            }

            KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                if self.is_in_edit_mode == false {
                    let overflow = self.display_cursor.next();
                    if overflow {
                        self.item_cursor.next();
                    }
                };
            }

            KeyCode::KEY_ENTER => {
                self.set_edit_mode(true);
            }

            KeyCode::KEY_ESC => {
                self.set_edit_mode(false);
            }

            _ => {
                // do nothing
            }
        }
    }

    fn update(&mut self) {
        for (index, menu_item) in self.displayed_items.iter_mut().enumerate() {
            let default = FlashString::new(&NAO_IDENTIFICADO);
            let items = self.items.clone();
            let index = self.item_cursor.get_current()+index;
            let text = items.get(index).unwrap_or(&default); //_or(&default).clone();            
            menu_item.set_caption(*text);
            menu_item.update();
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        // clear screen
        canvas.clear();
        // draw parameters
        for item in self.displayed_items.iter() {
            item.draw(canvas);
        }
        // draw item selector icon
        // clear
        for line in 0..2 {
            canvas.set_cursor(Point::new(0, line));
            if line as usize == self.display_cursor.get_current() {
                if self.is_in_edit_mode {
                    canvas.print_char('*');
                } else {
                    canvas.print_char('>');
                }
            } else {
                canvas.print_char(' ');
            }
        };

    }
}
