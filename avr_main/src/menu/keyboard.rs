use crate::board::keyboard::KeyCode;
use crate::board::keyboard::Keypad;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;

type TimePoint = u16;

const DEBOUNCE_TIME: TimePoint = 250; // miliseconds

pub struct Debounce {
    pub last_key_time: TimePoint,
    pub last_key: KeyCode,
}

impl Debounce {
    pub fn new() -> Self {
        Self {
            last_key_time: now() as TimePoint,
            last_key: KeyCode::NO_KEY,
        }
    }

    pub fn debounce_key(&mut self, current_key: KeyCode) -> Option<KeyCode> {
        let last_key_was_none = self.last_key == KeyCode::NO_KEY;
        let current_key_is_some = current_key != KeyCode::NO_KEY;
        if last_key_was_none {
            if current_key_is_some {
                // new key detected
                // register it
                self.last_key = current_key;
                self.last_key_time = now() as TimePoint;
                // initial point
                return Some(current_key);
            } else {
                // waiting key, no key has been pressed yet
                return None;
            }
        } else {
            // last key was some
            let current_and_last_key_are_equal = self.last_key == current_key;
            let current_key_is_none = current_key == KeyCode::NO_KEY;
            if current_key_is_none {
                // key unpressed
                // then reset debounce state
                self.last_key = current_key;
                self.last_key_time = now() as TimePoint;
                return None;
            } else {
                // last and current key are some
                if current_and_last_key_are_equal {
                    let has_debounce_time_been_passed =
                        now() as TimePoint > (self.last_key_time + DEBOUNCE_TIME);
                    if has_debounce_time_been_passed {
                        //TODO: PERFORM repetition code
                        self.last_key = current_key;
                        self.last_key_time = now() as TimePoint;
                        return Some(current_key);
                    } else {
                        return None;
                    }
                } else {
                    // last and current key are some, but they are different
                    // two keys pressed at same time
                    // TODO: Implement 'ctrl + key' code
                    return None;
                }
            }
        }
    }
}

/// High level function to control keyboard key strokes
pub struct Keyboard<'a> {
    pub beep: fn(bool),
    pub debouncer: Debounce,
}

impl Keyboard {
    pub fn new(beep: fn(on: bool)) -> Self {
        Self {
            keypad: Keypad::new(),
            beep,
            debouncer: Debounce::new(),
        }
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        //TODO: put this beep code in a better place and make its timeing non-halting
        let beep = |key| {
            (self.beep)(true);
            delay_ms(20);
            (self.beep)(false);
            key
        };

        let current_key = self.keypad.scan();
        self.debouncer.debounce_key(current_key).map(beep)
    }
}
