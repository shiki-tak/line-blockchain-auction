use cosmwasm_std::{Storage};

use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

use crate::constant::*;

use crate::types::*;

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
