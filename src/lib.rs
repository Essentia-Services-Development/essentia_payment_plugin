//! # Essentia Payment Plugin
//!
//! Lightning Network and micropayment system integration for the Essentia platform.
//!
//! ## Features
//!
//! - Lightning Network channel management
//! - Micropayment processing
//! - Payment routing
//! - Invoice generation and verification
//! - Quantum-safe payment channels (via `essentia_pqc`)
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Payment Plugin                            │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │  Lightning  │  │   Invoice   │  │   Payment Router    │  │
//! │  │   Channels  │  │  Generator  │  │                     │  │
//! │  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
//! │         │                │                     │             │
//! │         ▼                ▼                     ▼             │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │              Payment Processing Engine               │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```

mod types;
mod errors;
mod config;
mod channels;
mod invoices;
mod router;
mod plugin;

pub use types::{
    PaymentChannel, PaymentInvoice, PaymentRoute, PaymentStatus,
    ChannelState, PaymentAmount,
};
pub use errors::{PaymentError, PaymentResult};
pub use config::PaymentConfig;
pub use channels::ChannelManager;
pub use invoices::InvoiceGenerator;
pub use router::PaymentRouter;
pub use plugin::PaymentPlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = PaymentConfig::default();
        assert!(config.max_channel_capacity > 0);
    }
}
