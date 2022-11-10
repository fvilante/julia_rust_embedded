use crate::microcontroler::timer::now;


/// Generates a non-synchronous assymetric parametrizable retangular wave form
pub struct RectangularWave<T = u16> {
    up_interval: T,         // size = 2 bytes
    down_interval: T,       // size = 2 bytes
    next_time_point: u64,   // size = 8 bytes
    current_state: bool,    // size = 1 bytes
                            // TOTAL = 13 bytes
}

impl RectangularWave<u16>  {
    /// interval in miliseconds
    pub fn new(up_interval: u16, down_interval: u16) -> Self {
        let initial_state = true;
        Self {  
            up_interval: up_interval.clone(),
            down_interval,
            next_time_point: now() + up_interval as u64,
            current_state: initial_state,
        }   
    }

    pub fn reset(&mut self) -> &mut Self {
        let initial_state = true;
        self.next_time_point = now() + self.up_interval.clone() as u64;
        self.current_state = initial_state;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        let is_it_moment_to_change_state = now() > self.next_time_point;
        if is_it_moment_to_change_state {
            if self.current_state == true {
                // up-down edge
                self.current_state = false;
                self.next_time_point += self.down_interval.clone() as u64; 
            } else {
                // down-up edge
                self.current_state = true;
                self.next_time_point += self.up_interval.clone() as u64; 
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
