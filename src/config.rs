//! Payment plugin configuration.

/// Configuration for the payment plugin.
#[derive(Debug, Clone)]
pub struct PaymentConfig {
    /// Maximum channel capacity in satoshis.
    pub max_channel_capacity:    u64,
    /// Minimum channel capacity in satoshis.
    pub min_channel_capacity:    u64,
    /// Default invoice expiry in seconds.
    pub default_invoice_expiry:  u64,
    /// Maximum payment retries.
    pub max_payment_retries:     u32,
    /// Payment timeout in seconds.
    pub payment_timeout:         u64,
    /// Enable automatic channel management.
    pub auto_channel_management: bool,
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self {
            max_channel_capacity:    10_000_000, // 0.1 BTC
            min_channel_capacity:    20_000,     // 20k sats
            default_invoice_expiry:  3600,       // 1 hour
            max_payment_retries:     3,
            payment_timeout:         60,
            auto_channel_management: true,
        }
    }
}
