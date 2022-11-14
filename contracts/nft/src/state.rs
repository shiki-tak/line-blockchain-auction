use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_vec, Storage, Uint128};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

use crate::constant::*;

pub fn operators_resolver(storage: &mut dyn Storage) -> Bucket<bool> {
    bucket(storage, OPERATORS)
}

pub fn operators_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<bool> {
    bucket_read(storage, OPERATORS)
}

pub fn token_resolver(storage: &mut dyn Storage) -> Bucket<Token> {
    bucket(storage, TOKEN)
}

pub fn token_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Token> {
    bucket_read(storage, TOKEN)
}

pub fn owner_tokens_resolver(storage: &mut dyn Storage) -> Bucket<Vec<TokenId>> {
    bucket(storage, OWNER_TOKENS)
}

pub fn owner_tokens_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Vec<TokenId>> {
    bucket_read(storage, OWNER_TOKENS)
}

pub fn token_owner_resolver(storage: &mut dyn Storage) -> Bucket<String> {
    bucket(storage, TOKEN_OWNER)
}

pub fn token_owner_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<String> {
    bucket_read(storage, TOKEN_OWNER)
}

pub fn token_approvals_resolver(storage: &mut dyn Storage) -> Bucket<String> {
    bucket(storage, TOKEN_APPROVALS)
}

pub fn token_approvals_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<String> {
    bucket_read(storage, TOKEN_APPROVALS)
}

pub fn minted_token_ids_resolver(storage: &mut dyn Storage) -> Bucket<Vec<TokenId>> {
    bucket(storage, MINTED_TOKEN_ID)
}

pub fn minted_token_id_resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Vec<TokenId>> {
    bucket_read(storage, MINTED_TOKEN_ID)
}

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
