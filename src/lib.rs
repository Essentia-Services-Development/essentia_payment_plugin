//! Essentia Payment Plugin library.

#![allow(dead_code, missing_docs)]
#![allow(clippy::pedantic)]

pub mod errors;
pub mod implementation;
pub mod traits;
pub mod types;
mod flexforge;

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

#[cfg(test)]
mod tests;
