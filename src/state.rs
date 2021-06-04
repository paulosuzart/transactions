use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChargeId {
    pub provider_name: std::string::String,
    pub authz_id: std::string::String,
}

impl ChargeId {
    pub fn as_k(&self) -> std::string::String {
        let mut k = String::from(&self.authz_id);
        k.push_str(&self.provider_name);
        k
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Transaction {
    pub id: ChargeId,
    pub amount: i32,
    pub owner: Addr,
    pub settled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub trans: HashMap<Addr, Vec<Transaction>>,
}

pub const STATE: Item<State> = Item::new("state");
/// Maps for Address and Charge Id
pub const TRANS: Map<&[u8], Transaction> = Map::new("trans");
