//! Payment Plugin type definitions.
//!
//! This module contains all type definitions for the Payment plugin:
//! - Payment channel and state types
//! - Invoice and routing types
//! - Amount representations

mod core;

pub use core::{
    ChannelState, PaymentAmount, PaymentChannel, PaymentInvoice, PaymentRoute, PaymentStatus,
    RouteHop,
};
