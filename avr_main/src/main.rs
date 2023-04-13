
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(alloc_error_handler)] // necessary to execute file ./custom_alloc/alloc_error_handler.rs
#![feature(exclusive_range_pattern)]
#![allow(non_snake_case)] // remove this line when possible
#![allow(dead_code)]
#![feature(unchecked_math)]
#![feature(lang_items)] // Necessary to eh_personality and to run "cargo fix" on the code.

extern crate alloc;

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

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    // Entry point
    menu::menu_entry_point::development_entry_point()
}
