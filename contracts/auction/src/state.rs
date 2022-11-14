use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static AUCTION: &[u8] = b"auction";
pub static LIST_RESOLVER_KEY: &[u8] = b"listingresolver";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Auction {
    pub nft_contract_address: Option<Addr>,
    pub limit_block_height: u64,
}

pub fn auction(storage: &mut dyn Storage) -> Singleton<Auction> {
    singleton(storage, AUCTION)
}

pub fn auction_read(storage: &dyn Storage) -> ReadonlySingleton<Auction> {
    singleton_read(storage, AUCTION)
}

pub fn list_resolver(storage: &mut dyn Storage) -> Bucket<ListingToken> {
    bucket(storage, LIST_RESOLVER_KEY)
}

pub fn list_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<ListingToken> {
    bucket_read(storage, LIST_RESOLVER_KEY)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ListingToken {
    pub listing_id: String,
    pub token_id: Uint128,
    pub contract_address: Addr,
    pub seller: Addr,
    pub max_bid: Coin,
    pub max_bidder: Addr,
    pub block_limit: u64,
}
