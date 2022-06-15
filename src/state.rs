use cosmwasm_std::{Addr, Uint128, Coin};
use cw_storage_plus::{Item, Map};
use cw_utils::{Duration, Scheduled};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct to manage the contract configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Owner If None set, contract is frozen.
    pub owner: Option<Addr>,
    pub cw20_token_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Struct to manage start and end of static stages.
pub struct Stage {
    /// Starting event for the stage.
    pub start: Scheduled,
    /// Ending event for the stage.
    pub duration: Duration,
}

/// Storage to manage contract configuration.
pub const CONFIG: Item<Config> = Item::new("config");

/// Storage for the bid stage info.
pub const STAGE_BID: Item<Stage> = Item::new("stage_bid");

/// Storage for the airdrop stage info.
pub const STAGE_CLAIM_AIRDROP: Item<Stage> = Item::new("stage_claim_airdrop");

/// Storage for the claiming prize stage info.
pub const STAGE_CLAIM_PRIZE: Item<Stage> = Item::new("stage_claim_prize");

/// Storage to save the first game ticket price.
pub const TICKET_PRICE: Item<Coin> = Item::new("ticket_price");

/// Storage to save the number of allowed bins for the game.
pub const BINS: Item<u8> = Item::new("bins");

/// Storage to manage the bid of each address.
pub const BIDS: Map<&Addr, u8> = Map::new("bids");

/// Storage for the Merkle root of the airdrop.
pub const MERKLE_ROOT_AIRDROP: Item<String> = Item::new("merkle_root_airdrop");

/// Storage for the Merkle root of the game.
pub const MERKLE_ROOT_GAME: Item<String> = Item::new("merkle_root_game");

/// Storage for the amount of airdropped tokens claimed.
/// This variable will consider:
/// - Amount from simple airdrop.
/// - Amount airdropped to winners of the first game.
pub const CLAIMED_AIRDROP_AMOUNT: Item<Uint128> = Item::new("claimed_amount");

/// Storage for the amount of the prize coming from the tickets claimed.
pub const CLAIMED_PRIZE_AMOUNT: Item<Uint128> = Item::new("claimed_prize");

/// Storage to save the number of winning addresses.
pub const WINNERS: Item<Uint128> = Item::new("winners");

/// Storage to keep track of the total prize from game tickets.
pub const TOTAL_TICKET_PRIZE: Item<Uint128> = Item::new("total_ticket_prize");

/// Total amount of tokens for the plain airdrop.
pub const TOTAL_AIRDROP_AMOUNT: Item<Uint128> = Item::new("total_amount_airdrop");

/// Total amount of tokens for the airdrop of the game winners.
pub const TOTAL_AIRDROP_GAME_AMOUNT: Item<Uint128> = Item::new("total_amount_game");

/// Storage to save if an address has claimed the airdrop or not.
pub const CLAIM_AIRDROP: Map<&Addr, bool> = Map::new("CLAIM_AIRDROP_PREFIX");

/// Storage to save if a winning address has claimed the prize or not.
pub const CLAIM_PRIZE: Map<&Addr, bool> = Map::new("claim_prize");