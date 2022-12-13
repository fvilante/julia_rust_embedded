use core::ops::Range;

use super::common::{const_clamp, usize_to_u8_clamper};

/// Stateful Cursor that may oscilates between start (inclusive) and end (exclusive)
#[derive(Copy, Clone)]
pub struct Cursor {
    // size = 3 bytes
    current: u8, // oscilates between start (inclusive) and end (exclusive)
    start: u8,   // included
    end: u8,     // excluded
}

impl Cursor {
    pub const fn new(start: u8, end: u8, current: u8) -> Self {
        Self {
            current: Self::normalize_current(current, start, end),
            start,
            end,
        }
    }

    // Ensures current is inside start (inclusive) and end (exclusive) range
    const fn normalize_current(current: u8, start: u8, end: u8) -> u8 {
        const_clamp(current, start, end - 1)
    }

    pub const fn from_range(range: Range<usize>, current: u8) -> Self {
        let start = usize_to_u8_clamper(range.start);
        let end = usize_to_u8_clamper(range.end);
        Self::new(start, end, current)
    }

    pub fn get_current(&self) -> usize {
        self.current as usize // value already normalized
    }

    /// returns true if has reached the upper bound
    pub fn next(&mut self) -> bool {
        let last_index = self.end - 1;
        let current_index = self.current;
        let has_reached_upper_bound = current_index >= last_index;
        if has_reached_upper_bound == false {
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
        self.current = self.end - 1;
    }

    pub fn begin(&mut self) {
        self.current = self.start;
    }
}
