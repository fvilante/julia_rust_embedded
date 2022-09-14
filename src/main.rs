#![allow(non_snake_case)] // remove this line when possible
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]


#![allow(unused_imports)]
#![allow(dead_code)]

extern crate avr_std_stub;

mod common;
mod microcontroler;
mod board;
mod protocol;


use microcontroler::{
    timer::init_timer,
};



#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {

    protocol::datalink_out::development_entry_point();



    /*    
    init_timer();

    //

    loop {
         // infinite loop waiting for timer interruptions to occur
         // see function TIMER1_COMPA() in crate::microcontroler::timer
    }
    */
}





