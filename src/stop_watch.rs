use std::time::Instant;

pub struct StopWatch {
    instant: Instant,
    elapsed_s: f64
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            instant: Instant::now(),
            elapsed_s: 0.0,
        }
    }

    pub fn start(&mut self) {
        let now = Instant::now();
        self.instant = now;
    }

    pub fn stop(&mut self) {
        let elapsed = self.instant.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        self.elapsed_s = sec;
    }

    pub fn print(&self) {
        println!("Seconds: {}", self.elapsed_s);
    }
}