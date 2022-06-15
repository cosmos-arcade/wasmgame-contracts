use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Stage;
use cosmwasm_std::{Addr, Uint128, Coin};

// ======================================================================================
// Entrypoints data structures
// ======================================================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Owner if none set to info.sender.
    pub owner: Option<String>,
    /// Address of the token.
    pub cw20_token_address: String,
    /// Price of the ticket to bid.
    pub ticket_price: Coin,
    /// The winning probability is associasted to the number of bins.
    pub bins: u8,
    /// Info related to the bidding stage.
    pub stage_bid: Stage,
    /// Info related to the airdrop claiming stage.
    pub stage_claim_airdrop: Stage,
    /// Info related to the prize claiming stage.
    pub stage_claim_prize: Stage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Update current contract configuration.
    UpdateConfig {
        /// NewOwner if non sent, contract gets locked. Recipients can receive airdrops
        /// but owner cannot register new stages.
        new_owner: Option<String>,
    },
    /// Place a bid.
    Bid {
        /// bidding bin value
        bin: u8,
    },
    /// Change the value of a previously placed bid.
    ChangeBid {
        /// input a value to change a previous bid
        bin: u8,
    },
    /// Remove a previously placed bid.
    RemoveBid {},
    /// Register Merkle root in the contract.
    RegisterMerkleRoots {
        /// MerkleRoot is hex-encoded merkle root.
        merkle_root_airdrop: String,
        total_amount_airdrop: Option<Uint128>,
        merkle_root_game: String,
        total_amount_game: Option<Uint128>
    },
    // Claim does not check if contract has enough funds, owner must ensure it.
    /// Claim airdrop bin.
    ClaimAirdrop {
        amount: Uint128,
        /// Proof is hex-encoded merkle proof.
        proof_airdrop: Vec<String>,
        proof_game: Vec<String>
    },
    ClaimPrize {},
    // Withdraw the remaining Airdrop tokens after expire time (only owner)
    WithdrawAirdrop {
        address: Addr,
    },
    // Withdraw the remaining Prize tokens after expire time (only owner)
    WithdrawPrize {
        address: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Stages {},
    Bid { address: String },
    MerkleRoots {},
    GameAmounts {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

// ======================================================================================
// Responses data structures
// ======================================================================================
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Option<String>,
    pub cw20_token_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StagesResponse {
    pub stage_bid: Stage,
    pub stage_claim_airdrop: Stage,
    pub stage_claim_prize: Stage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidResponse {
    pub bid: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MerkleRootsResponse {
    /// MerkleRoot is hex-encoded merkle root.
    pub merkle_root_airdrop: String,
    pub total_amount: Uint128,
    pub merkle_root_game: String

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameAmountsResponse {
    pub total_ticket_prize: Uint128,
    pub total_airdrop_amount: Uint128,
    pub total_airdrop_game_amount: Uint128,
    pub winners_amount: Uint128,
    pub total_claimed_airdrop: Uint128,
    pub total_claimed_prize: Uint128,
}
