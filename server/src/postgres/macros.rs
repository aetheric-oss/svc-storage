//! log macro's for PostgreSQL logging

/// Writes a debug! message to the app::backend::psql logger
#[macro_export]
macro_rules! psql_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::backend::psql", $($arg)+);
    };
}

/// Writes a info! message to the app::backend::psql logger
#[macro_export]
macro_rules! psql_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::backend::psql", $($arg)+);
    };
}

/// Writes a warn! message to the app::backend::psql logger
#[macro_export]
macro_rules! psql_warn {
    ($($arg:tt)+) => {
        log::warn!(target: "app::backend::psql", $($arg)+);
    };
}

/// Writes a error! message to the app::backend::psql logger
#[macro_export]
macro_rules! psql_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::backend::psql", $($arg)+);
    };
}
