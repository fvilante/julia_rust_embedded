#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(alloc_error_handler)] // necessary to execute file ./custom_alloc/alloc_error_handler.rs
#![feature(exclusive_range_pattern)]
#![allow(non_snake_case)] // remove this line when possible
#![allow(dead_code)]
#![feature(lang_items)] // Necessary to eh_personality and to run "cargo fix" on the code.
                        //
extern crate alloc;
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
mod custom_alloc;

// Load main modules

mod app;
mod board;
mod geometry;
mod menu;
mod microcontroler;
mod panic;
mod string;

// Initialize app

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    // Application entry point
    app::run()
}
