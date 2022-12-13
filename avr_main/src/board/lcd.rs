// DEPRECATED: use lcd_2.rs instead!
// FIX: When possible remove this deprecated model from project
// driver for off-board lcd HITACH HD44780 display

use ruduino::cores::atmega328p::port;
use ruduino::Pin;

use crate::microcontroler::delay::{delay_ms, delay_us};

use lib_1::utils::common::{convert_u16_to_str_hex, convert_u8_to_str_hex, get_bit_at_as_bool};

const HIGH: bool = true;
const LOW: bool = false;

const MAX_LINES: u8 = 4;

// lcd internal state
static mut _DISPLAYFUNCTIO: u8 = 0x00;
static mut _DISPLAYCONTROL: u8 = 0x00;
static mut _DISPLAYMODE: u8 = 0x00;
static mut _ROW_OFFSETS: [u8; 4] = [0; 4];
static mut _NUMLINES: u8 = 0x00;

fn init_lcd_pins() -> () {
    port::B4::set_output(); // lcd_rs = PB4
    port::B5::set_output(); // lcd_enable = PB5
                            //
    port::C0::set_output(); // lcd_db7 = PC0
    port::C1::set_output(); // lcd_db6 = PC1
    port::C2::set_output(); // lcd_db5 = PC2
    port::C3::set_output(); // lcd_db4 = PC3

    //
    //lcd_rs(?);
    //lcd_enable(?);
    //lcd_db7(?);
    //lcd_db6(?);
    //lcd_db5(?);
    //lcd_db4(?);
}

fn lcd_rs(value: bool) -> () {
    if value == HIGH {
        port::B4::set_high();
    } else {
        port::B4::set_low();
    };
}

fn lcd_enable(value: bool) -> () {
    if value == HIGH {
        port::B5::set_high();
    } else {
        port::B5::set_low();
    };
}

fn lcd_db7(value: bool) -> () {
    if value == HIGH {
        port::C0::set_high();
    } else {
        port::C0::set_low();
    };
}

fn lcd_db6(value: bool) -> () {
    if value == HIGH {
        port::C1::set_high();
    } else {
        port::C1::set_low();
    };
}

fn lcd_db5(value: bool) -> () {
    if value == HIGH {
        port::C2::set_high();
    } else {
        port::C2::set_low();
    };
}

fn lcd_db4(value: bool) -> () {
    if value == HIGH {
        port::C3::set_high();
    } else {
        port::C3::set_low();
    };
}

// ---------------------------------------------------------------------------
/************ low level data pushing commands **********/

fn pulse_enable() -> () {
    lcd_enable(LOW);
    delay_us(1);
    lcd_enable(HIGH);
    delay_us(1); // enable pulse must be >450ns
    lcd_enable(LOW);
    delay_us(100); // commands need > 37us to settle
}

// write low four bits of data in the lcd data output channel
fn write4bits(data: u8) -> () {
    lcd_db4(get_bit_at_as_bool(data, 0));
    lcd_db5(get_bit_at_as_bool(data, 1));
    lcd_db6(get_bit_at_as_bool(data, 2));
    lcd_db7(get_bit_at_as_bool(data, 3));
    //
    pulse_enable();
}

// write either command or data
fn send(value: u8, rs_mode: bool) -> () {
    // @@ or "mode:u8" ?? (Please check and remove this line if possible)
    lcd_rs(rs_mode);
    write4bits(value >> 4); // most significant bits
    write4bits(value); // least significant bits
}

// ---------------------------------------------------------------------------
/*********** mid level commands, for sending data/cmds */

fn command(value: u8) -> () {
    send(value, LOW);
}

// print just one byte
fn write_u8(value: u8) -> () {
    send(value, HIGH);
    //return 1; // assume sucess // @@ line removed by considered unecessary. (Please check and remove this line if possible)
}

// --------------------------------------------------------------------------
// Very high level user functions

// print just one byte
pub fn print_bit(bit: bool) -> () {
    if bit == true {
        print_char('1');
    } else {
        print_char('0');
    }
}

// print just one byte
pub fn print_u8(value: u8) -> () {
    write_u8(value);
}

pub fn print_u8_in_hex(value: u8) -> () {
    let (high, low) = convert_u8_to_str_hex(value);
    print_char(high);
    print_char(low);
    print_char('h');
}

pub fn print_u16_in_hex(value: u16) -> () {
    let (Q3, Q2, Q1, Q0) = convert_u16_to_str_hex(value);
    print_char(Q3);
    print_char(Q2);
    print_char(Q1);
    print_char(Q0);
    print_char('h');
}

// print just one byte
pub fn print_char(char: char) -> () {
    write_u8(char as u8);
}

// prints a full string
pub fn print(text: &str) -> () {
    for char in text.as_bytes() {
        write_u8(*char);
    }
}

// prints a full string
pub fn print_u8_array<const N: usize>(text: &[u8; N]) -> () {
    for char in text {
        let char_ = *char; // deref
        if char_ > 0 {
            write_u8(char_);
        }
    }
}

// --------------------------------------------------------------------------
/********** high level commands, for the user! */
pub fn clear() -> () {
    command(LCD_CLEARDISPLAY); // clear display, set cursor position to zero
    delay_us(2000); // this command takes a long time!
}

fn home() -> () {
    command(LCD_RETURNHOME); // set cursor position to zero
    delay_us(2000); // this command takes a long time!
}

pub fn setCursor(col: u8, row: u8) {
    //bugfix: I don't discoved why but row 1 is mapped to number 2 instead of number 1.
    //        I'm implementing this simple workaround. This will not become an issue if
    //        display stays with just 2 lines.
    let row_bugfixed: u8 = match row {
        0 => 0,
        1 => 2,
        2.. => 2,
    };

    let max_lines: u8 = MAX_LINES; // @@ original code: "const size_t max_lines = sizeof(_row_offsets) / sizeof(*_row_offsets)"";
    let mut row_temp: usize = 0x00;
    // safe guard
    if row_bugfixed >= max_lines {
        row_temp = (max_lines - 1) as usize; // we count rows starting w/0
    }
    if row_bugfixed >= unsafe { _NUMLINES } {
        row_temp = (unsafe { _NUMLINES } - 1) as usize; // we count rows starting w/0
    }

    command(LCD_SETDDRAMADDR | (col + unsafe { _ROW_OFFSETS[row_temp] }));
}

// Turn the display on/off (quickly)
fn noDisplay() {
    unsafe {
        _DISPLAYCONTROL &= !LCD_DISPLAYON;
    }; // @@ check if in rust the equivalent of clang negation symbol '~' is '!' (Please check and remove this line if possible)
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

fn display() -> () {
    unsafe {
        _DISPLAYCONTROL |= LCD_DISPLAYON;
    }
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

// Turns the underline cursor on/off
fn noCursor() -> () {
    unsafe {
        _DISPLAYCONTROL &= !LCD_CURSORON;
    }; // @@ check if in rust the equivalent of clang negation symbol '~' is '!' (Please check and remove this line if possible)
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

fn cursor() -> () {
    unsafe {
        _DISPLAYCONTROL |= LCD_CURSORON;
    };
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

// Turn on and off the blinking cursor
fn noBlink() -> () {
    unsafe {
        _DISPLAYCONTROL &= !LCD_BLINKON;
    }; // @@ check if in rust the equivalent of clang negation symbol '~' is '!' (Please check and remove this line if possible)
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

fn blink() -> () {
    unsafe {
        _DISPLAYCONTROL |= LCD_BLINKON;
    };
    command(LCD_DISPLAYCONTROL | unsafe { _DISPLAYCONTROL });
}

// These commands scroll the display without changing the RAM
fn scrollDisplayLeft() -> () {
    command(LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVELEFT);
}

fn scrollDisplayRight() -> () {
    command(LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVERIGHT);
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
/*
in some 16x4 LCD when line 3 and 4 are not placed correctly you may try:
    setRowOffsets(0x00, 0x40, 0x14, 0x54)
or
    setRowOffsets(0x00, 0x40, 0x10, 0x50)
 */
fn setRowOffsets(row0: u8, row1: u8, row2: u8, row3: u8) -> () {
    //setRowOffsets(0x00, 0x40, 0x00 + cols, 0x40 + cols); <-- call example
    unsafe {
        _ROW_OFFSETS[0] = row0;
        _ROW_OFFSETS[1] = row1;
        _ROW_OFFSETS[2] = row2;
        _ROW_OFFSETS[3] = row3;
    }
}

// commands
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// flags for display on/off control
const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

// flags for display/cursor shift
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_CURSORMOVE: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_MOVELEFT: u8 = 0x00;

// flags for function set
const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5X10DOTS: u8 = 0x04;
const LCD_5X8DOTS: u8 = 0x00;

fn lcd_init() {
    init_lcd_pins();

    unsafe {
        _DISPLAYFUNCTIO = LCD_4BITMODE | LCD_1LINE | LCD_5X8DOTS;
    }

    //lcd_begin(16,1);
}

fn lcd_begin(cols: u8, lines: u8) {
    if lines > 1 {
        unsafe {
            _DISPLAYFUNCTIO |= LCD_2LINE;
        };
    }
    unsafe {
        _NUMLINES = lines;
    };

    setRowOffsets(0x00, 0x40, 0x00 + cols, 0x40 + cols);

    // SEE PAGE 45/46 FOR INITIALIZATION SPECIFICATION!
    // according to datasheet, we need at least 40ms after power rises above 2.7V
    // before sending commands. Arduino can turn on way before 4.5V so we'll wait 50
    delay_us(50000); // line 104

    // Now we pull both RS and R/W low to begin commands
    lcd_rs(LOW);
    lcd_enable(LOW);

    // put the LCD into 4 bit (or 8 bit mode, but not is the case)
    // this is according to the hitachi HD44780 datasheet
    // figure 24, pg 46

    // we start in 8bit mode, try to set 4 bit mode
    write4bits(0x03);
    delay_us(4500); // wait min 4.1ms

    // second try
    write4bits(0x03);
    delay_us(4500); // wait min 4.1ms

    // third go!
    write4bits(0x03);
    delay_us(150);

    // finally, set to 4-bit interface
    write4bits(0x02);

    // ==========================

    // finally, set # lines, font size, etc.
    command(LCD_FUNCTIONSET | unsafe { _DISPLAYFUNCTIO });

    // turn the display on with no cursor or blinking default
    unsafe { _DISPLAYCONTROL = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF };
    display();

    // clear it off
    clear();

    // Initialize to default text direction (for romance languages)
    unsafe { _DISPLAYMODE = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT };
    // set the entry mode
    command(LCD_ENTRYMODESET | unsafe { _DISPLAYMODE });
}

// API ---------------------------------------

pub fn lcd_initialize() -> () {
    lcd_init();
    lcd_begin(40, 2);
}

// ------------------------------------------
// Examples

pub fn example_01() -> ! {
    lcd_initialize();
    // cursor(); // This function is not working properly must be debuged

    let icon: char = 'X';

    loop {
        clear();
        setCursor(10, 0);
        print("Julia AVR Rust");
        setCursor(10, 1);
        print("@FlavioVilante");
        for row in 0..2 {
            for col in 0..40 {
                setCursor(col, row);
                print_char(icon);
                delay_ms(100);
                setCursor(col, row);
                print_char(' ');
            }
        }
    }
}
