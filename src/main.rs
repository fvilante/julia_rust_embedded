#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate avr_std_stub;

mod common;
mod microcontroler;
mod board;


use microcontroler::{
    timer::init_timer,
};



#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    
    init_timer();

    //

    loop {
         // infinite loop waiting for timer interruptions to occur
         // see function TIMER1_COMPA() in crate::microcontroler::timer
    }
}





