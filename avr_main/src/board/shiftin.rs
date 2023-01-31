//driver for on-board 3 shift-register serially connected for expand input of microcontroller

use ruduino::cores::atmega328p::port;
use ruduino::Pin;

use crate::board::lcd;
use crate::microcontroler::delay::{delay_ms, delay_us};

use super::shiftout::{write_shiftout, ShiftOutData};

const HIGH: bool = true;
const LOW: bool = false;

fn init_shiftin_pins() -> () {
    port::D2::set_output(); // clk_in       -> PD2
    port::D3::set_output(); // latch_in     -> PD3
    port::D7::set_input(); // serial_in    -> PD7
                           //
    clk_in(HIGH);
    latch_in(HIGH);
}

fn clk_in(value: bool) -> () {
    if value == HIGH {
        port::D2::set_high();
    } else {
        port::D2::set_low();
    };
}

fn latch_in(value: bool) -> () {
    if value == HIGH {
        port::D3::set_high();
    } else {
        port::D3::set_low();
    };
}

fn serial_in() -> bool {
    port::D7::is_high()
}

//
#[derive(PartialEq)]
pub struct ShiftInData {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
}

fn shiftInA() -> u8 {
    //it returns a byte with each bit in the byte corresponding
    //to pin on the shift register. leftBit 7 = pin 7 / bit 0 = pin 0

    let mut data: u8 = 0x00;

    for i in 0..8 {
        clk_in(LOW);
        delay_us(2);
        let bit_in = serial_in();
        if bit_in == true {
            data = data | (1 << (7 - i));
        }
        clk_in(HIGH);
    }
    //return result
    data
}

pub fn readShiftIn() -> ShiftInData {
    //FIX: When possible make this initialization execute once on first execution.
    init_shiftin_pins();

    let mut data: ShiftInData = ShiftInData {
        byte0: 0x00,
        byte1: 0x00,
        byte2: 0x00,
    };

    //Pulse the latch pin:
    //set it to 1 to collect parallel data
    latch_in(HIGH);
    //fix: is this delay necessary?
    delay_us(20);
    //set it to 0 to transmit data serially
    latch_in(LOW);

    //the register attached to the microcontroller comes in first
    data.byte0 = shiftInA();
    data.byte1 = shiftInA();
    data.byte2 = shiftInA();

    //return information
    data
}

//

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();

    let data: ShiftOutData = ShiftOutData {
        byte0: (0x00),
        byte1: (0x00),
        byte2: (0x00),
        byte3: (0x00),
    };
    write_shiftout(data);

    loop {
        let current = readShiftIn();
        lcd::clear();
        lcd::print_u8_in_hex(current.byte0);
        lcd::print(";");
        lcd::print_u8_in_hex(current.byte1);
        lcd::print(";");
        lcd::print_u8_in_hex(current.byte2);
        lcd::print(";");

        delay_ms(1000);
    }
}
