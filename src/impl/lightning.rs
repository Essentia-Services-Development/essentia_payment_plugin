//! Lightning Network integration for the payment plugin.

use crate::{
    errors::{PaymentError, PaymentResult},
    types::{LightningInvoice, LightningNode, PaymentHash, PaymentStatus},
};

/// Lightning Network node implementation
#[derive(Debug)]
pub struct LightningNodeImpl {
    /// Node public key
    pubkey: [u8; 33],
    /// Node alias
    alias: String,
    /// Active channels
    channels: std::collections::HashMap<[u8; 32], crate::types::PaymentChannel>,
    /// Pending invoices
    invoices: std::collections::HashMap<PaymentHash, LightningInvoice>,
}

impl LightningNodeImpl {
    /// Create new Lightning node
    pub fn new(alias: String) -> Self {
        // Generate a random pubkey for demo purposes
        let mut pubkey = [0u8; 33];
        pubkey[0] = 0x02; // Compressed pubkey prefix
        for (i, item) in pubkey.iter_mut().enumerate().skip(1) {
            *item = (i as u8).wrapping_mul(7);
        }

        Self {
            pubkey,
            alias,
            channels: std::collections::HashMap::new(),
            invoices: std::collections::HashMap::new(),
        }
    }

    /// Get node info
    pub fn get_node_info(&self) -> LightningNode {
        LightningNode {
            pubkey: self.pubkey,
            alias: self.alias.clone(),
            color: [0x49, 0x68, 0xad], // Default blue color
        }
    }

    /// Create a Lightning invoice
    pub async fn create_invoice(
        &mut self,
        amount_sats: u64,
        description: &str,
        expiry_secs: u64,
    ) -> PaymentResult<LightningInvoice> {
        // Generate payment hash
        let mut payment_hash_bytes = [0u8; 32];
        for (i, item) in payment_hash_bytes.iter_mut().enumerate() {
            *item = (i as u8).wrapping_add((amount_sats % 255) as u8);
        }
        let payment_hash = PaymentHash::new(payment_hash_bytes);

        // Create BOLT11-like encoded string (simplified)
        let bolt11 = format!("lnbc{}n1{}", amount_sats, bytes_to_hex(&payment_hash_bytes[..16]));

        let invoice = LightningInvoice {
            payment_hash,
            amount_sats: Some(amount_sats),
            description: description.to_string(),
            expiry: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| PaymentError::Invoice("Time error".to_string()))?
                .as_secs() + expiry_secs,
            bolt11,
            payment_secret: None,
        };

        self.invoices.insert(payment_hash, invoice.clone());
        Ok(invoice)
    }

    /// Check invoice payment status
    pub async fn check_invoice(&self, payment_hash: &PaymentHash) -> PaymentResult<PaymentStatus> {
        match self.invoices.get(payment_hash) {
            Some(_) => {
                // In a real implementation, this would check with the Lightning node
                // For demo, assume payment is successful after some time
                Ok(PaymentStatus::Succeeded)
            }
            None => Err(PaymentError::Invoice("Invoice not found".to_string())),
        }
    }

    /// Pay an invoice
    pub async fn pay_invoice(&self, invoice: &LightningInvoice) -> PaymentResult<PaymentStatus> {
        // In a real implementation, this would send the payment through the Lightning network
        // For demo, check if we have sufficient balance
        let total_balance: u64 = self.channels.values().map(|ch| ch.local_balance).sum();
        let amount = invoice.amount_sats.unwrap_or(0);

        if total_balance < amount {
            return Err(PaymentError::InsufficientFunds(format!(
                "Need {} sats, have {} in channels",
                amount, total_balance
            )));
        }

        Ok(PaymentStatus::Succeeded)
    }

    /// Open a channel with a peer
    pub async fn open_channel(
        &mut self,
        peer_pubkey: [u8; 33],
        capacity_sats: u64,
        push_sats: u64,
    ) -> PaymentResult<[u8; 32]> {
        let mut channel_id = [0u8; 32];
        for i in 0..32 {
            channel_id[i] = peer_pubkey[i % 33].wrapping_add(i as u8);
        }

        let channel = crate::types::PaymentChannel {
            channel_id,
            peer_pubkey,
            capacity: capacity_sats,
            local_balance: capacity_sats - push_sats,
            remote_balance: push_sats,
            state: crate::types::ChannelState::Active,
        };

        self.channels.insert(channel_id, channel);
        Ok(channel_id)
    }

    /// Close a channel
    pub async fn close_channel(&mut self, channel_id: &[u8; 32]) -> PaymentResult<()> {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            channel.state = crate::types::ChannelState::Closing;
            // In real implementation, would negotiate closure
            channel.state = crate::types::ChannelState::Closed;
        }
        Ok(())
    }

    /// Get total channel balance
    pub fn total_balance(&self) -> u64 {
        self.channels.values().map(|ch| ch.local_balance).sum()
    }
}

/// Convert bytes to hex string (SSOP compliant)
fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        hex.push(HEX_CHARS[(byte >> 4) as usize] as char);
        hex.push(HEX_CHARS[(byte & 0xF) as usize] as char);
    }
    hex
}
