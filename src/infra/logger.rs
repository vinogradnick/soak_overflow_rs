use std::time::{SystemTime, UNIX_EPOCH};

pub const VERBOSE_LOGS: bool = true;

pub fn log<T>(value: &T, func: &str)
where
    T: std::fmt::Debug + ?Sized,
{
    if !VERBOSE_LOGS {
        return;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let total_seconds = now.as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    let millis = now.subsec_millis();

    println!(
        "[{:02}:{:02}:{:02}:{:03}] [DEBUG] [{}] {:?}",
        hours, minutes, seconds, millis, func, value
    );
}

pub fn log_str<S: AsRef<str>>(value: S, func: &str)
where
    S: std::fmt::Debug,
{
    if !VERBOSE_LOGS {
        return;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let total_seconds = now.as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    let millis = now.subsec_millis();

    println!(
        "[{:02}:{:02}:{:02}:{:03}] [DEBUG] [{}] {:?}",
        hours, minutes, seconds, millis, func, value
    );
}
