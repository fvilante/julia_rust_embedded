use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::{ Keypad, KeyCode };
use crate::enviroment::front_panel::FrontPanel;
use crate::microcontroler::delay::delay_ms;


struct Keyboard {
    keypad: Keypad,
    last_key: KeyCode,
    beep: fn(bool),
}

impl Keyboard {
    pub fn new(beep: fn(on: bool)) -> Self {
        Self {
            keypad: Keypad::new(),
            last_key: KeyCode::NO_KEY,
            beep,
        }
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        //TODO: Extract this code to a better place
        (self.beep)(true); // init beep
        delay_ms(200); // debounce the key
        (self.beep)(false); // stop beep
        //
        let key = self.keypad.scan();
        if key == KeyCode::NO_KEY {
            None
        } else {
            Some(key)
        }
    }
}

struct Canvas {
    is_initialized: bool,
}

impl Canvas  {

    fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            is_initialized: true,
        }
    }

    fn print(&self, info: &str) {
        lcd::print(info);
    }

    fn print_char(&self, char: char) {
        lcd::print_char(char);
    }


    fn set_cursor(&self, col: u8, row: u8) {
        lcd::setCursor(col, row);
    }

    fn clear(&self) {
        lcd::clear();
    }
}



struct Caption<'a> {
    canvas: &'a Canvas,
    text: &'a str,
}

impl<'a> Caption<'a> {
    fn new(text: &'a str, canvas: &'a Canvas) -> Self {
        Self {
            text,
            canvas,
        }
    }

    fn draw(&self) {
        self.canvas.print(self.text);
    }
}

//


struct BufferedCursor<'a,T, const SIZE: usize> {
    buffer: &'a mut [T;SIZE],
    cursor: usize, // buffer index
}

impl<'a,T, const SIZE: usize> BufferedCursor<'a,T, SIZE> {
    pub fn new(buffer: &'a mut [T;SIZE]) -> Self {
        Self {
            buffer,
            cursor: 0,
        }
    }


    pub fn change_cursor_item_to(&mut self, item: T) -> &mut Self {
        self.buffer[self.cursor] = item;
        self
    }

    /// increment_cursor_safe
    pub fn move_cursor_right(&mut self) -> &mut Self {
        let upper_bound = self.buffer.len()-1;
        if self.cursor < upper_bound {
            self.cursor += 1;
        };
        self
    }

    /// decrement_cursor_safe
    pub fn move_cursor_left(&mut self) -> &mut Self {
        let lower_bound = 0;
        if self.cursor > lower_bound {
            self.cursor -= 1;
        };
        self
    }

    pub fn move_cursor_begin(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }

    pub fn move_cursor_end(&mut self) -> &mut Self {
        self.cursor = self.buffer.len()-1;
        self
    }

    pub fn addAndMoveRight(&mut self, item: T) -> &mut Self {
        self
            .change_cursor_item_to(item)
            .move_cursor_right()
    }

    pub fn as_array(&self) -> &[T; SIZE] {
        self.buffer
    }


}


enum FieldKind {
    Numeric,
}

struct Field<'a, const SIZE: usize> {
    canvas: &'a Canvas,
    buffer: BufferedCursor<'a,char,SIZE>,
    kind: FieldKind,
}

impl<'a, const SIZE: usize> Field<'a, SIZE> {
    fn new(buffer: BufferedCursor<'a,char,SIZE>, kind: FieldKind, canvas: &'a Canvas) -> Self {
        Self {
            canvas,
            buffer,
            kind,
        }
    }

    fn update(&mut self, key: KeyCode) {
        match key {
            // navigation_key left and right
            KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.buffer.move_cursor_left(); }, 
            KeyCode::KEY_SETA_BRANCA_DIREITA => { self.buffer.move_cursor_right(); },
            KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.buffer.move_cursor_right(); },
            KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.buffer.move_cursor_left(); },
            KeyCode::KEY_0 => { self.buffer.change_cursor_item_to('0').move_cursor_right(); },
            KeyCode::KEY_1 => { self.buffer.change_cursor_item_to('1').move_cursor_right(); },
            KeyCode::KEY_2 => { self.buffer.change_cursor_item_to('2').move_cursor_right(); },
            KeyCode::KEY_3 => { self.buffer.change_cursor_item_to('3').move_cursor_right(); },
            KeyCode::KEY_4 => { self.buffer.change_cursor_item_to('4').move_cursor_right(); },
            KeyCode::KEY_5 => { self.buffer.change_cursor_item_to('5').move_cursor_right(); },
            KeyCode::KEY_6 => { self.buffer.change_cursor_item_to('6').move_cursor_right(); },
            KeyCode::KEY_7 => { self.buffer.change_cursor_item_to('7').move_cursor_right(); },
            KeyCode::KEY_8 => { self.buffer.change_cursor_item_to('8').move_cursor_right(); },
            KeyCode::KEY_9 => { self.buffer.change_cursor_item_to('9').move_cursor_right(); },
            _ => { },
        }
    }

    fn draw(&self) {
        self.canvas.clear();
        for digit in self.buffer.as_array() {
            self.canvas.print_char(digit.clone());
        };
    }

  
}


pub fn development_entry_point() -> ! {

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let canvas = Canvas::new();
    
    
    let caption = Caption::new("My name is...", &canvas);

    caption.draw();

    let mut buffer = ['x';5];
    let mut cursor = BufferedCursor::new(&mut buffer);
    let mut field = Field::new(cursor, FieldKind::Numeric, &canvas);

    loop { 
        // scan: read one key on keyboard
        // update: send key to the Field
        if let Some(key) = keyboard.get_key() {
            field.update(key);
            field.draw();
        }
        // draw: draw the Field
    }
}