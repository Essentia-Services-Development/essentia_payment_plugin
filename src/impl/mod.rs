//! Payment Plugin implementations.
//!
//! This module contains all implementations for the Payment plugin:
//! - `PaymentConfig` - Configuration
//! - `ChannelManager` - Lightning channel management
//! - `InvoiceGenerator` - Invoice creation and verification
//! - `PaymentRouter` - Payment routing
//! - `PaymentPlugin` - Main plugin interface

mod channels;
mod config;
mod invoices;
mod lightning;
mod plugin;
mod router;

pub use channels::ChannelManager;
pub use config::PaymentConfig;
pub use invoices::InvoiceGenerator;
pub use lightning::LightningNodeImpl;
pub use plugin::PaymentPlugin;
pub use router::PaymentRouter;
