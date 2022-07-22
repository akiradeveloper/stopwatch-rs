use quanta::Instant;
use std::time::Duration;

enum State {
    Running {
        lap_start_time: Instant,
    },
    Stopped {
        lap_start_time: Instant,
        suspend_time: Instant,
    },
}
use State::*;

pub struct StopWatch {
    start_time: Instant,
    state: State,
    cur_suspend: Duration,
    total_suspend: Duration,
}
impl StopWatch {
    pub fn start() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            state: Running {
                lap_start_time: now,
            },
            cur_suspend: Duration::new(0, 0),
            total_suspend: Duration::new(0, 0),
        }
    }
    pub fn suspend(&mut self) {
        if let Running {
            lap_start_time: start_time,
        } = self.state
        {
            let now = Instant::now();
            self.state = Stopped {
                lap_start_time: start_time,
                suspend_time: now,
            };
        }
    }
    pub fn resume(&mut self) {
        if let Stopped {
            lap_start_time: start_time,
            suspend_time,
        } = self.state
        {
            let now = Instant::now();
            let suspend_time = now.duration_since(suspend_time);
            self.cur_suspend += suspend_time;
            self.total_suspend += suspend_time;
            self.state = Running {
                lap_start_time: start_time,
            }
        }
    }
    pub fn split(&mut self) -> Split {
        match self.state {
            State::Running {
                lap_start_time: start_time,
            } => {
                let now = Instant::now();
                let lap = now.duration_since(start_time) - self.cur_suspend;
                let split = now.duration_since(self.start_time) - self.total_suspend;
                self.state = Running {
                    lap_start_time: now,
                };
                self.cur_suspend = Duration::new(0, 0);
                Split { split, lap }
            }
            State::Stopped {
                lap_start_time: start_time,
                suspend_time,
            } => {
                let lap = suspend_time.duration_since(start_time) - self.cur_suspend;
                let split = suspend_time.duration_since(self.start_time) - self.total_suspend;
                Split { split, lap }
            }
        }
    }
}
pub struct Split {
    pub split: Duration,
    pub lap: Duration,
}
impl std::fmt::Display for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lap={:?}, split={:?}", self.lap, self.split)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::*;
    #[test]
    fn test() {
        let mut sw = StopWatch::start();
        sleep(Duration::from_secs(1));
        println!("{}", sw.split());

        sw.suspend();
        sleep(Duration::from_secs(2));
        sw.resume();

        sw.suspend();
        sleep(Duration::from_secs(3));
        sw.resume();

        sleep(Duration::from_secs(4));
        println!("{}", sw.split());

        sw.suspend();
        sleep(Duration::from_secs(1));
        println!("{}", sw.split());
    }

    #[test]
    fn test_suspend() {
        let mut sw = StopWatch::start();
        sleep(Duration::from_secs(1));
        sw.suspend();
        sleep(Duration::from_secs(2));
        println!("{}", sw.split());
    }
}
