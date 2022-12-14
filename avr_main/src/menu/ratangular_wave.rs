use crate::microcontroler::timer::now;

/// TODO: I do not know why, but to change this type to reduce ram consuption is producing a little bug
/// which is not to blink displayed NumericalField when editing
type X = u64;

/// Generates a non-synchronous assymetric parametrizable retangular wave form
pub struct RectangularWave<T = X> {
    up_interval: T,
    down_interval: T,
    next_time_point: u64,
    current_state: bool,
}

impl RectangularWave<X> {
    /// interval in miliseconds
    pub fn new(up_interval: X, down_interval: X) -> Self {
        let initial_state = true;
        Self {
            up_interval: up_interval.clone(),
            down_interval,
            next_time_point: now() + up_interval as u64,
            current_state: initial_state,
        }
    }

    /// Restarts the wave form
    pub fn reset(&mut self) -> &mut Self {
        let initial_state = true;
        self.next_time_point = now() + self.up_interval.clone() as u64;
        self.current_state = initial_state;
        self
    }

    /// Updates the counter
    ///
    /// You should call this method in a frequency relative to the wave form being generated
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

    /// Reads current wave form state
    ///
    /// Do not forgot that you have to call [`update`] method frequently to calculate the waveform and maintain it up-to-date
    pub fn read(&self) -> bool {
        self.current_state
    }
}
