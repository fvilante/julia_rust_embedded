use core::ops::Range;


pub struct Cursor {
    current: usize,     // oscilates between 'range' values
    range: Range<usize>, //(inclusive-exclusive) 
}

impl Cursor {
    pub const fn new(range: Range<usize>, current: usize) -> Self {
        let range_copy = range.start..range.end;
        let current_normalized = Self::__normalize(range_copy, current);
        Self {
            current: current_normalized,
            range,
        }
    }

    pub fn clone(&self) -> Self {
        //NOTE: By default 'Range' type is not copy. For more see: https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
        let current = self.current;
        let range = self.range.start..self.range.end;
        let copied_cursor = Cursor::new(range, current);
        copied_cursor
    }

    /// normalize given cursor position to make sure it is inside valid range
    const fn __normalize(range: Range<usize>, unsafe_cursor: usize) -> usize {
        let min = range.start;
        let max = range.end-1;
        if unsafe_cursor < min {
            min
        } else if unsafe_cursor > max {
            max
        } else {
            unsafe_cursor
        }
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
        self.current = self.range.end-1;
    }
    
    pub fn begin(&mut self) {
        self.current = self.range.start;
    }
}