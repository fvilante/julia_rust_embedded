use core::ops::Range;

use super::common::{const_clamp, usize_to_u8_clamper};

/// The purpose of a [`Cursor`] is to statefully move a current unsigned integer around a predefined interval of values
/// normally defined between start (inclusive) and end (exclusive). Through a serie of defined methods.
///
#[derive(Copy, Clone)]
pub struct Cursor {
    /// varies between start (inclusive) and end (exclusive)
    current: u8,
    /// included
    start: u8,
    /// excluded
    end: u8,
}

impl Cursor {
    /// Constructs new instance.
    ///
    /// The `start` and `end` parameters represents the interval of valid values, consider also the following convention:
    /// -   `start` (included),
    /// -   `end` (excluded),
    /// -   `initial_value` (included).
    pub const fn new(start: u8, end: u8, initial_value: u8) -> Self {
        Self {
            current: Self::normalize_current(initial_value, start, end),
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

    /// Value returned is already normalized, that means that it will never be out of the defined interval
    pub fn get_current(&self) -> u8 {
        // value already normalized
        self.current
    }

    /// Sets current cursor position
    ///
    /// Note that if proposed `cursor_position` is outside the valid range its value is clamped
    pub fn set_current(&mut self, cursor_position: u8) {
        let clamped_cursor_position = cursor_position;
        let normalized_current =
            Self::normalize_current(clamped_cursor_position, self.start, self.end);
        self.current = normalized_current;
    }

    pub fn get_range(&self) -> Range<u8> {
        self.start..self.end
    }

    /// returns true if has reached the upper bound
    pub fn next(&mut self) -> bool {
        StatelessCursor::next(self.end, &mut self.current)
    }

    /// returns true if has reached the lower bound
    pub fn previous(&mut self) -> bool {
        StatelessCursor::previous(self.start, &mut self.current)
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

/// ATTENTION: unsafe code node marked
/// TODO: if `start`>0 then this is unsafe because is not garanteed to have at least two objects to point, but i'm
/// for short to develop menu system. When possible improve this code
impl Default for Cursor {
    fn default() -> Self {
        Self {
            current: 0,
            start: 0,
            end: 2,
        }
    }
}

/////////////////////////////////////////////////

/// Stateless Cursor for low memory consumption strategies.
///
/// TODO: In future check if this type or the type [`Cursor`] cannot be substituted for a bidirectional iterator trait
/// or a Indexable (Array-like) trait.
pub struct StatelessCursor;

impl StatelessCursor {
    /// if current_index is updated, then returns bool.
    /// `end` is equal the last index plus one, in other words: it is exclusive
    pub fn next(end: u8, current_index: &mut u8) -> bool {
        let last_index = end - 1;
        let has_reached_upper_bound = *current_index >= last_index;
        if has_reached_upper_bound == false {
            *current_index += 1;
        }
        has_reached_upper_bound
    }

    /// `start` is inclusive
    pub fn previous(start: u8, current_index: &mut u8) -> bool {
        let first_index = start;
        let has_reached_lower_bound = *current_index <= first_index;
        if has_reached_lower_bound == false {
            *current_index -= 1;
        }
        has_reached_lower_bound
    }
}
