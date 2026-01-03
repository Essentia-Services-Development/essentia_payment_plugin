//! Payment error types.

use core::fmt;

/// Payment operation errors.
#[derive(Debug)]
pub enum PaymentError {
    /// Channel operation error.
    Channel(String),
    /// Invoice error.
    Invoice(String),
    /// Routing error.
    Routing(String),
    /// Insufficient funds.
    InsufficientFunds(String),
    /// Payment timeout.
    Timeout(String),
    /// Configuration error.
    Configuration(String),
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel(msg) => write!(f, "Channel error: {msg}"),
            Self::Invoice(msg) => write!(f, "Invoice error: {msg}"),
            Self::Routing(msg) => write!(f, "Routing error: {msg}"),
            Self::InsufficientFunds(msg) => write!(f, "Insufficient funds: {msg}"),
            Self::Timeout(msg) => write!(f, "Payment timeout: {msg}"),
            Self::Configuration(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

/// Result type for payment operations.
pub type PaymentResult<T> = Result<T, PaymentError>;
