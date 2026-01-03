//! Payment Plugin trait definitions.
//!
//! This module contains all trait definitions for the Payment plugin.

mod core;

pub use core::{ChannelProvider, InvoiceProvider, PaymentProcessor};
