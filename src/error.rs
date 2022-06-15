use cosmwasm_std::StdError;
use hex::FromHexError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Hex(#[from] FromHexError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid input")]
    InvalidInput {},

    #[error("Already claimed")]
    AlreadyClaimed {},

    #[error("Wrong length")]
    WrongLength {},

    #[error("Verification failed for {merkle_root}")]
    VerificationFailed { merkle_root: String },

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },

    // Claim prize errors.
    #[error("Not eligible to claim game prize")]
    NoteEligible {},

    #[error("Claim Prize stage is not over yet")]
    ClaimPrizeStageNotFinished {},

    // General stage errors.
    #[error("The {stage_name} has not started")]
    StageNotStarted { stage_name: String },

    #[error("The {stage_name} has ended")]
    StageEnded { stage_name: String },

    #[error("{second} stage overlaps {first} stage.")]
    StagesOverlap { first: String, second: String },

    // Bid errors.
    #[error("Bid stage cannot start in the past.")]
    BidStartPassed {},

    #[error("Fund sent insufficent for paying the bid price")]
    TicketPriceNotPaid {},

    #[error("Cannot be placed more than one bid per address")]
    CannotBidMoreThanOnce {},

    #[error("A bid must be placed before changing it")]
    BidNotPresent {},

    #[error("InsufficientFunds")]
    InsufficientFunds {},
    
    #[error("Bin does not exist. Number of bins: {bins}.")]
    BinDoesNotExist { bins: u8 },
}
