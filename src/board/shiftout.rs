// board io

use ruduino::Pin;
use ruduino::cores::atmega328p::{port};

use crate::microcontroler::delay::{ 
    delay_ms,
    delay_us,
};

use super::debug::{
    set_led3,
    blink_led3,
};

const HIGH: bool = true;
const LOW: bool = false;

const WAIT: u64 = 200;

pub fn init_shiftout_pins() -> () {
    //delay_ms(1);
    port::B0::set_output();
    //delay_us(WAIT);
    port::B2::set_output();
    //delay_us(WAIT);
    port::D6::set_output();
    //delay_us(WAIT);
    port::C5::set_output();
    //delay_us(WAIT);
    port::C4::set_output();
    //delay_us(WAIT);
    //
    srenab_out(HIGH);
    srclr_out(HIGH); // @@
    rclk_out(HIGH);
    srclk_out(HIGH);
    serial_out(LOW);
}

fn serial_out(value: bool) -> () {  
    //delay_us(WAIT);
    if value == HIGH {
        port::B0::set_high();
    } else {
        port::B0::set_low();
    };
}

fn srclk_out(value: bool) -> () {    
    //delay_us(WAIT);
    if value == HIGH {
        port::B2::set_high();
    } else {
        port::B2::set_low();
    };
}

fn srclr_out(value: bool) -> () {    
    //delay_us(WAIT);
    if value == HIGH {
        port::D6::set_high();
    } else {
        port::D6::set_low();
    };
}

fn rclk_out(value: bool) -> () {    
    //delay_us(WAIT);
    if value == HIGH {
        port::C5::set_high();
    } else {
        port::C5::set_low();
    };
}

fn srenab_out(value: bool) -> () {  
    //delay_us(WAIT); 
    if value == HIGH {
        port::C4::set_high();
    } else {
        port::C4::set_low();
    };
}

pub struct ShiftOutData {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
}

fn shiftout__(data_out: u8 ) {
    // This shifts 8 bits out MSB first, 
    // on the rising edge of the clock,
    // clock idles low

    //clear everything out just in case to
    //prepare shift register for bit shifting
    serial_out(LOW);
    srclk_out(LOW);
    //delay_us(50);

    let mut pin_state: bool;

    let mut i = 7;
    let end = 0;
    let step = -1;
    
    for i in 0..8 {

        srclk_out(LOW);
        //delay_us(50);

        if (data_out & (1<<(7-i)))>=1 {
            pin_state = HIGH;
        } else {
            pin_state = LOW;
        }

        //Sets the pin to HIGH or LOW depending on pin_state
        serial_out(pin_state);
        //delay_us(50);
        //register shifts bits on upstroke of clock pin  
        srclk_out(HIGH);
        //delay_us(50);
        //zero the data pin after shift to prevent bleed through
        serial_out(LOW);
        //delay_us(50);
    }
}

pub fn write_shiftout(data: ShiftOutData) -> () {
    //enable chips
    srenab_out(LOW);

    //clear register
    srclr_out(LOW);
    //delay_us(50);
    srclr_out(HIGH);
    //delay_us(50);
    
    //latch
    rclk_out(LOW);
    //delay_us(50);

    //the register attached to the microcontroller goes last
    shiftout__(data.byte3);
    shiftout__(data.byte2);
    shiftout__(data.byte1);
    shiftout__(data.byte0);
    
    //latch
    rclk_out(HIGH);
    //delay_us(50);

}

// eletrical test result:
// serial_out => ok (checar com pulso assimetrico)
// srclk_out => ok
// srclr_out => ok
// rclk_out => ok
pub fn test_signal() -> ! {
    loop {
        serial_out(HIGH);
        srenab_out(HIGH);
        rclk_out(HIGH);
        set_led3(HIGH);
        //delay_ms(300);
        serial_out(LOW);
        srenab_out(LOW);
        rclk_out(LOW);
        set_led3(LOW);
        //delay_ms(100);
    }
}


pub fn test2() -> ! {
    
    init_shiftout_pins();

    //test_signal();

// OUTPUT_BUS0     KBD-SA                   BIT0 - SHIFT-REGISTER 1 BEGIN
// OUTPUT_BUS1     KBD-SB                   BIT1             
// OUTPUT_BUS2     KBD-S1                   BIT2
// OUTPUT_BUS3     KBD-S2                   BIT3
// OUTPUT_BUS4     KBD-S3                   BIT4
// OUTPUT_BUS5     KBD-S4                   BIT5
// OUTPUT_BUS6     KBD-S5                   BIT6
// OUTPUT_BUS7     KBD-S6                   BIT7
// OUTPUT_BUS8     KBD-S7                   BIT0 - SHIFT-REGISTER 2 BEGIN
// OUTPUT_BUS9     KBD-S8                   BIT1
// OUTPUT_BUS10    SAIDA-VAGO1              BIT2
// OUTPUT_BUS11    COPIA-SINAL-PTR          BIT3
// OUTPUT_BUS12    SAIDA-START-OUTRO        BIT4
// OUTPUT_BUS13    INVMEN                   BIT5
// OUTPUT_BUS14    P3                       BIT6
// OUTPUT_BUS15    P2                       BIT7
// OUTPUT_BUS16    P1                       BIT0 - SHIFT-REGISTER 3 BEGIN
// OUTPUT_BUS17    P0                       BIT1
// OUTPUT_BUS18    DMOTOR-1                 BIT2
// OUTPUT_BUS19    DMOTOR-2                 BIT3
// OUTPUT_BUS20    EMOTOR-1                 BIT4
// OUTPUT_BUS21    EMOTOR-2                 BIT5
// OUTPUT_BUS22    NMOTOR-1                 BIT6
// OUTPUT_BUS23    NMOTOR-2                 BIT7
// OUTPUT_BUS24    H/F-1                    BIT0 - SHIFT-REGISTER 4 BEGIN
// OUTPUT_BUS25    H/F-2                    BIT1
// OUTPUT_BUS26    BUZZER                   BIT2
// OUTPUT_BUS27    LED_POS_ALC              BIT3
// OUTPUT_BUS28    LED_PROGRAMA             BIT4
// OUTPUT_BUS29    LED_ERRO                 BIT5
// OUTPUT_BUS30    LED_EXECUCAO             BIT6
// OUTPUT_BUS31    LED_MANUAL               BIT7


    loop {
    
        let mut data: ShiftOutData = ShiftOutData { 
            byte0: (0x00), 
            byte1: (0x00), 
            byte2: (0x00), 
            byte3: (0x00), 
        };
        //serial_out(true);
        write_shiftout(data); 
        //delay_ms(100);
        data = ShiftOutData { 
            byte0: (0x00), 
            byte1: (0x00), 
            byte2: (0x00), 
            byte3: (1<<6), 
        };
        //serial_out(false);
        write_shiftout(data);
        //delay_ms(100);    
    }
}




