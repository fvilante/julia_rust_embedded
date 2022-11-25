use core::ops::Range;

use lib_1::utils::common::usize_to_u8_clamper;

/// TODO: Make Cursor<T=u8>
#[derive(Copy, Clone)]
pub struct Cursor {     // size = 3 bytes
    current: u8,     // oscilates between 'range' values
    //range: Range<usize>, //(inclusive-exclusive) 
    start: u8, // min_included
    end: u8, // max_excluded
                         
}

impl Cursor {
    pub const fn new(range: Range<usize>, current: usize) -> Self {
        let range_copy = range.start..range.end;
        let current_normalized = Self::__normalize_current(range_copy, current);
        Self {
            current: current_normalized,
            start: Self::__cast_usize_to_u8(range.start),
            end: Self::__cast_usize_to_u8(range.end),
        }
    }

    ///TODO: Improve safety by making this function unnecessary, using a generic type T over Cursor<T> type
    const fn __cast_usize_to_u8(value: usize) -> u8 {
        usize_to_u8_clamper(value)
    }

    /// normalize given cursor position to make sure it is inside valid range, also converts it to u8 (compact) format
    const fn __normalize_current(range: Range<usize>, unsafe_cursor_: usize) -> u8 {
        let min = Self::__cast_usize_to_u8(range.start);
        let max = Self::__cast_usize_to_u8(range.end-1);
        let unsafe_cursor = Self::__cast_usize_to_u8(unsafe_cursor_);
        if unsafe_cursor < min {
            min
        } else if unsafe_cursor > max {
            max
        } else {
            let safe_cursor = unsafe_cursor;
            safe_cursor
        }
    }

    pub fn get_current(&self) -> usize {
        self.current as usize// value already normalized
    }

    // sets current cursor position
    pub fn set_current(&mut self, current_cursor_position: usize) {
        let range = self.start as usize..self.end as usize;
        let current_normalized = Self::__normalize_current(range, current_cursor_position);
        self.current = current_normalized;

    }



    /// returns true if has reached the upper bound
    pub fn next(&mut self) -> bool {
        let last_index = self.end-1;
        let current_index = self.current;
        let has_reached_upper_bound = current_index >= last_index;
        if has_reached_upper_bound == false  {
            self.current += 1;
        }
        has_reached_upper_bound
    }

    /// returns true if has reached the lower bound
    pub fn previous(&mut self) -> bool {
        let first_index = self.start;
        let current_index = self.current;
        let has_reached_lower_bound = current_index <= first_index;
        if has_reached_lower_bound == false {
            self.current -= 1;
        }
        has_reached_lower_bound
    }

    pub fn next_wrap_around(&mut self) {
        let has_finished = self.next();
        if has_finished {
            self.begin();
        } 
    }
    
    pub fn previous_wrap_around(&mut self) {
        let has_finished = self.previous();
        if has_finished {
            self.end();
        }
    }

    pub fn end(&mut self) {
        self.current = self.end-1;
    }
    
    pub fn begin(&mut self) {
        self.current = self.start;
    }
}