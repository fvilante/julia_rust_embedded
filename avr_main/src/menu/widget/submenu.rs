use avr_progmem::progmem;

use crate::{menu::{flash::FlashString, point::Point, canvas::Canvas}, board::keyboard::KeyCode};

use super::{cursor::Cursor, menu_item::MenuItem, widget::Editable, widget::Widget};

use heapless::{
    String,
    Vec,
};

progmem! {
    static progmem string NOP1 = "nop1";
    static progmem string NOP2 = "nop2";
    static progmem string NAO_IDENTIFICADO = "Nao identificado";
}

const MAX_ITEMS_PER_SUB_MENU: usize = 12; 
const NUMBER_OF_ROWS_IN_SUBMENU: usize = 2;

pub type Items = Vec<FlashString, MAX_ITEMS_PER_SUB_MENU>;

pub struct SubMenu {
    items: Items,
    item_cursor: Cursor,
    display_cursor: Cursor,
    is_in_edit_mode: bool,
    displayed_items: [MenuItem; NUMBER_OF_ROWS_IN_SUBMENU],
}

impl SubMenu {
    pub fn new(items: Items) -> Self {
        let s1 = FlashString::new(&NOP1);
        let s2 = FlashString::new(&NOP2);
        let f1: String<10> = String::from("0000");
        let f2: String<10> = String::from("00000");
        let menu_item1 = MenuItem::new(Point::new(2,0), s1, Point::new(35,0), f1);
        let menu_item2 = MenuItem::new(Point::new(2,1), s2, Point::new(34,1), f2);
        Self {
            items: items.clone(),
            item_cursor: Cursor::new(0..items.len()), // number of items to show
            display_cursor: Cursor::new(0..2), // number of lines in the display 
            is_in_edit_mode: false,
            displayed_items: [
                menu_item1,
                menu_item2,
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