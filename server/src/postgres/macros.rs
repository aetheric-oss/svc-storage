#[macro_export]
/// Macro wrapper to log debug message to the specified PSQL_LOG_TARGET
macro_rules! psql_debug {
    ($($arg:tt)+) => {
        log::debug!(target: PSQL_LOG_TARGET, $($arg)+);
    };
}

#[macro_export]
/// Macro wrapper to log info message to the specified PSQL_LOG_TARGET
macro_rules! psql_info {
    ($($arg:tt)+) => {
        log::info!(target: PSQL_LOG_TARGET, $($arg)+);
    };
}

#[macro_export]
/// Macro wrapper to log warn message to the specified PSQL_LOG_TARGET
macro_rules! psql_warn {
    ($($arg:tt)+) => {
        log::warn!(target: PSQL_LOG_TARGET, $($arg)+);
    };
}

#[macro_export]
/// Macro wrapper to log error message to the specified PSQL_LOG_TARGET
macro_rules! psql_error {
    ($($arg:tt)+) => {
        log::error!(target: PSQL_LOG_TARGET, $($arg)+);
    };
}
