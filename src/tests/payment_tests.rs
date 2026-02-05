//! Payment Plugin Tests

use crate::{LightningNodeImpl, PaymentAmount, PaymentConfig, PaymentPlugin, SubscriptionTier};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_amount_conversion() {
        let amount = PaymentAmount::from_satoshis(1000);
        assert_eq!(amount.satoshis, 1000);
        assert_eq!(amount.as_millisatoshis(), 1_000_000);
    }

    #[test]
    fn test_lightning_invoice_creation() {
        let node = LightningNodeImpl::new("TestNode".to_string());
        let node_info = node.get_node_info();
        assert_eq!(node_info.alias, "TestNode");
    }

    #[test]
    fn test_subscription_tier_features() {
        let free_features = SubscriptionTier::Free.features();
        assert_eq!(free_features.ai_operations_per_month, 10);
        assert_eq!(free_features.private_repos, 0);

        let pro_features = SubscriptionTier::Pro.features();
        assert_eq!(pro_features.ai_operations_per_month, 1000);
        assert!(pro_features.priority_seeding);
    }

    #[test]
    fn test_subscription_pricing() {
        assert_eq!(SubscriptionTier::Free.monthly_price_sats(), 0);
        assert_eq!(SubscriptionTier::Pro.monthly_price_sats(), 10_000);
        assert_eq!(SubscriptionTier::Enterprise.monthly_price_sats(), 100_000);
    }

    #[test]
    fn test_payment_plugin_creation() {
        let config = PaymentConfig::default();
        let _plugin = PaymentPlugin::new(config);
        // Just check that it creates successfully
    }
}
