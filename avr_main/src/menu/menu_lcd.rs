use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::{ Keypad, KeyCode };
use crate::enviroment::front_panel::FrontPanel;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;


pub struct RectangularWave {
    up_interval: u64,
    down_interval: u64,
    next_interval: u64,
    current_state: bool,
}

impl RectangularWave {
    /// interval in miliseconds
    pub fn new(up_interval: u64, down_interval: u64) -> Self {
        Self {  
            up_interval,
            down_interval,
            next_interval: now() + up_interval,
            current_state: true,
        }   
    }

    pub fn reset(&mut self) -> &mut Self {
        self.next_interval = now() + self.up_interval;
        self.current_state = true;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        let is_it_moment_to_change_level = now() > self.next_interval;
        if is_it_moment_to_change_level {
            if self.current_state == true {
                // up-down edge
                self.current_state = false;
                self.next_interval += self.down_interval; 
            } else {
                // down-up edge
                self.current_state = true;
                self.next_interval += self.up_interval; 
            }
        } else {
            // no nothing
        }
        self
    }

    pub fn read(& self) -> bool {
        self.current_state
    }

}


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


struct CursorPosition { col: u8, row: u8 }

struct Canvas {
    is_initialized: bool,
    cursor_position: CursorPosition, // for screen_buffer_input
    screen_buffer_input: [u8; 80],
    screen_buffer_output: [u8; 80],
}

impl Canvas  {

    fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            is_initialized: true,
            cursor_position: CursorPosition { col: 0, row: 0 },
            screen_buffer_input: ['x' as u8; 80],
            screen_buffer_output: ['x' as u8; 80],
        }
    }

    fn print_char(&self, char: char) {
        lcd::print_char(char);
    }


    fn set_cursor(&mut self, col: u8, row: u8) {
        //self.cursor_position = CursorPosition{col, row};
        lcd::setCursor(col, row);
    }

    fn clear(&self) {
        lcd::clear();
    }
}



struct Caption<'a> {
    text: &'a str,
}

impl<'a> Caption<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
        }
    }

    fn send_key(&mut self, _key: KeyCode) { 
        // ignore key
    }

    fn update() {
        // do nothing
    }

    fn draw(&self, canvas: &'a Canvas) {
        for char_ in self.text.chars() {
            canvas.print_char(char_);
        }
    }
}

//


struct BufferedCursor<T, const SIZE: usize> {
    buffer: [T;SIZE],
    cursor: usize, // buffer index
}

impl<T, const SIZE: usize> BufferedCursor<T, SIZE> {
    pub fn new(buffer: [T;SIZE]) -> Self {
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
        &self.buffer
    }


}


enum FieldKind {
    Numeric,
}

struct Field<const SIZE: usize> {
    buffer: BufferedCursor<char,SIZE>,
    kind: FieldKind,
    blink: RectangularWave,
}

impl<const SIZE: usize> Field<SIZE> {
    fn new(array: [char;SIZE], kind: FieldKind) -> Self {
        Self {
            buffer: BufferedCursor::new(array),
            kind,
            blink: RectangularWave::new(500,500),
        }
    }

    fn send_key(&mut self, key: KeyCode) {        

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
            _ => { None },
        };

        if let Some(_) = effect {
            self.blink.reset();
        }  
    }

    fn update(&mut self) {
        self.blink.update();
    }

    fn draw(&self, canvas: &Canvas) {
        //canvas.clear();
        for (position,digit) in self.buffer.as_array().iter().enumerate() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.buffer.cursor;
            let is_time_to_blink = self.blink.read();
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }

  
}


pub fn development_entry_point() -> ! {

    //temp
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let  canvas = Canvas::new();
    
    //widgets
    let caption = Caption::new("My name is...");
    let mut field = Field::new(['x';5], FieldKind::Numeric);

    loop { 
        // scan: read one key on keyboard
        // update: send key to the Field
        if let Some(key) = keyboard.get_key() {
            field.send_key(key);
        }
        field.update();
        canvas.clear();
        caption.draw(&canvas);
        field.draw(&canvas);
        // draw: draw the Field
    }
}