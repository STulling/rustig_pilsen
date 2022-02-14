use colored::*;

enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub fn info(msg: String) {
    log(LogLevel::INFO, msg);
}

pub fn debug(msg: String) {
    log(LogLevel::DEBUG, msg);
}

pub fn warn(msg: String) {
    log(LogLevel::WARN, msg);
}

pub fn error(msg: String) {
    log(LogLevel::ERROR, msg);
}

fn log(level: LogLevel, msg: String) {
    let log = match level {
        LogLevel::DEBUG => format!("[DEBUG] {}", msg).truecolor(100, 100, 100),
        LogLevel::INFO => format!("[INFO] {}", msg).white(),
        LogLevel::WARN => format!("[WARN] {}", msg).yellow(),
        LogLevel::ERROR => format!("[ERROR] {}", msg).red(),
    };
    println!("{}", log);
}