//! Payment plugin implementation.

use crate::channels::ChannelManager;
use crate::config::PaymentConfig;
use crate::errors::{PaymentError, PaymentResult};
use crate::invoices::InvoiceGenerator;
use crate::router::PaymentRouter;
use crate::types::{PaymentAmount, PaymentInvoice, PaymentStatus};

/// Main payment plugin interface.
pub struct PaymentPlugin {
    config: PaymentConfig,
    channel_manager: ChannelManager,
    invoice_generator: InvoiceGenerator,
    router: PaymentRouter,
}

impl PaymentPlugin {
    /// Create a new payment plugin.
    pub fn new(config: PaymentConfig) -> Self {
        let invoice_generator = InvoiceGenerator::new(config.clone());

        Self {
            config,
            channel_manager: ChannelManager::new(),
            invoice_generator,
            router: PaymentRouter::new(),
        }
    }

    /// Get current configuration.
    pub fn config(&self) -> &PaymentConfig {
        &self.config
    }

    /// Get channel manager.
    pub fn channels(&self) -> &ChannelManager {
        &self.channel_manager
    }

    /// Get mutable channel manager.
    pub fn channels_mut(&mut self) -> &mut ChannelManager {
        &mut self.channel_manager
    }

    /// Create an invoice.
    pub fn create_invoice(
        &self,
        amount: Option<u64>,
        description: impl Into<String>,
    ) -> PaymentResult<PaymentInvoice> {
        self.invoice_generator.generate(amount, description)
    }

    /// Send a payment.
    pub fn send_payment(
        &self,
        invoice: &PaymentInvoice,
    ) -> PaymentResult<PaymentStatus> {
        // Verify invoice first
        self.invoice_generator.verify(invoice)?;

        // Check we have sufficient balance
        let amount = invoice.amount.unwrap_or(0);
        let balance = self.channel_manager.total_local_balance();

        if amount > balance {
            return Err(PaymentError::InsufficientFunds(
                format!("Need {} sats, have {}", amount, balance)
            ));
        }

        // In production, would route and send payment
        Ok(PaymentStatus::Pending)
    }

    /// Get total spendable balance.
    pub fn spendable_balance(&self) -> PaymentAmount {
        PaymentAmount::from_satoshis(self.channel_manager.total_local_balance())
    }
}

impl Default for PaymentPlugin {
    fn default() -> Self {
        Self::new(PaymentConfig::default())
    }
}

#[cfg(test)]
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
