use crate::microcontroler::delay::delay_ms;
use crate::board::keyboard::Keypad;
use crate::microcontroler::timer::now;
use crate::board::keyboard::KeyCode;


pub struct Debounce {
    pub debounce_time: u64,
    pub last_key_time: u64,
    pub last_key: KeyCode,
}

impl Debounce {
    pub fn new(debounce_time: u64) -> Self {
        Self {
            debounce_time,
            last_key_time: now(),
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
                self.last_key_time = now();
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
                self.last_key_time = now();
                return None;
            } else {
                // last and current key are some
                if current_and_last_key_are_equal {
                    let has_debounce_time_been_passed = now() > (self.last_key_time + self.debounce_time);
                    if has_debounce_time_been_passed {
                        //TODO: PERFORM repetition code
                        self.last_key = current_key;
                        self.last_key_time = now();
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

pub struct Keyboard {
    pub keypad: Keypad,
    pub last_key: KeyCode,
    pub beep: fn(bool),
    pub debouncer: Debounce,
}

impl Keyboard {
    pub fn new(beep: fn(on: bool)) -> Self {
        Self {
            keypad: Keypad::new(),
            last_key: KeyCode::NO_KEY,
            beep,
            debouncer: Debounce::new(250),
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
        self.debouncer
            .debounce_key(current_key)
            .map(beep)
    }
}
