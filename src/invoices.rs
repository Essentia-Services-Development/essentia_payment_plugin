//! Invoice generation and management.

use crate::config::PaymentConfig;
use crate::errors::{PaymentError, PaymentResult};
use crate::types::PaymentInvoice;

/// Invoice generator for creating payment invoices.
pub struct InvoiceGenerator {
    config: PaymentConfig,
}

impl InvoiceGenerator {
    /// Create a new invoice generator.
    pub fn new(config: PaymentConfig) -> Self {
        Self { config }
    }

    /// Generate a new invoice.
    pub fn generate(
        &self,
        amount: Option<u64>,
        description: impl Into<String>,
    ) -> PaymentResult<PaymentInvoice> {
        let description = description.into();

        if description.is_empty() {
            return Err(PaymentError::Invoice("Description cannot be empty".into()));
        }

        // Generate random payment hash
        let payment_hash = generate_random_bytes();

        // Calculate expiry
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let expiry = now + self.config.default_invoice_expiry;

        // In production, this would generate a proper BOLT11 invoice
        let encoded = format!("lnbc{}...placeholder", amount.unwrap_or(0));

        Ok(PaymentInvoice {
            payment_hash,
            amount,
            description,
            expiry,
            encoded,
        })
    }

    /// Verify an invoice.
    pub fn verify(&self, invoice: &PaymentInvoice) -> PaymentResult<bool> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if invoice.expiry < now {
            return Err(PaymentError::Invoice("Invoice has expired".into()));
        }

        Ok(true)
    }
}

/// Generate random bytes for payment hash.
fn generate_random_bytes() -> [u8; 32] {
    // Simple PRNG for placeholder - use proper entropy in production
    let mut bytes = [0u8; 32];
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((seed >> (i % 16)) & 0xFF) as u8;
    }
    bytes
}
