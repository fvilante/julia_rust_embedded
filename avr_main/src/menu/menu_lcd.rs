use crate::board::lcd;
use crate::board::keyboard::{ Keypad, KeyCode };
use crate::microcontroler::delay::delay_ms;


struct Keyboard {
    keypad: Keypad,
    last_key: KeyCode,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keypad: Keypad::new(),
            last_key: KeyCode::NO_KEY,
        }
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        delay_ms(200); // debounce time
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

enum FieldKind {
    Numeric,
}

struct Field<'a> {
    canvas: &'a Canvas,
    size: u8,
    kind: FieldKind,
}

impl<'a> Field<'a> {
    fn new(size: u8, kind: FieldKind, canvas: &'a Canvas) -> Self {
        Self {
            size,
            kind,
            canvas,
        }
    }

    fn update(&self, key: KeyCode) {
        //self.canvas.clear();
        if key.is_numeral() {
            self.canvas.print("numeral;");
        } else {
            self.canvas.print("nao_numeral;");
        }
    }

    fn draw(&self) {
        unreachable!();
    }

  
}




pub fn development_entry_point() -> ! {

    // initialization
    let mut keyboard = Keyboard::new();
    let canvas = Canvas::new();
    
    let caption = Caption::new("My name is...", &canvas);

    caption.draw();

    let field = Field::new(5, FieldKind::Numeric, &canvas);

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