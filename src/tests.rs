use super::*;

#[test]
fn test_plugin_creation() {
    let config = PaymentConfig::default();
    assert!(config.max_channel_capacity > 0);
}
