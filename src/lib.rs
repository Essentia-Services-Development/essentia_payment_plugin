//! # Essentia Payment Plugin
//!
//! Lightning Network and micropayment system integration for the Essentia
//! platform.

#![allow(missing_docs)]
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

// Payment plugin pedantic lint allowances (PAYMENT-LINT-STAGING-01)
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::implicit_clone)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::if_not_else)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::unnested_or_patterns)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::match_bool)]
#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::range_plus_one)]

// EMD Module Structure
pub mod errors;
pub mod implementation;
pub mod traits;
pub mod types;

// FlexForge integration (root level)
mod flexforge;

// Re-exports for convenience
pub use errors::{PaymentError, PaymentResult};
pub use flexforge::PaymentFlexForgeIntegration;
pub use implementation::{
    ChannelManager, InvoiceGenerator, LightningNodeImpl, PaymentConfig, PaymentPlugin,
    PaymentRouter,
};
pub use traits::{ChannelProvider, InvoiceProvider, PaymentProcessor};
pub use types::{
    ChannelState, EscrowStatus, EscrowType, LightningInvoice, LightningNode, PaymentAmount,
    PaymentChannel, PaymentHash, PaymentInvoice, PaymentRoute, PaymentStatus, RouteHop, Satoshis,
    SubscriptionTier, TierFeatures,
};

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = PaymentConfig::default();
        assert!(config.max_channel_capacity > 0);
    }
}

#[cfg(test)]
mod tests;
