

use avr_progmem::wrapper::ProgMem;

use avr_progmem::string::PmString;

#[derive(Copy,Clone)]
pub struct FlashString {
    pub(crate) flash_ptr: *const u8,
    pub(crate) size_N: u8, // size in quantity of u8's
}

impl FlashString {
    pub fn new<const N: usize>(val: &PmString<N>) -> Self {
        let ptr = val.as_bytes().as_ptr() as *const u8;
        Self {
            flash_ptr: ptr,
            size_N: N as u8,
        }
    }

    pub fn chars(self) -> FlashStringIterator {
        FlashStringIterator { flash_string: (self), counter: (0) }
    }
}

pub struct FlashStringIterator {
    pub(crate) flash_string: FlashString,
    pub(crate) counter: u8,
}

impl Iterator for FlashStringIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let is_running = self.counter < self.flash_string.size_N;
        if is_running {
            let byte = unsafe { ProgMem::new(self.flash_string.flash_ptr.add(self.counter as usize)).load() };
            self.counter += 1;
            Some(byte)
        } else {
            None
        }
    }
}
