use std::time::{Instant, Duration};

pub struct TimeUtils {
    last_frame: Instant,
    delta_s : f32,
    frame_duration : Duration,
    time : f32,
}

impl TimeUtils {
    pub fn new() -> TimeUtils {
        TimeUtils {
            last_frame: Instant::now(),
            delta_s : 0.0,
            frame_duration : Duration::new(0, 0),
            time : 0.0,
        }
    }

    pub fn update(&mut self) {
        let delta_s = self.last_frame.elapsed();
        self.time += delta_s.as_secs() as f32 + delta_s.subsec_nanos() as f32 * 1e-9;
        self.frame_duration = delta_s; 
        self.delta_s = delta_s.as_secs() as f32 + delta_s.subsec_nanos() as f32 * 1e-9;
        self.last_frame = Instant::now();
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.delta_s
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }


}