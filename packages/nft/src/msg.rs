use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        token_id: Uint128,
    },
    TransferFrom {
        sender: String,
        recipient: String,
        token_id: Uint128,
    },
    Approve {
        recipient: String,
        token_id: Uint128,
    },
    ApproveForAll {
        opeartor: String,
        approved: bool,
    },
    Mint {
        name: String,
        uri: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: String },
    Owner { token_id: Uint128 },
    Allowance { token_id: Uint128 },
    Token { token_id: Uint128 },
}
