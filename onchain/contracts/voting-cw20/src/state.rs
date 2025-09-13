
use cosmwasm_std::Addr;
use cosmwasm_storage::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub end: u64,
    pub voters: Vec<Addr>,
}

pub const POLLS: Item<Vec<Poll>> = Item::new("polls");
