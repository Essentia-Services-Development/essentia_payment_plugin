//! # Essentia Payment Plugin
//!
//! Lightning Network and micropayment system integration for the Essentia
//! platform.
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

mod channels;
mod config;
mod errors;
mod invoices;
mod plugin;
mod router;
mod types;

pub use channels::ChannelManager;
pub use config::PaymentConfig;
pub use errors::{PaymentError, PaymentResult};
pub use invoices::InvoiceGenerator;
pub use plugin::PaymentPlugin;
pub use router::PaymentRouter;
pub use types::{
    ChannelState, PaymentAmount, PaymentChannel, PaymentInvoice, PaymentRoute, PaymentStatus,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = PaymentConfig::default();
        assert!(config.max_channel_capacity > 0);
    }
}
