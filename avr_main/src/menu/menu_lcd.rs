use core::borrow::BorrowMut;

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


#[derive(Copy, Clone)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
        }
    }
}


struct CursorPosition { 
    point: Point,
}

impl CursorPosition {

    fn new() -> Self {
        Self {
            point: Point::new(0, 0),
        }
    }

    fn new_from_point(point: Point) -> Self {
        Self {
            point,
        }
    }

    fn from_index(index: usize) -> Self {
        let col: u8 = (index % 40).try_into()./*clamp(0,79).*/unwrap_or(0);
        let row: u8 = (index / 40).try_into()./*clamp(0,1).*/unwrap_or(0);
        let point = Point::new(col, row);
        Self {
            point,
        }
    }
    
    fn get_index(&self ) -> usize {
        let Point{x:col, y:row} = self.point;
        let index = col + (40*row);
        index.into()
    }

    fn set_point(&mut self, point: Point) -> &mut Self {
        self.point = point;
        self
    }

    fn increment(&mut self) -> &mut Self {
        let mut index = self.get_index();
        index += 1;
        let new_point = Self::from_index(index.clamp(0, 79)).point ;
        self.point = new_point;
        self
    }
}

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
            cursor_position: CursorPosition::new_from_point(Point::new(0,0)),
            screen_buffer_input: [' ' as u8; 80],
            screen_buffer_output: ['x' as u8; 80],
        }
    }

    // input part

    fn print_char(&mut self, char: char) {
        let index = self.cursor_position.get_index();
        self.screen_buffer_input[index as usize] = char as u8; //TODO: check if this convertion is safe
        self.cursor_position.increment();
        //lcd::print_char(char);
    }


    fn set_cursor(&mut self, point: Point) {
        self.cursor_position.set_point(point);
        //lcd::setCursor(col, row);
    }

    fn clear(&mut self) {
        self.screen_buffer_input = [' ' as u8; 80];
        self.cursor_position.set_point(Point::new(0,0));
        //lcd::clear();
    }

    // output part


    /// The purpose of this routine is to avoid unecessary writings to LCD.
    /// It swaps two lcd buffers: The output_buffer represents current state of lcd and
    /// input_buffer represent the desired state of lcd
    fn render(&mut self) {
        let mut last_printed_index: isize = -1;
        for (index, input_byte) in self.screen_buffer_input.iter().enumerate() {
            let output_byte = self.screen_buffer_output[index].borrow_mut();
            if input_byte != output_byte {
                let Point{x:col, y:row } = CursorPosition::from_index(index).point;
                //write input to screen
                let does_must_move_cursor_manually = (index - 1) as isize != last_printed_index;
                if does_must_move_cursor_manually {
                    lcd::setCursor(col, row);
                };
                lcd::print_u8(*input_byte);
                //update output buffer
                *output_byte = *input_byte;
                last_printed_index = index as isize;
            }
        }
    }

}


// 


trait Widget {
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


struct Caption<'a> {
    text: &'a str,
    start_point: Point,
}

impl<'a> Caption<'a> {
    fn new(start_point: Point, text: &'a str) -> Self {
        Self {
            text,
            start_point,
        }
    }
}

impl Widget for Caption<'_> {
    fn send_key(&mut self, _key: KeyCode) { 
        // ignore key
    }

    fn update(&mut self) {
        // do nothing
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_cursor(self.start_point);
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
    start_point: Point,
    edit_mode: EditMode,
}

impl<const SIZE: usize> Field<SIZE> {
    fn new(start_point: Point, array: [char;SIZE], kind: FieldKind) -> Self {
        Self {
            buffer: BufferedCursor::new(array),
            kind,
            blink: RectangularWave::new(400,700),
            start_point,
            edit_mode: EditMode::new(false),
        }
    }
}

impl<const SIZE: usize> Widget for Field<SIZE> {

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
        for (position,digit) in self.buffer.as_array().iter().enumerate() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.buffer.cursor;
            let is_time_to_blink = self.blink.read() && self.is_in_edit_mode(); // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }
}

impl<const SIZE: usize> Editable for Field<SIZE> {
    fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode
    }
}


struct MenuItem<'a, const SIZE: usize> {
    caption: Caption<'a>,
    field: Field<SIZE>,
}

impl<'a, const SIZE: usize> MenuItem<'a,SIZE> {
    /// NOTE: client should put point1 and point2 in the same line
    fn new(point1: Point, text: &'a str, point2: Point, array: [char; SIZE]) -> Self {
        Self {
            caption: Caption ::new(point1, text),
            field: Field::<SIZE>::new(point2, array, FieldKind::Numeric),
        }
    }
}

impl<'a, const SIZE: usize> Widget for MenuItem<'a,SIZE> {
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

impl<'a, const SIZE: usize> Editable for MenuItem<'a,SIZE> {
    fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}


pub fn development_entry_point() -> ! {

    //temp
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let mut canvas = Canvas::new();

    canvas.render();

    //loop { }
    
    //widgets
    let mut menu_item1 = MenuItem::new(Point::new(1,0), "Posicao Final", Point::new(35,0), ['0';4]);
    let mut menu_item2 = MenuItem::new(Point::new(1,1), "Aceleracao de avanco", Point::new(34,1), ['0';5]);

    canvas.clear();

    menu_item1.set_edit_mode(true);

    loop { 
        // scan: read one key on keyboard
        // update: send key to the Field
        if let Some(key) = keyboard.get_key() {
            menu_item1.send_key(key);
        }

        // draw: draw the Field
        canvas.render();

        // draw
        menu_item1.update();
        menu_item2.update();
        menu_item1.draw(&mut canvas);
        menu_item2.draw(&mut canvas);
        
        //
        //canvas.clear();
        
    }
}