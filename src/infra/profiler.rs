use std::time::{Duration, Instant};

pub struct Profiler {
    start: Instant,
    name: String,
}

impl Profiler {
    pub fn record(name: &str) -> Self {
        let p = Profiler {
            start: Instant::now(),
            name: name.to_string(),
        };
        p
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }
}

// Drop автоматически выведет время при выходе из области видимости
impl Drop for Profiler {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        println!(
            "[PROFILER] [{}] elapsed: {}.{:03} ms",
            self.name,
            elapsed.as_millis(),
            elapsed.subsec_micros() % 1000
        );
    }
}

// // Пример использования
// fn example_function() {
//     let _prof = Profiler::new("example_function");

//     // Код, который хотим профилировать
//     std::thread::sleep(std::time::Duration::from_millis(123));
// }

// fn main() {
//     example_function();
// }
