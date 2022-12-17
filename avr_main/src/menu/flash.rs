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

/* pub enum FindParam {
    Byte(u8),
    Bytes(&[u8]),
} */

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

    /// Search for a sliece of bytes in the [`FlashString`] and returns Some(index_position) or None
    pub fn find_index(&self, pattern: &[char]) -> Option<u8> {
        let mut is_first_run = true;
        let mut index = 0;
        let mut possible_index = 0;
        for (byte, current_index) in self.chars_indices() {
            // if one more byte found.
            if byte as char == pattern[index] {
                // if first run save current index.
                if is_first_run {
                    is_first_run = false;
                    possible_index = current_index;
                }
                index += 1; // points to next byte_to_find.
                            // bound check.
                if index as usize >= pattern.len() {
                    // does match
                    return Some(possible_index);
                }
            } else {
                // false match, so reset
                is_first_run = true;
                possible_index = 0;
                index = 0;
            }
        }
        None
    }
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
