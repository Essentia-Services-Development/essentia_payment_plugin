//! Core payment types.

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
    #[must_use]
    pub fn from_satoshis(sats: u64) -> Self {
        Self { satoshis: sats }
    }

    /// Create from millisatoshis.
    #[must_use]
    pub fn from_millisatoshis(msats: u64) -> Self {
        Self { satoshis: msats / 1000 }
    }

    /// Get as millisatoshis.
    #[must_use]
    pub fn as_millisatoshis(&self) -> u64 {
        self.satoshis * 1000
    }
}

/// Lightning Network node interface
#[derive(Debug, Clone)]
pub struct LightningNode {
    /// Node public key
    pub pubkey: [u8; 33],
    /// Node alias
    pub alias:  String,
    /// Color for node identification
    pub color:  [u8; 3],
}

/// Lightning invoice (BOLT11)
#[derive(Debug, Clone)]
pub struct LightningInvoice {
    /// Payment hash
    pub payment_hash:   PaymentHash,
    /// Amount in satoshis
    pub amount_sats:    Option<u64>,
    /// Description
    pub description:    String,
    /// Expiry timestamp
    pub expiry:         u64,
    /// Encoded BOLT11 string
    pub bolt11:         String,
    /// Payment secret for AMP
    pub payment_secret: Option<[u8; 32]>,
}

/// Payment hash wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaymentHash(pub [u8; 32]);

impl PaymentHash {
    /// Create new payment hash from bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get as byte array
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Satoshis wrapper for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Satoshis(pub u64);

impl Satoshis {
    /// Create from u64
    pub fn new(amount: u64) -> Self {
        Self(amount)
    }

    /// Get as u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Escrow types for bounty system
#[derive(Debug, Clone)]
pub enum EscrowType {
    /// Lightning hold invoice (preimage release on completion)
    LightningHold { payment_hash: PaymentHash, preimage: Option<[u8; 32]> },
    /// On-chain multi-sig (2-of-3)
    MultiSig {
        funder_pubkey:   String,
        claimant_pubkey: String,
        arbiter_pubkey:  String,
        redeem_script:   String,
    },
}

/// Escrow status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    /// Funds deposited
    Funded,
    /// Funds released to recipient
    Released,
    /// Funds refunded to sender
    Refunded,
    /// Escrow disputed
    Disputed,
}

/// Subscription tier for VCS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionTier {
    /// Free tier - basic P2P features
    Free,
    /// Pro tier - AI features, priority seeding
    Pro,
    /// Enterprise - private repos, SLA
    Enterprise,
}

impl SubscriptionTier {
    /// Monthly price in satoshis
    pub fn monthly_price_sats(&self) -> u64 {
        match self {
            Self::Free => 0,
            Self::Pro => 10_000,         // ~$10 at $100k/BTC
            Self::Enterprise => 100_000, // ~$100 at $100k/BTC
        }
    }

    /// Features included
    pub fn features(&self) -> TierFeatures {
        match self {
            Self::Free => TierFeatures {
                ai_operations_per_month: 10,
                private_repos:           0,
                priority_seeding:        false,
                sla_guarantee:           false,
                max_repo_size_gb:        1.0,
            },
            Self::Pro => TierFeatures {
                ai_operations_per_month: 1000,
                private_repos:           10,
                priority_seeding:        true,
                sla_guarantee:           false,
                max_repo_size_gb:        10.0,
            },
            Self::Enterprise => TierFeatures {
                ai_operations_per_month: u32::MAX,
                private_repos:           u32::MAX,
                priority_seeding:        true,
                sla_guarantee:           true,
                max_repo_size_gb:        100.0,
            },
        }
    }
}

/// Features for each subscription tier
#[derive(Debug, Clone)]
pub struct TierFeatures {
    pub ai_operations_per_month: u32,
    pub private_repos:           u32,
    pub priority_seeding:        bool,
    pub sla_guarantee:           bool,
    pub max_repo_size_gb:        f64,
}
