use ffi::*;

pub enum Level {
    Quiet,
    Panic,
    Fatal,
    Error,
    Warning,
    Info,
    Verbose,
}

pub fn set_log_level(lvl: Level) {
    let lvl = match lvl {
        Level::Quiet => AV_LOG_QUIET,
        Level::Panic => AV_LOG_PANIC,
        Level::Fatal => AV_LOG_FATAL,
        Level::Error => AV_LOG_ERROR,
        Level::Warning => AV_LOG_WARNING,
        Level::Info => AV_LOG_INFO,
        Level::Verbose => AV_LOG_VERBOSE,
    };
    unsafe {
        av_log_set_level(lvl);
    }
}
