use core::ops::Range;

pub struct Cursor {
    current: usize,
    range: Range<usize>, //(inclusive-exclusive)
}

impl Cursor {
    pub fn new(range: Range<usize>, current: usize) -> Self {
        let current_normalized = Self::__normalize(range.clone(), current);
        Self {
            current: current_normalized,
            range,
        }
    }

    /// normalize given cursor position to make sure it is inside valid range
    fn __normalize(range: Range<usize>, unsafe_cursor: usize) -> usize {
        let min = range.start;
        let max = range.end-1;
        unsafe_cursor.clamp(min, max)
    }

    pub fn get_current(&self) -> usize {
        self.current // value already normalized
    }

    // sets current cursor position
    pub fn set_current(&mut self, current_cursor_position: usize) {
        let current_normalized = Self::__normalize(self.range.clone(), current_cursor_position);
        self.current = current_normalized;

    }

    /// returns true if has reached the upper bound
    pub fn next(&mut self) -> bool {
        let last_index = self.range.end-1;
        let current_index = self.current;
        let has_reached_upper_bound = current_index >= last_index;
        if has_reached_upper_bound == false  {
            self.current += 1;
        }
        has_reached_upper_bound
    }

    /// returns true if has reached the lower bound
    pub fn previous(&mut self) -> bool {
        let first_index = self.range.start;
        let current_index = self.current;
        let has_reached_lower_bound = current_index <= first_index;
        if has_reached_lower_bound == false {
            self.current -= 1;
        }
        has_reached_lower_bound
    }

    pub fn end(&mut self) {
        self.current = self.range.end;
    }
    
    pub fn begin(&mut self) {
        self.current = self.range.start;
    }
}