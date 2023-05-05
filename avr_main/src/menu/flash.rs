use core::mem::size_of;
use core::ops::Range;

//use alloc::borrow::ToOwned;
use avr_progmem::progmem;
use avr_progmem::wrapper::ProgMem;

use avr_progmem::string::PmString;
//use heapless::String;

progmem! {
    static progmem string DEFAULT_FLASH_STRING = "";
}

#[derive(Copy, Clone)]
pub struct FlashString {
    /// This pointer MUST NEVER be dereferenced because it represents a pointer to Flash DOMAIN, but rust does not
    /// understand it natively. Use the metods in this class to work with this pointer instead of using the pointer
    /// directly (except if you know what are you doing)
    flash_ptr: *const u8,
    /// Number of characters in the flash_ptr. If zero, than the string is considered empty
    length: u8,
}

impl Default for FlashString {
    fn default() -> Self {
        FlashString::new(&DEFAULT_FLASH_STRING)
    }
}

/* pub enum FindParam {
    Byte(u8),
    Bytes(&[u8]),
} */

impl FlashString {
    pub fn from_raw(flash_ptr: *const u8, length: u8) -> Self {
        Self { flash_ptr, length }
    }

    /// Creates a new [`FlashString`].
    pub fn new<const N: usize>(val: &PmString<N>) -> Self {
        let ptr = val.as_bytes().as_ptr() as *const u8;
        Self {
            flash_ptr: ptr,
            length: N as u8,
        }
    }

    /// Returns the chars indices of this [`FlashString`].
    /// returns (Char, CharIndex)
    pub fn chars_indices(&self) -> FlashStringIterator {
        FlashStringIterator {
            flash_string: self.clone(),
            counter: 0,
        }
    }

    /// Returns the length of this [`FlashString`].
    pub fn len(&self) -> u8 {
        self.length
    }

    // pub fn to_string<const T: usize>(&self) -> Option<String<T>> {
    //     let mut s: String<T> = String::new();
    //     if s.capacity() < self.length as usize {
    //         return None;
    //     } else {
    //         for (byte, _index) in self.chars_indices() {
    //             s.push(byte as char);
    //         }
    //         return Some(s.to_owned());
    //     };
    // }

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
        let first = self.sub_string(0..mid_index);
        let second = self.sub_string(mid_index..self.len());
        (first, second)
    }

    /// Returns a substring based on given range.
    ///
    /// The range refers to the string index, where range.start (incluive) and range.end (exclusive).
    /// If range exceeds the string size, than a clamp is applied.
    pub fn sub_string(&self, range: Range<u8>) -> FlashString {
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

    /// returns (Char, CharIndex)
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

/// Makes an array slice from data in Flash memory
#[derive(Copy, Clone)]
pub struct FlashSlice<T> {
    /// This pointer MUST NEVER be dereferenced because it represents a pointer to Flash DOMAIN, but rust does not
    /// understand it natively. Is the metods in this class to work with this pointer instead of using the pointer
    /// directly (except if you know what are you doing)
    flash_ptr: *const T,
    /// Number of characters in the flash_ptr. If zero, than the string is considered empty
    length: u8,
}

impl<T: Copy> FlashSlice<T> {
    /// Creates a new [`FlashSlice<T>`].
    pub fn new<const N: usize>(val: &ProgMem<[T; N]>) -> Self {
        let ptr = val.as_ptr() as *const T;
        Self {
            flash_ptr: ptr,
            length: N as u8,
        }
    }

    pub fn len(&self) -> u8 {
        self.length
    }

    pub fn get_by_index(&self, index: u8) -> Option<T> {
        if index < self.length {
            let pointer = self.flash_ptr;
            let size = size_of::<T>();
            let pointer_offset = unsafe { pointer.add(size * index as usize) };
            let reference = unsafe { ProgMem::new(pointer_offset) };
            let data = reference.load();
            Some(data)
        } else {
            None
        }
    }

    pub fn to_iterator(&self) -> FlashSliceIterator<T> {
        FlashSliceIterator {
            flash_slice: self.clone(),
            counter_index: 0,
        }
    }
}

pub struct FlashSliceIterator<T> {
    flash_slice: FlashSlice<T>,
    counter_index: u8,
}

impl<T: Copy> Iterator for FlashSliceIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.flash_slice.get_by_index(self.counter_index);
        self.counter_index += 1;
        item
    }
}
