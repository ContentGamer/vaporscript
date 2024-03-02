use alloc::format;
use crate::logger::color;

pub enum LogLevel {
    Info,
    Warn,
    Error
}

pub fn log(level: LogLevel, message: &str) {
    let current_time = std::time::SystemTime::now();
    let dur_since_epoch = current_time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let seconds = dur_since_epoch.as_secs();

    let hours = ((seconds / 3600) % 24) + 1;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;

    let format = if hours == 0 {
        format!("[12:{}:{} AM]", minutes, seconds)
    } else if hours < 12 {
        format!("[{}:{}:{} AM]", hours, minutes, seconds)
    } else if hours == 12 {
        format!("[{}:{}:{} PM]", hours, minutes, seconds)
    } else {
        format!("[{}:{}:{} PM]", hours-12, minutes, seconds)
    };

    match level {
        LogLevel::Info => {
            println!("\n{}{} {}{}", color::CYAN, format, color::RESET, message);
        }
        LogLevel::Error => {
            println!("\n{}{} {}{}", color::RED, format, color::RESET, message);
            std::process::exit(1);
        }
        LogLevel::Warn => {
            println!("\n{}{} {}{}", color::YELLOW, format, color::RESET, message);
        }
    }
}