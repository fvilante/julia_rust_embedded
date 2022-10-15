use crate::microcontroler::timer::now;


pub struct RectangularWave {
    up_interval: u64,
    down_interval: u64,
    next_interval: u64,
    current_state: bool,
}

impl RectangularWave {
    /// interval in miliseconds
    pub fn new(up_interval: u64, down_interval: u64) -> Self {
        Self {  
            up_interval,
            down_interval,
            next_interval: now() + up_interval,
            current_state: true,
        }   
    }

    pub fn reset(&mut self) -> &mut Self {
        self.next_interval = now() + self.up_interval;
        self.current_state = true;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        let is_it_moment_to_change_level = now() > self.next_interval;
        if is_it_moment_to_change_level {
            if self.current_state == true {
                // up-down edge
                self.current_state = false;
                self.next_interval += self.down_interval; 
            } else {
                // down-up edge
                self.current_state = true;
                self.next_interval += self.up_interval; 
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
