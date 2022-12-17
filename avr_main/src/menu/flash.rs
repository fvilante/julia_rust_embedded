use core::str::FromStr;

use alloc::borrow::ToOwned;
use avr_progmem::wrapper::ProgMem;

use avr_progmem::string::PmString;
use heapless::String;

#[derive(Copy, Clone)]
pub struct FlashString {
    flash_ptr: *const u8,
    size_N: u8, // size in quantity of u8's
}

impl FlashString {
    pub fn new<const N: usize>(val: &PmString<N>) -> Self {
        let ptr = val.as_bytes().as_ptr() as *const u8;
        Self {
            flash_ptr: ptr,
            size_N: N as u8,
        }
    }

    pub fn chars_indices(&self) -> FlashStringIterator {
        FlashStringIterator {
            flash_string: self.clone(),
            counter: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size_N as usize
    }

    pub fn to_string<const T: usize>(&self) -> Option<String<T>> {
        let mut s: String<T> = String::new();
        if s.capacity() < self.size_N as usize {
            return None;
        } else {
            for (byte, _index) in self.chars_indices() {
                s.push(byte as char);
            }
            return Some(s.to_owned());
        };
    }

    /*     /// Search for `char` in the [`FlashString`] and returns Some(index_position) or None
    pub fn find_char_index(&self, char_to_find: char) -> Option<u8> {
        for each in self.chars()

    } */
}

pub struct FlashStringIterator {
    flash_string: FlashString,
    counter: u8,
}

type Char = u8;
type CharIndex = u8;

impl Iterator for FlashStringIterator {
    type Item = (Char, CharIndex);

    fn next(&mut self) -> Option<Self::Item> {
        let is_running = self.counter < self.flash_string.size_N;
        if is_running {
            let byte = unsafe {
                // reads next byte from flash
                ProgMem::new(self.flash_string.flash_ptr.add(self.counter as usize)).load()
            };
            let current_index = self.counter;
            let next_index = self.counter + 1;
            let response = (byte, current_index);
            self.counter = next_index; // updates counter index
            Some(response)
        } else {
            None
        }
    }
}
