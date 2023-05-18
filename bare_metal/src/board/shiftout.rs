//! Driver for output expander made of 4 serially connected shift registers
//!
//! # Characteristics
//!
//! 32bits I/O Expander (using 4 output shift-registers in the board) has max output frequency about => 5kHz
//! perid = 200usec / f = 5khz
//! high pulse width = 4usec
//! note: the focus was just to take a first measurement, no conclusions have beeing derived from that.
//!
//! # Example
//!
//! ```
//! // Speed Test Measurement Results:
//! pub fn example_1() -> ! {
//!     loop {
//!         let mut data: ShiftOutData = ShiftOutData {
//!             byte0: (0x00),
//!             byte1: (0x00),
//!             byte2: (0x00),
//!             byte3: (0x00),
//!         };
//!         write_shiftout(data);
//!         //delay_ms(100);
//!         data = ShiftOutData {
//!             byte0: (0x00),
//!             byte1: (0x00),
//!             byte2: (0x00),
//!             byte3: (1 << 6), // this pulse has 230 usec of period and
//!         };
//!         write_shiftout(data);
//!         //delay_ms(100);
//!     }
//! }
//! ```
//!
//! # Electrical connections on the board
//!
//! OUTPUT_BUS0     KBD-SA                   BIT0 - SHIFT-REGISTER 1 BEGIN
//! OUTPUT_BUS1     KBD-SB                   BIT1
//! OUTPUT_BUS2     KBD-S1                   BIT2
//! OUTPUT_BUS3     KBD-S2                   BIT3
//! OUTPUT_BUS4     KBD-S3                   BIT4
//! OUTPUT_BUS5     KBD-S4                   BIT5
//! OUTPUT_BUS6     KBD-S5                   BIT6
//! OUTPUT_BUS7     KBD-S6                   BIT7
//! OUTPUT_BUS8     KBD-S7                   BIT0 - SHIFT-REGISTER 2 BEGIN
//! OUTPUT_BUS9     KBD-S8                   BIT1
//! OUTPUT_BUS10    SAIDA-VAGO1              BIT2
//! OUTPUT_BUS11    COPIA-SINAL-PTR          BIT3
//! OUTPUT_BUS12    SAIDA-START-OUTRO        BIT4
//! OUTPUT_BUS13    INVMEN                   BIT5
//! OUTPUT_BUS14    P3                       BIT6
//! OUTPUT_BUS15    P2                       BIT7
//! OUTPUT_BUS16    P1                       BIT0 - SHIFT-REGISTER 3 BEGIN
//! OUTPUT_BUS17    P0                       BIT1
//! OUTPUT_BUS18    DMOTOR-1                 BIT2
//! OUTPUT_BUS19    DMOTOR-2                 BIT3
//! OUTPUT_BUS20    EMOTOR-1                 BIT4
//! OUTPUT_BUS21    EMOTOR-2                 BIT5
//! OUTPUT_BUS22    NMOTOR-1                 BIT6
//! OUTPUT_BUS23    NMOTOR-2                 BIT7
//! OUTPUT_BUS24    H/F-1                    BIT0 - SHIFT-REGISTER 4 BEGIN
//! OUTPUT_BUS25    H/F-2                    BIT1
//! OUTPUT_BUS26    BUZZER                   BIT2
//! OUTPUT_BUS27    LED_POS_ALC              BIT3
//! OUTPUT_BUS28    LED_PROGRAMA             BIT4
//! OUTPUT_BUS29    LED_ERRO                 BIT5
//! OUTPUT_BUS30    LED_EXECUCAO             BIT6
//! OUTPUT_BUS31    LED_MANUAL               BIT7
//!
//!
use ruduino::cores::atmega328p::port;
use ruduino::Pin;

const HIGH: bool = true;
const LOW: bool = false;

/// Initialize shift registers
pub fn init_shiftout_pins() {
    port::B0::set_output();
    port::B2::set_output();
    port::D6::set_output();
    port::C5::set_output();
    port::C4::set_output();
    //
    srenab_out(HIGH);
    rclk_out(HIGH);
    srclk_out(HIGH);
    serial_out(LOW);
}

fn serial_out(value: bool) {
    if value == HIGH {
        port::B0::set_high();
    } else {
        port::B0::set_low();
    };
}

fn srclk_out(value: bool) {
    if value == HIGH {
        port::B2::set_high();
    } else {
        port::B2::set_low();
    };
}

fn srclr_out(value: bool) {
    if value == HIGH {
        port::D6::set_high();
    } else {
        port::D6::set_low();
    };
}

fn rclk_out(value: bool) {
    if value == HIGH {
        port::C5::set_high();
    } else {
        port::C5::set_low();
    };
}

fn srenab_out(value: bool) {
    if value == HIGH {
        port::C4::set_high();
    } else {
        port::C4::set_low();
    };
}

/// Represents the data to placed in the PCI Julia on-board output shift-register circuit.
/// Each byte to each shift-register integrated circuit.
#[derive(Copy, Clone)]
pub struct ShiftOutData {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
}

/// This shifts 8 bits out MSB first, on the rising edge of the clock, clock idles low
fn shiftout__(data_out: u8) {
    //clear everything out just in case to
    //prepare shift register for bit shifting
    serial_out(LOW);
    srclk_out(LOW);

    let mut pin_state: bool;

    for i in 0..8 {
        srclk_out(LOW);

        if (data_out & (1 << (7 - i))) >= 1 {
            pin_state = HIGH;
        } else {
            pin_state = LOW;
        }

        //Sets the pin to HIGH or LOW depending on pin_state
        serial_out(pin_state);
        //register shifts bits on upstroke of clock pin
        srclk_out(HIGH);
        //zero the data pin after shift to prevent bleed through
        serial_out(LOW);
    }
}

pub fn write_shiftout(data: ShiftOutData) {
    //enable chips
    srenab_out(LOW);

    //clear register
    srclr_out(LOW);
    srclr_out(HIGH);

    //latch
    rclk_out(LOW);

    //the register attached to the microcontroller goes last
    shiftout__(data.byte3);
    shiftout__(data.byte2);
    shiftout__(data.byte1);
    shiftout__(data.byte0);

    //latch
    rclk_out(HIGH);
}
