use core::ops::Range;


#[derive(Copy, Clone)]
pub struct Cursor {     // size = 3 bytes
    current: usize,     // oscilates between 'range' values
    //range: Range<usize>, //(inclusive-exclusive) 
    start: usize, // min_included
    end: usize, // max_excluded
                         
}

impl Cursor {
    pub const fn new(range: Range<usize>, current: usize) -> Self {
        let range_copy = range.start..range.end;
        let current_normalized = Self::__normalize(range_copy, current);
        Self {
            current: current_normalized,
            start: range.start,
            end: range.end,
        }
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
        let current_normalized = Self::__normalize(self.start..self.end, current_cursor_position);
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