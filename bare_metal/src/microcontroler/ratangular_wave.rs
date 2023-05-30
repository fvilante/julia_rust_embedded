//! Utility to generates asynchronous assymetric parametrizable retangular wave form

use crate::microcontroler::timer::now;

type TimePoint = u64;
type Duration = u16;

/// Generates a non-synchronous assymetric parametrizable retangular wave form
pub struct RectangularWave {
    up_interval: Duration,
    down_interval: Duration,
    next_time_point: TimePoint,
    current_state: bool,
}

impl RectangularWave {
    /// interval in miliseconds
    pub fn new(up_interval: Duration, down_interval: Duration) -> Self {
        let initial_state = true;
        let next_time_point = now() as TimePoint + up_interval as TimePoint;
        Self {
            up_interval,
            down_interval,
            next_time_point,
            current_state: initial_state,
        }
    }

    /// Updates the counter
    ///
    /// You should call this method in a frequency relative to the wave form being generated
    pub fn update(&mut self) {
        let is_time_to_change_state = now() as TimePoint > self.next_time_point;
        if is_time_to_change_state {
            // toggle state
            self.current_state = !self.current_state;
            // calc next time point
            let delta: TimePoint = if self.current_state {
                self.up_interval
            } else {
                self.down_interval
            }
            .into();
            self.next_time_point += delta;
        }
    }

    /// Reads current wave form state
    ///
    /// Do not forgot that you have to call [`update`] method frequently to calculate the waveform and maintain it up-to-date
    pub fn read(&self) -> bool {
        self.current_state
    }
}
