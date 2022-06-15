# WasmGame

This is a smart contract for the gamified airdrop of a new protocol for the cosmwasm smart contract engine.

## Description
This contract allows to distribute a CW20 token while adding a funny part logic. There are 3 main stage that compose the business logic of the contract:

* __Users can try to guess__ in which bin they will fall during the CW20 contract airdrop. Every user is assigned to a bin according to the number of bin and the number of eligible addresses.

* __Users can claim their airdrop__ allocation by providing the merkle proof associated to the merkle root uploaded by the contract owner.

* __Users can claim the game prize__ which is divided evenly among all the winners.

The rules are really simple, every eligibile address will receive an airdrop of the protocol token and the lucky ones will receive even a prize. In order to bid on a bin a small ticket price must be payed. The amount derived from all the tickets will be divided among the winners of the game.

## Entry points

### InstantiateMsg

The instantiation of the contract requires information related to the start and end of each stage, the ticket price, the number of allowed bins on which a user can end up, the token to be airdropped.

```rust
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub cw20_token_address: String,
    pub ticket_price: Coin,
    pub bins: u8,
    pub stage_bid: Stage,
    pub stage_claim_airdrop: Stage,
    pub stage_claim_prize: Stage,
}
```

### ExecuteMsg

```rust
pub enum ExecuteMsg {
    UpdateConfig {
        new_owner: Option<String>,
    },
    Bid {
        bin: u8,
    },
    ChangeBid {
        bin: u8,
    },
    RemoveBid {},
    RegisterMerkleRoots {
        merkle_root_airdrop: String,
        total_amount_airdrop: Option<Uint128>,
        merkle_root_game: String,
        total_amount_game: Option<Uint128>
    },
    ClaimAirdrop {
        amount: Uint128,
        proof_airdrop: Vec<String>,
        proof_game: Vec<String>
    },
    ClaimPrize {},
    WithdrawAirdrop {
        address: Addr,
    },
    WithdrawPrize {
        address: Addr,
    },
}
```

- `UpdateConfig`: updates configuration.

- `Bid`: allows an address to try to guess the respective bin. To place a bid is necessary to pay a ticket price.

- `ChangeBid`: allows a user to change the previously chosen bin.

- `RemoveBid`: allows a user to remove the previously chosen bin. A user ho remove the bid will not partecipate to the game and will receive back the ticket price.

- `RegisterMerkleRoots`: allows the contract owner to register the Merkle root associated to the airdrop and the one associated to the game result.

- `ClaimAirdrop`: allows an eligible user to claim its airdrop.

- `ClaimAirdrop`: allows a winner user to claim its prize.

- `WithdrawAirdrop`: allows the contract owner to send the unclaimed airdrop to an address.

- `WithdrawPrize`: allows the contract owner to send the unclaimed game prize to an address.

### QueryMsg

``` rust
pub enum QueryMsg {
    Config {},
    Stages {},
    Bid { address: String },
    MerkleRoots {},
    GameAmounts {},
}
```

- `Config` returns configuration.

- `Stages` returns the stages.

- `Bid` returns the bid associated to an address.

- `MerkleRoots` returns the registered Merkle roots.

- `GameAmounts` returns the quantities associated to the airdrop, as for example, the amount of tickets payed, the amount of prize claimed, ecc.

## Schema

To generate schema inside `./schema` run:

``` shell
cargo schema
```

## Doc

To generate the documentation of the smart contract run:

``` shell
cargo doc
```

## Tests 
    
Is it possible to run all the tests with:

``` shell
cargo test
```

or just a subset of them with:

``` shell
cargo test WORDS_IN_TEST_NAME
```

The integration tests are grouped inside `./src/integration_test` and are:

1. `integration_test::test_instantiate`

2. `integration_test::valid_bid_no_change`

3. `integration_test::valid_bid_with_change`

4. `integration_test::change_bid`

5. `integration_test::invalid_bid`

6. `integration_test::remove_bid`

7. `integration_test::register_merkle_root`

8. `integration_test::claim_airdrop`

9. `integration_test::claim_prize`

10. `integration_test::withdraw_airdrop_and_prize`
