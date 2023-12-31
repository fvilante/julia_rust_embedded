//! Timer 16 bits interrupt setup and handling
//!
//! Below code has beeing derived from code here:
//! https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-timer.rs

/*
 This is derived from Rahix' comment to
https://github.com/Rahix/avr-hal/issues/75
and then modernized to account for API drift since 2020
*/

use avr_device::atmega328p::tc1::tccr1b::CS1_A;
use avr_device::atmega328p::TC1;
use core::mem;
//use panic_halt as _;

struct InterruptState {
    clock_counter: ClockCounter, // increments on each tick of the clock
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

fn rig_timer(tmr1: &TC1) {
    /*
     https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
     section 15.11
    */
    use arduino_hal::clock::Clock;

    const ARDUINO_UNO_CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_256;
    let clock_divisor: u32 = match CLOCK_SOURCE {
        CS1_A::DIRECT => 1,
        CS1_A::PRESCALE_8 => 8,
        CS1_A::PRESCALE_64 => 64,
        CS1_A::PRESCALE_256 => 256,
        CS1_A::PRESCALE_1024 => 1024,
        CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => 1,
    };

    let ticks = calc_overflow(ARDUINO_UNO_CLOCK_FREQUENCY_HZ, 1000, clock_divisor) as u16;

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

pub fn init_timer() {
    fn set_initial_state(initial_state: InterruptState) {
        unsafe {
            // SAFETY: Interrupts are not enabled at this point so we can safely write the global
            // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
            // after any operation that enables interrupts.
            INTERRUPT_STATE = mem::MaybeUninit::new(initial_state);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
    }

    fn configure_timer() {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let tmr1: TC1 = dp.TC1;
        rig_timer(&tmr1);
    }

    fn enable_interrupts_globally() {
        // Enable interrupts globally, not a replacement for the specific interrupt enable
        unsafe {
            // SAFETY: Not inside a critical section and any non-atomic operations have been completed
            // at this point.
            avr_device::interrupt::enable();
        }
    }

    let clock_counter = ClockCounter::new();

    //Do all
    set_initial_state(InterruptState { clock_counter });
    configure_timer();
    enable_interrupts_globally();
}

// This is the timer interruption handler
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let state = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };

    state.clock_counter.increment();

    //lcd::clear();
    //lcd::print_u16_in_hex(state.clock_counter.read().try_into().unwrap());
}

//

struct ClockCounter {
    count: u64,
}

impl ClockCounter {
    fn new() -> Self {
        Self { count: 0x00 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn read(&self) -> u64 {
        self.count.clone()
    }
}

/// Returns time since power-on
///
/// expected one tick every 1 milisec. NOTE: Period may change in future implementations.
/// this implemantation is to be considered an initial version.
pub fn now() -> u64 {
    let state = unsafe { &*INTERRUPT_STATE.as_ptr() };

    let value = state.clock_counter.read();
    value
}
