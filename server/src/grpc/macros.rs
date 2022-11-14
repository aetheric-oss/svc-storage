/// Macro wrapper to log debug message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_debug {
    ($($arg:tt)+) => {
        log::debug!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log info message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_info {
    ($($arg:tt)+) => {
        log::info!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log warn message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_warn {
    ($($arg:tt)+) => {
        log::warn!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log error message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_error {
    ($($arg:tt)+) => {
        log::error!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}
