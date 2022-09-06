#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate avr_std_stub;

mod common;
mod microcontroler;
mod board;

use board::debug::set_led3;

// This example was from here: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-timer.rs

/*
 This is derived from Rahix' comment to
https://github.com/Rahix/avr-hal/issues/75
and then modernized to account for API drift since 2020
*/

use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use avr_device::atmega328p::tc1::tccr1b::CS1_A;
use avr_device::atmega328p::TC1;
use core::mem;
//use panic_halt as _;

struct InterruptState {
    counter: u32,
    step: u32,
    led_state: bool,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState {
            counter: 0,
            step: 1,
            led_state: false,
        });
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    //
    
    let tmr1: TC1 = dp.TC1;

    rig_timer(&tmr1);

    // Enable interrupts globally, not a replacement for the specific interrupt enable
    unsafe {
        // SAFETY: Not inside a critical section and any non-atomic operations have been completed
        // at this point.
        avr_device::interrupt::enable();
    }

    loop {
        avr_device::asm::sleep()
    }
}

pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

pub fn rig_timer(tmr1: &TC1) {
    /*
     https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
     section 15.11
    */
    use arduino_hal::clock::Clock;

    const ARDUINO_UNO_CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_8;
    let clock_divisor: u32 = match CLOCK_SOURCE {
        CS1_A::DIRECT => 1,
        CS1_A::PRESCALE_8 => 8,
        CS1_A::PRESCALE_64 => 64,
        CS1_A::PRESCALE_256 => 256,
        CS1_A::PRESCALE_1024 => 1024,
        CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => {
            1
        }
    };

    let ticks = calc_overflow(ARDUINO_UNO_CLOCK_FREQUENCY_HZ, 10, clock_divisor) as u16;

    tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));
    tmr1.tccr1b.write(|w| {
        w.cs1()
            //.prescale_256()
            .variant(CLOCK_SOURCE)
            .wgm1()
            .bits(0b01)
    });
    tmr1.ocr1a.write(|w| unsafe { w.bits(ticks) });
    tmr1.timsk1.write(|w| w.ocie1a().set_bit()); //enable this specific interrupt
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let state = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };
    
    state.counter += 1;

    if state.counter >= state.step {
        state.counter = 0;
        state.step = state.step + 1;
        //togle_led
        state.led_state = {
            set_led3(!state.led_state);
            !state.led_state
        };
    } else {
        
    }
}
