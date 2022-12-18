use core::ops::Range;
use core::str::FromStr;

use alloc::borrow::ToOwned;
use avr_progmem::wrapper::ProgMem;

use avr_progmem::string::PmString;
use heapless::String;

#[derive(Copy, Clone)]
pub struct FlashString {
    /// This pointer MUST NEVER be dereferenced because it represents a pointer to Flash DOMAIN, but rust does not
    /// understand it natively. Is the metods in this class to work with this pointer instead of using the pointer
    /// directly (except if you know what are you doing)
    flash_ptr: *const u8,
    /// Number of characters in the flash_ptr. If zero, than the string is considered empty
    length: u8,
}

/* pub enum FindParam {
    Byte(u8),
    Bytes(&[u8]),
} */

impl FlashString {
    pub fn from_raw(flash_ptr: *const u8, length: u8) -> Self {
        Self { flash_ptr, length }
    }

    pub fn new<const N: usize>(val: &PmString<N>) -> Self {
        let ptr = val.as_bytes().as_ptr() as *const u8;
        Self {
            flash_ptr: ptr,
            length: N as u8,
        }
    }

    pub fn chars_indices(&self) -> FlashStringIterator {
        FlashStringIterator {
            flash_string: self.clone(),
            counter: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.length
    }

    pub fn to_string<const T: usize>(&self) -> Option<String<T>> {
        let mut s: String<T> = String::new();
        if s.capacity() < self.length as usize {
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

    /// Splits the string in two parts by a given index.
    ///
    /// Tries to imitate behaviour of heapless::String::split_at
    pub fn split_at(&self, mid_index: u8) -> (FlashString, FlashString) {
        let first = self.get_from_range(0..mid_index);
        let second = self.get_from_range(mid_index..self.len());
        (first, second)
    }

    /// Returns a substring based on given range.
    ///
    /// The range refers to the string index, where range.start (incluive) and range.end (exclusive).
    /// If range exceeds the string size, than a clamp is applied.
    pub fn get_from_range(&self, range: Range<u8>) -> FlashString {
        let first_possible_index = 0;
        let last_possible_index = self.len();
        let index_start = range.start.clamp(first_possible_index, last_possible_index);
        let index_end = range.end.clamp(first_possible_index, last_possible_index);
        let length = index_end - index_start;
        let new_start_address = unsafe { self.flash_ptr.add(index_start as usize) };
        FlashString::from_raw(new_start_address, length)
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
        let is_running = self.counter < self.flash_string.length;
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
