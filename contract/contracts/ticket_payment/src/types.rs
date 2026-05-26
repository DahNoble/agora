use soroban_sdk::{contracttype, Address, String};

pub const TRANSFER_FEE_BPS: u32 = 100;
pub const MAX_BPS: u32 = 10000;

// Re-export DataKey from the dedicated keys module so all existing imports continue to work.
pub use crate::keys::DataKey;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscountData {
    pub percentage: u32,
    pub expires_at: u64,
    pub max_uses: u32,
    pub current_uses: u32,
}

/// Optional parameters for `process_payment` that bundle rarely-used fields
/// to stay within Soroban's 10-argument limit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchaseOptions {
    /// SHA-256 preimage for the legacy global discount code system.
    pub code_preimage: Option<soroban_sdk::Bytes>,
    /// Optional referrer address for referral rewards.
    pub referrer: Option<soroban_sdk::Address>,
    /// Per-event limited-time discount code string.
    pub discount_code: Option<soroban_sdk::String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuctionConfig {
    pub start_price: i128,
    pub end_time: u64,
    pub min_increment: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriceSchedule {
    pub price: i128,
    pub valid_until: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParameterChange {
    AddGovernor(Address),
    RemoveGovernor(Address),
    AddTokenToWhitelist(Address),
    RemoveTokenFromWhitelist(Address),
    UpdateWithdrawalCap(Address, i128), // This is still i128 amount
    UpdateSlippage(u32),
    UpdateTransferFee(String, u32), // Changed from i128 to u32 basis points
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Executed,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterProposal {
    pub id: u64,
    pub proposer: Address,
    pub change: ParameterChange,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub vote_count: u32,
    pub voters: soroban_sdk::Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Confirmed,
    Refunded,
    Failed,
    CheckedIn,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    pub payment_id: String,
    pub event_id: String,
    pub buyer_address: Address,
    pub owner_address: Address, // The recipient who owns the ticket (can be different from buyer)
    pub ticket_tier_id: String,
    pub token_address: Address,
    pub amount: i128, // Payment token amount in stroops
    pub platform_fee: i128,
    pub organizer_amount: i128,
    pub status: PaymentStatus,
    pub transaction_hash: String,
    pub created_at: u64,
    pub confirmed_at: Option<u64>,
    pub refunded_amount: i128,
    pub is_soulbound: bool,
    pub last_checked_in_at: u64,
    pub referral_amount: i128,
    pub referrer: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventBalance {
    pub organizer_amount: i128,
    pub total_withdrawn: i128,
    pub platform_fee: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighestBid {
    pub bidder: Address,
    pub token_address: Address,
    pub amount: i128,
}
