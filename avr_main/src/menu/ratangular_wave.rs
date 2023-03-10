use crate::microcontroler::timer::now;

type X = u16;

/// Generates a non-synchronous assymetric parametrizable retangular wave form
pub struct RectangularWave {
    up_interval: X,
    down_interval: X,
    next_time_point: X,
    current_state: bool,
}

impl RectangularWave {
    /// interval in miliseconds
    pub fn new(up_interval: X, down_interval: X) -> Self {
        let initial_state = true;
        Self {
            up_interval: up_interval.clone(),
            down_interval,
            next_time_point: now() as X + up_interval as X,
            current_state: initial_state,
        }
    }

    /// Restarts the wave form
    pub fn reset(&mut self) -> &mut Self {
        let initial_state = true;
        self.next_time_point = now() as X + self.up_interval.clone() as X;
        self.current_state = initial_state;
        self
    }

    /// Updates the counter
    ///
    /// You should call this method in a frequency relative to the wave form being generated
    pub fn update(&mut self) -> &mut Self {
        let is_it_moment_to_change_state = now() as X > self.next_time_point;
        if is_it_moment_to_change_state {
            if self.current_state == true {
                // up-down edge
                self.current_state = false;
                self.next_time_point += self.down_interval.clone() as X;
            } else {
                // down-up edge
                self.current_state = true;
                self.next_time_point += self.up_interval.clone() as X;
            }
        } else {
            // no nothing
        }
        self
    }

    /// Reads current wave form state
    ///
    /// Do not forgot that you have to call [`update`] method frequently to calculate the waveform and maintain it up-to-date
    pub fn read(&self) -> bool {
        self.current_state
    }
}
