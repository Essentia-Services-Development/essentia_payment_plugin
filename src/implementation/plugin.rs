//! Payment plugin implementation.

use crate::{
    errors::{PaymentError, PaymentResult},
    implementation::{
        ChannelManager, InvoiceGenerator, LightningNodeImpl, PaymentConfig, PaymentRouter,
    },
    traits::{ChannelProvider, InvoiceProvider},
    types::{LightningInvoice, PaymentAmount, PaymentInvoice, PaymentStatus},
};

/// Main payment plugin interface.
#[derive(Debug)]
pub struct PaymentPlugin {
    config:            PaymentConfig,
    channel_manager:   ChannelManager,
    invoice_generator: InvoiceGenerator,
    router:            PaymentRouter,
    lightning_node:    LightningNodeImpl,
}

impl PaymentPlugin {
    /// Create a new payment plugin.
    #[must_use]
    pub fn new(config: PaymentConfig) -> Self {
        let invoice_generator = InvoiceGenerator::new(config.clone());

        Self {
            config,
            channel_manager: ChannelManager::new(),
            invoice_generator,
            router: PaymentRouter::new(),
            lightning_node: LightningNodeImpl::new("EssentiaNode".to_string()),
        }
    }

    /// Get current configuration.
    #[must_use]
    pub fn config(&self) -> &PaymentConfig {
        &self.config
    }

    /// Get channel manager.
    #[must_use]
    pub fn channels(&self) -> &ChannelManager {
        &self.channel_manager
    }

    /// Get mutable channel manager.
    pub fn channels_mut(&mut self) -> &mut ChannelManager {
        &mut self.channel_manager
    }

    /// Get the payment router.
    #[must_use]
    pub fn router(&self) -> &PaymentRouter {
        &self.router
    }

    /// Get the Lightning node.
    #[must_use]
    pub fn lightning_node(&self) -> &LightningNodeImpl {
        &self.lightning_node
    }

    /// Get mutable Lightning node.
    pub fn lightning_node_mut(&mut self) -> &mut LightningNodeImpl {
        &mut self.lightning_node
    }

    /// Create an invoice.
    pub fn create_invoice(
        &self, amount: Option<u64>, description: impl Into<String>,
    ) -> PaymentResult<PaymentInvoice> {
        self.invoice_generator.generate_invoice(amount, &description.into())
    }

    /// Create a Lightning invoice.
    pub async fn create_lightning_invoice(
        &mut self, amount_sats: u64, description: &str, expiry_secs: u64,
    ) -> PaymentResult<LightningInvoice> {
        self.lightning_node.create_invoice(amount_sats, description, expiry_secs).await
    }

    /// Send a payment.
    pub fn send_payment(&self, invoice: &PaymentInvoice) -> PaymentResult<PaymentStatus> {
        // Verify invoice first
        self.invoice_generator.verify(invoice)?;

        // Check we have sufficient balance
        let amount = invoice.amount.unwrap_or(0);
        let balance = self.channel_manager.total_local_balance();

        if amount > balance {
            return Err(PaymentError::InsufficientFunds(format!(
                "Need {} sats, have {}",
                amount, balance
            )));
        }

        // In production, would route and send payment
        Ok(PaymentStatus::Pending)
    }

    /// Send a Lightning payment.
    pub async fn send_lightning_payment(
        &self, invoice: &LightningInvoice,
    ) -> PaymentResult<PaymentStatus> {
        self.lightning_node.pay_invoice(invoice).await
    }

    /// Get total spendable balance.
    #[must_use]
    pub fn spendable_balance(&self) -> PaymentAmount {
        PaymentAmount::from_satoshis(self.channel_manager.total_local_balance())
    }

    /// Get payment status by hash.
    pub fn get_payment_status(&self, _payment_hash: &[u8; 32]) -> PaymentResult<PaymentStatus> {
        // In a real implementation, this would check the payment status
        // For now, return pending
        Ok(PaymentStatus::Pending)
    }

    /// Check Lightning invoice status.
    pub async fn check_lightning_invoice(
        &self, payment_hash: &crate::types::PaymentHash,
    ) -> PaymentResult<PaymentStatus> {
        self.lightning_node.check_invoice(payment_hash).await
    }
}

impl Default for PaymentPlugin {
    fn default() -> Self {
        Self::new(PaymentConfig::default())
    }
}

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = PaymentPlugin::default();
        assert!(plugin.config().max_channel_capacity > 0);
    }

    #[test]
    fn test_create_invoice() {
        let plugin = PaymentPlugin::default();
        let invoice = plugin.create_invoice(Some(1000), "Test payment");
        assert!(invoice.is_ok());
    }

    #[test]
    fn test_initial_balance() {
        let plugin = PaymentPlugin::default();
        assert_eq!(plugin.spendable_balance().satoshis, 0);
    }
}
