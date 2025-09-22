use std::time::{SystemTime, UNIX_EPOCH};

pub const VERBOSE_LOGGS: bool = true;

pub fn log<T>(value: &T, func: &str)
where
    T: std::fmt::Debug,
{
    if !VERBOSE_LOGGS {
        return;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!(
        "[{}.{:03}] [{}] [{}] {:?}",
        now.as_secs(),
        now.subsec_millis(),
        "DEBUG",
        func,
        value
    );
}
