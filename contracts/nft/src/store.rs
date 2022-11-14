use cosmwasm_std::{StdResult, Storage};

use crate::state::*;

pub fn read_operators_store(
    store: &dyn Storage,
    sender: String,
    operator: String,
) -> StdResult<Option<bool>> {
    let ok = OperatorKey {sender, operator};
    let key = bincode::serialize(&ok).unwrap();
    let res = operators_resolver_read(store).may_load(&key)?;
    Ok(res)
}

pub fn write_operators_store(
    store: &mut dyn Storage,
    sender: String,
    operator: String,
    approved: bool,
) -> StdResult<()> {
    let ok = OperatorKey {sender, operator};
    let key = bincode::serialize(&ok).unwrap();
    operators_resolver(store).save(&key, &approved)?;
    Ok(())
}

pub fn read_token_store(
    store: &dyn Storage,
    token_id: TokenId,
) -> StdResult<Option<Token>> {
    let token = token_resolver_read(store).may_load(&token_id.as_bytes())?;
    Ok(token)
}

pub fn write_token_store(
    store: &mut dyn Storage,
    token_id: TokenId,
    token: Token,
) -> StdResult<()> {
    token_resolver(store).save(&token_id.as_bytes(), &token)?;
    Ok(())
}

pub fn read_owner_tokens_store(
    store: &dyn Storage,
    owner: String,
) -> StdResult<Vec<TokenId>> {
    let token_list = match owner_tokens_resolver_read(store).may_load(&owner.as_bytes())? {
        Some(record) => record,
        None => {
            let v: Vec<TokenId> = vec![];
            v
        }
    };
    Ok(token_list)
}

pub fn write_owner_tokens_store(
    store: &mut dyn Storage,
    owner: String,
    token_id_set: Vec<TokenId>,
) -> StdResult<()> {
    owner_tokens_resolver(store).save(owner.as_bytes(), &token_id_set)?;
    Ok(())
}

pub fn read_token_owner_store(
    store: &dyn Storage,
    token_id: TokenId,
) -> StdResult<Option<String>> {
    token_owner_resolver_read(store).may_load(&token_id.as_bytes())
}

pub fn write_token_owner_store(
    store: &mut dyn Storage,
    token_id: TokenId,
    owner: String,
) -> StdResult<()> {
    token_owner_resolver(store).save(&token_id.as_bytes(), &owner)?;
    Ok(())
}

pub fn read_minted_token_id_store(store: &dyn Storage) -> StdResult<Option<Vec<TokenId>>> {
    minted_token_id_resolver_read(store).may_load(b"minter")
}

pub fn write_minted_token_id_store(
    store: &mut dyn Storage,
    token_id_set: Vec<TokenId>,
) -> StdResult<()> {
    minted_token_ids_resolver(store).save(b"minter", &token_id_set)?;
    Ok(())
}

pub fn read_token_approvals_store(
    store: &dyn Storage,
    token_id: TokenId,
) -> StdResult<Option<String>> {
    token_approvals_resolver_read(store).may_load(&token_id.as_bytes())
}

pub fn write_token_approvals_store(
    store: &mut dyn Storage,
    token_id: TokenId,
    addr: String,
) -> StdResult<()> {
    token_approvals_resolver(store).save(&token_id.as_bytes(), &addr)?;
    Ok(())
}
