//! Core payment traits.

use crate::{
    errors::PaymentResult,
    types::{PaymentChannel, PaymentInvoice, PaymentRoute, PaymentStatus},
};

/// Trait for channel management providers.
pub trait ChannelProvider: Send + Sync {
    /// Get all channels.
    fn channels(&self) -> &[PaymentChannel];

    /// Get active channels only.
    fn active_channels(&self) -> Vec<&PaymentChannel>;

    /// Get total local balance across all active channels.
    fn total_local_balance(&self) -> u64;

    /// Open a new channel with a peer.
    fn open_channel(&mut self, peer_pubkey: [u8; 33], capacity: u64) -> PaymentResult<[u8; 32]>;

    /// Close a channel cooperatively.
    fn close_channel(&mut self, channel_id: &[u8; 32]) -> PaymentResult<()>;
}

/// Trait for invoice generation and verification.
pub trait InvoiceProvider: Send + Sync {
    /// Generate a new invoice.
    fn generate_invoice(
        &self, amount: Option<u64>, description: &str,
    ) -> PaymentResult<PaymentInvoice>;

    /// Verify an invoice is valid and not expired.
    fn verify_invoice(&self, invoice: &PaymentInvoice) -> PaymentResult<bool>;
}

/// Trait for payment processing.
pub trait PaymentProcessor: Send + Sync {
    /// Find a route to the destination.
    fn find_route(&self, destination: &[u8; 33], amount_msat: u64) -> PaymentResult<PaymentRoute>;

    /// Send a payment using the provided invoice.
    fn send_payment(&self, invoice: &PaymentInvoice) -> PaymentResult<PaymentStatus>;

    /// Get total spendable balance.
    fn spendable_balance(&self) -> u64;
}
