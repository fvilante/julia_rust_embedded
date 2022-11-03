use crate::microcontroler::timer::now;


pub struct RectangularWave<T> {
    up_interval: T,
    down_interval: T,
    next_interval: T,
    current_state: bool,
}


impl RectangularWave<u32>  {
    /// interval in miliseconds
    pub fn new(up_interval: u32, down_interval: u32) -> Self {
        Self {  
            up_interval: up_interval.clone(),
            down_interval,
            next_interval: Self::__now() + up_interval,
            current_state: true,
        }   
    }

    fn __now() -> u32  {
        let value = now();
        if value > core::u32::MAX as u64 {
            0
        } else {
            value as u32
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.next_interval = Self::__now() + self.up_interval.clone();
        self.current_state = true;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        let is_it_moment_to_change_level = Self::__now() > self.next_interval;
        if is_it_moment_to_change_level {
            if self.current_state == true {
                // up-down edge
                self.current_state = false;
                self.next_interval += self.down_interval.clone(); 
            } else {
                // down-up edge
                self.current_state = true;
                self.next_interval += self.up_interval.clone(); 
            }
        } else {
            // no nothing
        }
        self
    }

    pub fn read(& self) -> bool {
        self.current_state
    }

}
