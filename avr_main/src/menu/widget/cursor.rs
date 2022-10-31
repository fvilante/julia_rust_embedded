use core::ops::Range;

pub struct Cursor {
    current: usize,
    range: Range<usize>, //(inclusive-exclusive)
}

impl Cursor {
    pub fn new(range: Range<usize>) -> Self {
        Self {
            current: 0,
            range,
        }
    }

    pub fn get_current(&self) -> usize {
        self.current
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