/// Macro wrapper to log debug message to the specified MEMDB_LOG_TARGET
#[macro_export]
macro_rules! memdb_debug {
    ($($arg:tt)+) => {
        log::debug!(target: MEMDB_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log info message to the specified MEMDB_LOG_TARGET
#[macro_export]
macro_rules! memdb_info {
    ($($arg:tt)+) => {
        log::info!(target: MEMDB_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log warn message to the specified MEMDB_LOG_TARGET
#[macro_export]
macro_rules! memdb_warn {
    ($($arg:tt)+) => {
        log::warn!(target: MEMDB_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log error message to the specified MEMDB_LOG_TARGET
#[macro_export]
macro_rules! memdb_error {
    ($($arg:tt)+) => {
        log::error!(target: MEMDB_LOG_TARGET, $($arg)+);
    };
}
