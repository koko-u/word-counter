//! application errors

/// structure that aggregates errors encountered by the application.
#[derive(Debug, derive_more::Display)]
#[display(fmt = "Application Error")]
pub struct AppError;

impl error_stack::Context for AppError {}
