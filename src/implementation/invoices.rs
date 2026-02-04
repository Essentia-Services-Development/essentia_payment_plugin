//! Invoice generation and management.

use essentia_core::time;

use crate::{
    errors::{PaymentError, PaymentResult},
    implementation::config::PaymentConfig,
    traits::InvoiceProvider,
    types::PaymentInvoice,
};

/// Invoice generator for creating payment invoices.
#[derive(Debug)]
pub struct InvoiceGenerator {
    config: PaymentConfig,
}

impl InvoiceGenerator {
    /// Create a new invoice generator.
    #[must_use]
    pub fn new(config: PaymentConfig) -> Self {
        Self { config }
    }

    /// Verify an invoice.
    pub fn verify(&self, invoice: &PaymentInvoice) -> PaymentResult<bool> {
        let now = time::unix_seconds_sync();

        if invoice.expiry < now {
            return Err(PaymentError::Invoice("Invoice has expired".into()));
        }

        Ok(true)
    }
}

impl InvoiceProvider for InvoiceGenerator {
    fn generate_invoice(
        &self, amount: Option<u64>, description: &str,
    ) -> PaymentResult<PaymentInvoice> {
        if description.is_empty() {
            return Err(PaymentError::Invoice("Description cannot be empty".into()));
        }

        // Generate random payment hash
        let payment_hash = generate_random_bytes();

        // Calculate expiry
        let now = time::unix_seconds_sync();
        let expiry = now + self.config.default_invoice_expiry;

        // In production, this would generate a proper BOLT11 invoice
        let encoded = format!("lnbc{}...placeholder", amount.unwrap_or(0));

        Ok(PaymentInvoice {
            payment_hash,
            amount,
            description: description.to_string(),
            expiry,
            encoded,
        })
    }

    fn verify_invoice(&self, invoice: &PaymentInvoice) -> PaymentResult<bool> {
        self.verify(invoice)
    }
}

/// Generate random bytes for payment hash.
fn generate_random_bytes() -> [u8; 32] {
    // Simple PRNG for placeholder - use proper entropy in production
    let mut bytes = [0u8; 32];
    let seed = time::unix_nanos_sync();

    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((seed >> (i % 16)) & 0xFF) as u8;
    }
    bytes
}
