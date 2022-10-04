use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::{ Keypad, KeyCode };
use crate::enviroment::front_panel::FrontPanel;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;


struct Debounce {
    debounce_time: u64,
    last_key_time: u64,
    last_key: KeyCode,
}

impl Debounce {
    fn new(debounce_time: u64) -> Self {
        Self {
            debounce_time,
            last_key_time: now(),
            last_key: KeyCode::NO_KEY,
        }
    }

    fn debounce_key(&mut self, current_key: KeyCode) -> Option<KeyCode> {
        let last_key_was_none = self.last_key == KeyCode::NO_KEY;
        let current_key_is_some = current_key != KeyCode::NO_KEY;
        if last_key_was_none {
            if current_key_is_some {
                // new key detected
                // register it
                self.last_key = current_key;
                self.last_key_time = now();
                // initial point
                return Some(current_key);
            } else {
                // waiting key, no key has been pressed yet
                return None;
            }
        } else {
            // last key was some 
            let current_and_last_key_are_equal = self.last_key == current_key;
            let current_key_is_none = current_key == KeyCode::NO_KEY;
            if current_key_is_none {
                // key unpressed
                // then reset debounce state
                self.last_key = current_key;
                self.last_key_time = now();
                return None;
            } else {
                // last and current key are some
                if current_and_last_key_are_equal {
                    let has_debounce_time_been_passed = now() > (self.last_key_time + self.debounce_time);
                    if has_debounce_time_been_passed {
                        //TODO: PERFORM repetition code
                        self.last_key = current_key;
                        self.last_key_time = now();
                        return Some(current_key);
                    } else {
                        return None;
                    }
                } else {
                    // last and current key are some, but they are different
                    // two keys pressed at same time
                    // TODO: Implement 'ctrl + key' code
                    return None;
                }
            }
            
        }
    }

}

struct Keyboard {
    keypad: Keypad,
    last_key: KeyCode,
    beep: fn(bool),
    debouncer: Debounce,
}

impl Keyboard {
    pub fn new(beep: fn(on: bool)) -> Self {
        Self {
            keypad: Keypad::new(),
            last_key: KeyCode::NO_KEY,
            beep,
            debouncer: Debounce::new(250),
        }
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        //TODO: put this beep code in a better place and make its timeing non-halting
        let beep = |key| {
            (self.beep)(true);
            delay_ms(20);
            (self.beep)(false);
            key
        };
        
        let current_key = self.keypad.scan();
        self.debouncer
            .debounce_key(current_key)
            .map(beep)
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

    //temp
    let mut output_expander = OutputExpander::new();
    let front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let canvas = Canvas::new();
    
    
    let caption = Caption::new("My name is...", &canvas);

    caption.draw();

    let mut buffer = ['x';5];
    let  cursor = BufferedCursor::new(&mut buffer);
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