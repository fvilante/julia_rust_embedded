//! Driver for off-board lcd HITACH HD44780 display
//!
//! > *IMPORTANT*: You need to call the function [`lcd_initialize`] at least once in your program,
//! > and before call any other function of this library.
//!
//! # Example
//!
//! ```
//! pub fn main() -> ! {
//!     lcd_initialize();
//!     // cursor(); // This function is not working properly must be debuged
//!
//!     let icon: char = 'X';
//!
//!     loop {
//!         clear();
//!         set_cursor(10, 0);
//!         print("Julia AVR Rust");
//!         set_cursor(10, 1);
//!         print("@FlavioVilante");
//!         for row in 0..2 {
//!             for col in 0..40 {
//!                 set_cursor(col, row);
//!                 print_char(icon);
//!                 delay_ms(100);
//!                 set_cursor(col, row);
//!                 print_char(' ');
//!             }
//!         }
//!     }
//! }
//! ```
use ruduino::cores::atmega328p::port;
use ruduino::Pin;

use crate::microcontroler::delay::delay_us;

use lib_1::utils::{
    bit_wise::get_bit_at_as_bool,
    numerical::{convert_u16_to_str_hex, convert_u8_to_str_hex},
};

const HIGH: bool = true;
const LOW: bool = false;

/// LCD CONFIGURATION: in this case 40x2
const NUMBER_OF_COLS: u8 = 40;
pub const NUMBER_OF_LINES: u8 = 2;

fn init_lcd_pins() {
    port::B4::set_output(); // lcd_rs = PB4
    port::B5::set_output(); // lcd_enable = PB5
                            //
    port::C0::set_output(); // lcd_db7 = PC0
    port::C1::set_output(); // lcd_db6 = PC1
    port::C2::set_output(); // lcd_db5 = PC2
    port::C3::set_output(); // lcd_db4 = PC3
}

fn lcd_rs(value: bool) {
    if value == HIGH {
        port::B4::set_high();
    } else {
        port::B4::set_low();
    };
}

fn lcd_enable(value: bool) {
    if value == HIGH {
        port::B5::set_high();
    } else {
        port::B5::set_low();
    };
}

fn lcd_db7(value: bool) {
    if value == HIGH {
        port::C0::set_high();
    } else {
        port::C0::set_low();
    };
}

fn lcd_db6(value: bool) {
    if value == HIGH {
        port::C1::set_high();
    } else {
        port::C1::set_low();
    };
}

fn lcd_db5(value: bool) {
    if value == HIGH {
        port::C2::set_high();
    } else {
        port::C2::set_low();
    };
}

fn lcd_db4(value: bool) {
    if value == HIGH {
        port::C3::set_high();
    } else {
        port::C3::set_low();
    };
}

// ---------------------------------------------------------------------------
/************ low level data pushing commands **********/

/// write low four bits of data in the lcd data output channel
fn write4bits(data: u8) {
    /// Pulses the enable pin
    fn pulse_enable() {
        lcd_enable(LOW);
        delay_us(1);
        lcd_enable(HIGH);
        delay_us(1); // enable pulse must be >450ns
        lcd_enable(LOW);
        delay_us(100); // commands need > 37us to settle
    }

    // push data
    lcd_db4(get_bit_at_as_bool(data, 0));
    lcd_db5(get_bit_at_as_bool(data, 1));
    lcd_db6(get_bit_at_as_bool(data, 2));
    lcd_db7(get_bit_at_as_bool(data, 3));
    // pulse enable
    pulse_enable();
}

/// Writes either command or data, base in the value of parameter `rs_mode`.
///
/// If `rs_mode` is false then writes a command, otherwise data
fn send(value: u8, rs_mode: bool) {
    lcd_rs(rs_mode);
    write4bits(value >> 4); // most significant bits
    write4bits(value); // least significant bits
}

// ---------------------------------------------------------------------------
/*********** mid level commands, for sending data/cmds */

/// Sends commands
fn command(value: u8) {
    send(value, LOW);
}

/// Sends data
/// prints just one byte
fn write_u8(value: u8) {
    send(value, HIGH);
}

// --------------------------------------------------------------------------
// Very high level user functions

// TODO: Improve these `high level user functions` to be something more generic.

/// print just one byte
pub fn print_bit(bit: bool) {
    if bit == true {
        print_char('1');
    } else {
        print_char('0');
    }
}

/// print just one byte
pub fn print_u8(value: u8) {
    write_u8(value);
}

pub fn print_u8_in_hex(value: u8) {
    let (high, low) = convert_u8_to_str_hex(value);
    print_char(high);
    print_char(low);
    print_char('h');
}

pub fn print_u16_in_hex(value: u16) {
    let (Q3, Q2, Q1, Q0) = convert_u16_to_str_hex(value);
    print_char(Q3);
    print_char(Q2);
    print_char(Q1);
    print_char(Q0);
    print_char('h');
}

/// print just one byte
pub fn print_char(char: char) {
    write_u8(char as u8);
}

/// prints a full string
pub fn print(text: &str) {
    for char in text.as_bytes() {
        write_u8(*char);
    }
}

/// prints a full string
pub fn print_u8_array<const N: usize>(text: &[u8; N]) {
    for char in text {
        let char_ = *char; // deref
        if char_ > 0 {
            write_u8(char_);
        }
    }
}

/// --------------------------------------------------------------------------
/********** high level commands, for the user! */
pub fn clear() {
    const LCD_CLEARDISPLAY: u8 = 0x01;
    command(LCD_CLEARDISPLAY); // clear display, set cursor position to zero
    delay_us(2000); // this command takes a long time!
}

pub fn set_cursor(col: u8, row: u8) {
    const LCD_SETDDRAMADDR: u8 = 0x80;
    const LINE_0_OFFSET: u8 = 0x00;
    const LINE1_OFFSET: u8 = 0x00 + NUMBER_OF_COLS;

    let line_offset = match row {
        0 => LINE_0_OFFSET,
        _ => LINE1_OFFSET,
    };

    command(LCD_SETDDRAMADDR | (col + line_offset));
}

// other commands
// const LCD_RETURNHOME: u8 = 0x02;
// const LCD_CURSORSHIFT: u8 = 0x10;
// const LCD_SETCGRAMADDR: u8 = 0x40;

// flags for display entry mode
// const LCD_ENTRYRIGHT: u8 = 0x00;
// const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;

// flags for display on/off control
//const LCD_DISPLAYOFF: u8 = 0x00;
//const LCD_CURSORON: u8 = 0x02;
//const LCD_BLINKON: u8 = 0x01;

// flags for display/cursor shift
// const LCD_DISPLAYMOVE: u8 = 0x08;
// const LCD_CURSORMOVE: u8 = 0x00;
// const LCD_MOVERIGHT: u8 = 0x04;
// const LCD_MOVELEFT: u8 = 0x00;

// flags for function set
// const LCD_8BITMODE: u8 = 0x10;
// const LCD_1LINE: u8 = 0x00;
// const LCD_5X10DOTS: u8 = 0x04;

/// Executes part of the initialization protocol for LCD according to its datasheet specification.
///
/// SEE PAGE 45/46 FOR INITIALIZATION SPECIFICATION!
fn initialization_protocol() {
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
}

// API ---------------------------------------

/// Initializes LCD given the number of collumns and lines
///
/// NOTE: This function was just tested in LCD 40x2, but my work with little or no
/// modification in other LCD sizes.
/// NOTE: I made this function still more specialized in LCD 40x2 display, to try to
/// save some bytes of memory state. But the more generic function can be acessed
/// in the repository. See commit: 1cf9c0efe402afa2cfe61b67e3fff476cf1b9f01
pub fn lcd_initialize() {
    const LCD_ENTRYMODESET: u8 = 0x04;
    const LCD_DISPLAYCONTROL: u8 = 0x08;
    const LCD_FUNCTIONSET: u8 = 0x20;
    const LCD_ENTRYLEFT: u8 = 0x02;
    const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;
    const LCD_DISPLAYON: u8 = 0x04;
    const LCD_CURSOROFF: u8 = 0x00;
    const LCD_BLINKOFF: u8 = 0x00;
    const LCD_4BITMODE: u8 = 0x00;
    const LCD_2LINE: u8 = 0x08;
    const LCD_5X8DOTS: u8 = 0x00;

    init_lcd_pins();
    initialization_protocol();

    const _DISPLAYFUNCTION: u8 = LCD_4BITMODE | LCD_2LINE | LCD_5X8DOTS;

    // finally, set # lines, font size, etc.
    command(LCD_FUNCTIONSET | _DISPLAYFUNCTION);

    // turn the display on with no cursor or blinking default
    const _DISPLAYCONTROL: u8 = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
    command(LCD_DISPLAYCONTROL | _DISPLAYCONTROL);

    // Initialize to default text direction (for romance languages)
    const _DISPLAYMODE: u8 = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
    // set the entry mode
    command(LCD_ENTRYMODESET | _DISPLAYMODE);

    // clear it off
    clear();
}
