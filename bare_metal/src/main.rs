#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(alloc_error_handler)] // necessary to execute file ./custom_alloc/alloc_error_handler.rs
#![feature(exclusive_range_pattern)]
#![allow(non_snake_case)] // remove this line when possible
#![allow(dead_code)]
#![feature(lang_items)]
// Necessary to eh_personality and to run "cargo fix" on the code.
//#![feature(return_position_impl_trait_in_trait)] // used in the peripherals factory abstraction
#![feature(cell_update)] // used to update davigation state // TODO: check if this feature is stable enough
#![feature(let_else)]
//
extern crate alloc;
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
mod custom_alloc;

// Load main modules

mod app;
mod board;
mod fatal_error;
mod geometry;
mod menu;
mod microcontroler;
mod panic;
mod printable;
mod string;

// Initialize app
#[no_mangle] // TODO: it seems this line is not necessary remove it if possible.
#[arduino_hal::entry]
fn main() -> ! {
    // Application entry point
    app::run()
}
