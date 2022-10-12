use core::borrow::{BorrowMut, Borrow};
use core::str::FromStr;
use core::ops::Range;
use alloc::borrow::ToOwned;

use heapless::String;
use heapless::Vec;

use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::{ Keypad, KeyCode };
use crate::enviroment::front_panel::FrontPanel;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;

use avr_progmem::progmem_str as F;
use avr_progmem::progmem;
use avr_progmem::string::PmString;


type Text = String<30>; // Dependeing on string size the memory can be exauhsted


struct FlashString<'a, const N: usize> {
    flash_ref: &'a PmString<N>,
}

impl<'a, const N: usize> FlashString<'a, N> {
    fn new(flash_ref: &'a PmString<N>) -> Self {
        Self {
            flash_ref,
        }
    }

    fn to_string(&self) -> Text {
        let default = String::from("Loading Error");
        let ram_loaded = self.flash_ref.load();
        let str = &*ram_loaded;
        Text::from_str(str).unwrap_or(default)
    }
}



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

    fn print_flash_str<const SIZE: usize>(&mut self, prog_mem_pointer: &PmString<SIZE>) {
        let s = FlashString::new(prog_mem_pointer);
        for char in s.to_string().chars() {
            self.print_char(char);
        }
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
        /// The current implementation of this function is very! very! simplified, it may be improved later    
        lcd::setCursor(0, 0);
        for byte in self.screen_buffer_input {
            lcd::print_u8(byte);
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

struct Caption {
    text: Text,
    start_point: Point,
}

impl Caption {
    fn new(start_point: Point, text: Text) -> Self {
        Self {
            text,
            start_point,
        }
    }

    fn set_caption(&mut self, text: Text) {
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
        for char_ in self.text.chars() {
            canvas.print_char(char_);
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


enum FieldKind {
    Numeric,
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


struct MenuItem {
    caption: Caption,
    field: Field,
}

impl MenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    fn new(point1: Point, text: Text, point2: Point, array: String<10>) -> Self {
        Self {
            caption: Caption::new(point1, text),
            field: Field::new(point2, array),
        }
    }

    fn set_caption(&mut self, text: Text) {
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

struct ClassicMenu {
    items: Vec<Text, 10>,
    item_cursor: Cursor,
    display_cursor: Cursor,
    is_in_edit_mode: bool,
    displayed_items: [MenuItem; 2],
}

impl ClassicMenu {
    fn new(items: Vec<Text, 10>) -> Self {
        let s1: Text = String::from("Fake item 1");
        let s2: Text = String::from("Fake item 2");
        let f1: String<10> = String::from("0000");
        let f2: String<10> = String::from("00000");
        Self {
            items: items.clone(),
            item_cursor: Cursor::new(0..items.len()), // number of items to show
            display_cursor: Cursor::new(0..2), // number of lines in the display 
            is_in_edit_mode: false,
            displayed_items: [
                MenuItem::new(Point::new(2,0), s1, Point::new(35,0), f1),
                MenuItem::new(Point::new(2,1), s2, Point::new(34,1), f2),
            ]
        }
    }
}

impl Editable for ClassicMenu {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode = value;
    }

    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode
    }
}

impl Widget for ClassicMenu {
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
            let default: Text = String::from("Nao identificado");
            let items = self.items.clone();
            let index = self.item_cursor.get_current()+index;
            let text: Text = items.get(index).unwrap_or(&default).clone();            
            menu_item.set_caption(text);
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


struct Sized_<const N: usize> {
    size: [u8;N],
}




pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    //temp
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let mut canvas = Canvas::new();

    progmem! {
        static progmem string S0 = "Posicao Inicial";
        static progmem string S1 = "Posicao Final";
        static progmem string S2 = "Velocidade de Avanco";
        static progmem string S3 = "Velocidade de Retorno";
        static progmem string S4 = "Aceleracao de Avanco";
        static progmem string S5 = "Aceleracao de Retorno";
    }

    //lcd::print("Iniciado");
    //let s0 = FlashString::new(&S0);
    //for char in s0.to_string().chars() {
    //    lcd::print_char(char);
    //}
    //
    //canvas.print_flash_str(&S1);
    //canvas.render();
    //
    //loop { }


    canvas.render();
    
    //widgets
    let default: Text = String::from("Erro de carga de parametro");
    let mut items: Vec<Text, 10> = Vec::new();
    let s0: Text = String::from_str("Posicao Inicial").unwrap_or(default.clone());
    let s1: Text = String::from_str("Posicao Final").unwrap_or(default.clone());
    let s2: Text = String::from_str("Velocidade de Avanco").unwrap_or(default.clone());
    let s3: Text = String::from_str("Velocidade de Retorno").unwrap_or(default.clone());
    let s4: Text = String::from_str("Aceleracao de Avanco").unwrap_or(default.clone());
    let s5: Text = String::from_str("Aceleracao de Retorno").unwrap_or(default.clone());
    let s6: Text = String::from_str("Start automatico no avanco").unwrap_or(default.clone());
    let s7: Text = String::from_str("Start automatico no retorno").unwrap_or(default.clone());
    
    items.push(s0);
    items.push(s1);
    items.push(s2);
    items.push(s3);
    items.push(s4);
    items.push(s5);
    //items.push(s6); 
    //items.push(s7);
    //ATTENTION: Se eu liberar uma das duas linhas comentadas acima da um erro (provavelmente stackoverflow)
    //A solucao é bolar uma forma de alocar todas estas strings na flash (progmem) e descarrega-las na ram lazelly
    //conforme o uso. 


    
    let mut menu = ClassicMenu::new(items);

    canvas.clear();

    let mut c:u16=0;

    loop { 
        //c += 1;
        //lcd::clear();
        //lcd::print_u16_in_hex(c);
        //loop { }
        // scan: read one key on keyboard
        // update: send key to the Field
        if let Some(key) = keyboard.get_key() {
            menu.send_key(key);
        }

  
        // draw: draw the Field
        canvas.render();

        
        // update & draw
        menu.update();
      
        menu.draw(&mut canvas);
       
      

        
        
    }
}