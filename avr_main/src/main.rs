#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(alloc_error_handler)] // necessary to execute file ./custom_alloc/alloc_error_handler.rs
#![feature(panic_info_message)]
#![feature(exclusive_range_pattern)]
// necessary to send error messages to the panic_handler
//#![allow(warnings)] // TODO: remove this on future
//#![allow(unused_imports)]
#![allow(non_snake_case)] // remove this line when possible
#![allow(dead_code)]
#![feature(lang_items)] // Necessary to eh_personality and to run "cargo fix" on the code.

extern crate alloc;
//extern crate avr_std_stub;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

mod board;
mod cmpp;
mod custom_alloc;
mod enviroment;
mod menu;
mod microcontroler;
mod panic;
mod utils;

use lib_1;
use microcontroler::timer::init_timer;

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    //protocol::datalink_comm::development_entry_point();
    //board::lcd::example_01();

    init_timer(); // initialize timer couting (1khz)

    //

    //crate::enviroment::front_panel::development_entry_point();
    cmpp::main::development_entry_point()
    //menu::menu_entry_point::development_entry_point()

    //loop {

    //lcd::clear();
    //lcd::print_u16_in_hex(microcontroler::timer::now().try_into().unwrap());
    // infinite loop waiting for timer interruptions to occur
    // see function TIMER1_COMPA() in crate::microcontroler::timer
    //}
}
