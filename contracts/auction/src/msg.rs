use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub auction_nft: Option<AuctionNft>,
    pub auction_limit_block_height: u64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct AuctionNft {
    pub nft_code_id: u64,
    pub nft_contract_name: String,
    pub nft_contract_symbol: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Listing {
        nft_contract_address: Option<String>,
        id: Uint128,
        minimum_bid: Coin
    },
    Bid {
        listing_id: String,
    },
    Withdraw {
        listing_id: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ListingToken {
        listing_id: String,
    },
}

