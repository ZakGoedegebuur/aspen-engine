use std::time::Instant;

pub struct TimingStruct {
    pub begin_time: Instant,
    pub prev_time: Instant,
    pub current_time: Instant,
    pub cumulative: f64,
}

impl TimingStruct {
    pub fn new() -> TimingStruct {
        TimingStruct {
            begin_time: Instant::now(),
            prev_time: Instant::now(),
            current_time: Instant::now(),
            cumulative: 0.0
        }
    }

    /// Updates self and returns info
    pub fn update(&mut self, fixed_rate: u16) -> UpdateTimes {
        self.prev_time = self.current_time;
        self.current_time = Instant::now();
        let delta = self.current_time.duration_since(self.prev_time).as_secs_f64();
        self.cumulative += delta;

        let fixed_delta = 1.0 / fixed_rate as f64;
        let fixed_steps = (self.cumulative / fixed_delta) as u64;
        self.cumulative %= fixed_delta;

        UpdateTimes {
            delta,
            fixed_delta,
            fixed_steps
        }
    }
}

pub struct UpdateTimes {
    pub delta: f64,
    pub fixed_delta: f64,
    pub fixed_steps: u64
}