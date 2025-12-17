//! Payment plugin type definitions.

/// Payment channel representation.
#[derive(Debug, Clone)]
pub struct PaymentChannel {
    /// Channel identifier.
    pub channel_id:     [u8; 32],
    /// Remote peer public key.
    pub peer_pubkey:    [u8; 33],
    /// Channel capacity in satoshis.
    pub capacity:       u64,
    /// Local balance in satoshis.
    pub local_balance:  u64,
    /// Remote balance in satoshis.
    pub remote_balance: u64,
    /// Current channel state.
    pub state:          ChannelState,
}

/// Channel state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelState {
    /// Channel is being opened.
    Opening,
    /// Channel is active.
    Active,
    /// Channel is being closed cooperatively.
    Closing,
    /// Channel was force closed.
    ForceClosed,
    /// Channel is fully closed.
    Closed,
}

/// Payment invoice.
#[derive(Debug, Clone)]
pub struct PaymentInvoice {
    /// Invoice identifier (payment hash).
    pub payment_hash: [u8; 32],
    /// Amount in satoshis (None for any amount).
    pub amount:       Option<u64>,
    /// Invoice description.
    pub description:  String,
    /// Expiry timestamp.
    pub expiry:       u64,
    /// Encoded invoice string (BOLT11).
    pub encoded:      String,
}

/// Payment route.
#[derive(Debug, Clone)]
pub struct PaymentRoute {
    /// Route hops.
    pub hops:             Vec<RouteHop>,
    /// Total fees in millisatoshis.
    pub total_fees_msat:  u64,
    /// Total time lock delta.
    pub total_cltv_delta: u32,
}

/// Single hop in a payment route.
#[derive(Debug, Clone)]
pub struct RouteHop {
    /// Node public key.
    pub pubkey:            [u8; 33],
    /// Short channel ID.
    pub short_channel_id:  u64,
    /// Fee in millisatoshis.
    pub fee_msat:          u64,
    /// CLTV expiry delta.
    pub cltv_expiry_delta: u16,
}

/// Payment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    /// Payment is pending.
    Pending,
    /// Payment is in flight.
    InFlight,
    /// Payment succeeded.
    Succeeded,
    /// Payment failed.
    Failed,
}

/// Payment amount representation.
#[derive(Debug, Clone, Copy)]
pub struct PaymentAmount {
    /// Amount in satoshis.
    pub satoshis: u64,
}

impl PaymentAmount {
    /// Create from satoshis.
    pub fn from_satoshis(sats: u64) -> Self {
        Self { satoshis: sats }
    }

    /// Create from millisatoshis.
    pub fn from_millisatoshis(msats: u64) -> Self {
        Self { satoshis: msats / 1000 }
    }

    /// Get as millisatoshis.
    pub fn as_millisatoshis(&self) -> u64 {
        self.satoshis * 1000
    }
}
