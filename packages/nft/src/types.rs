use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_vec, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub token_id: TokenId,
    pub name: String,
    pub uri: String,
}

impl Token {
    pub fn new(token_id: TokenId, name: String, uri: String) -> Self {
        Token {token_id, name, uri}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenId(Uint128);

impl TokenId {
    pub fn new(v: Uint128) -> Self {
        TokenId(v)
    }

    pub fn as_u128(&self) -> Uint128 {
        self.0
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        to_vec(&self.0).unwrap()
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    pub fn equal(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OperatorKey {
    pub sender: String,
    pub operator: String
}
