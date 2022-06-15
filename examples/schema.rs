use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use wasmgame_contracts::msg::{
    InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, StagesResponse,
    BidResponse, MerkleRootsResponse, GameAmountsResponse
};
use wasmgame_contracts::state::{Config, Stage};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(Stage), &out_dir);
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);

    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(StagesResponse), &out_dir);
    export_schema(&schema_for!(BidResponse), &out_dir);
    export_schema(&schema_for!(MerkleRootsResponse), &out_dir);
    export_schema(&schema_for!(GameAmountsResponse), &out_dir);
}
