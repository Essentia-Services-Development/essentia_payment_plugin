//! FlexForge Universal Editor Integration for Payment Plugin
//!
//! Provides payment and Lightning Network configuration panel within FlexForge.

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use essentia_traits::plugin_contracts::flexforge_integration::{
    ConfigField, ConfigSchema, FlexForgeIntegration, FlexForgePanelCategory, UiConfigurable,
};

/// FlexForge payment panel configuration
#[derive(Debug, Clone)]
pub struct FlexForgePaymentConfig {
    pub lightning_enabled:     bool,
    pub micropayments_enabled: bool,
    pub default_network:       String,
    pub min_channel_capacity:  u64,
    pub auto_channel:          bool,
    pub pqc_channels:          bool,
}

impl Default for FlexForgePaymentConfig {
    fn default() -> Self {
        Self {
            lightning_enabled:     true,
            micropayments_enabled: true,
            default_network:       "mainnet".to_string(),
            min_channel_capacity:  100000,
            auto_channel:          false,
            pqc_channels:          true,
        }
    }
}

/// FlexForge integration for the Payment plugin
#[derive(Debug)]
pub struct PaymentFlexForgeIntegration {
    config: Arc<Mutex<FlexForgePaymentConfig>>,
}

impl PaymentFlexForgeIntegration {
    /// Create a new FlexForge integration instance
    pub fn new() -> Self {
        Self { config: Arc::new(Mutex::new(FlexForgePaymentConfig::default())) }
    }

    fn config(&self) -> FlexForgePaymentConfig {
        self.config.lock().map(|c| c.clone()).unwrap_or_default()
    }

    fn set_config(&self, config: FlexForgePaymentConfig) {
        if let Ok(mut guard) = self.config.lock() {
            *guard = config;
        }
    }
}

impl Default for PaymentFlexForgeIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexForgeIntegration for PaymentFlexForgeIntegration {
    fn panel_id(&self) -> &str {
        "payment_config"
    }

    fn category(&self) -> FlexForgePanelCategory {
        FlexForgePanelCategory::System
    }

    fn display_name(&self) -> &str {
        "Payments"
    }

    fn on_panel_activate(&mut self) {}

    fn on_panel_deactivate(&mut self) {}
}

impl UiConfigurable for PaymentFlexForgeIntegration {
    fn config_schema(&self) -> ConfigSchema {
        ConfigSchema::new()
            .with_field(ConfigField::toggle(
                "lightning_enabled",
                "Lightning Network",
                true,
            ))
            .with_field(ConfigField::toggle(
                "micropayments_enabled",
                "Micropayments",
                true,
            ))
            .with_field(ConfigField::select("default_network", "Network", vec![
                "mainnet".to_string(),
                "testnet".to_string(),
                "regtest".to_string(),
            ]))
            .with_field(ConfigField::number(
                "min_channel_capacity",
                "Min Channel (sats)",
                100000.0,
                10000.0,
                10000000.0,
            ))
            .with_field(ConfigField::toggle("auto_channel", "Auto-Channel", false))
            .with_field(ConfigField::toggle("pqc_channels", "PQC Channels", true))
    }

    fn on_config_changed(&mut self, key: &str, value: &str) -> Result<(), String> {
        let mut config = self.config();
        match key {
            "lightning_enabled" => config.lightning_enabled = value == "true",
            "micropayments_enabled" => config.micropayments_enabled = value == "true",
            "default_network" => config.default_network = value.to_string(),
            "min_channel_capacity" => {
                config.min_channel_capacity = value.parse().map_err(|_| "Invalid number")?;
            },
            "auto_channel" => config.auto_channel = value == "true",
            "pqc_channels" => config.pqc_channels = value == "true",
            _ => return Err(format!("Unknown key: {}", key)),
        }
        self.set_config(config);
        Ok(())
    }

    fn apply_config(&mut self, config: &[(String, String)]) -> Result<(), String> {
        for (key, value) in config {
            self.on_config_changed(key, value)?;
        }
        Ok(())
    }

    fn get_current_config(&self) -> Vec<(String, String)> {
        let config = self.config();
        vec![
            (
                "lightning_enabled".to_string(),
                config.lightning_enabled.to_string(),
            ),
            (
                "micropayments_enabled".to_string(),
                config.micropayments_enabled.to_string(),
            ),
            ("default_network".to_string(), config.default_network),
            (
                "min_channel_capacity".to_string(),
                config.min_channel_capacity.to_string(),
            ),
            ("auto_channel".to_string(), config.auto_channel.to_string()),
            ("pqc_channels".to_string(), config.pqc_channels.to_string()),
        ]
    }

    fn reset_to_defaults(&mut self) {
        self.set_config(FlexForgePaymentConfig::default());
    }
}

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let integration = PaymentFlexForgeIntegration::new();
        let config = integration.config();
        assert!(config.lightning_enabled);
        assert!(config.pqc_channels);
    }
}
