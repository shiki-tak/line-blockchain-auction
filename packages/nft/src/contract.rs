#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, callable_point, to_binary, to_vec, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Storage, Uint128,
};

use cosmwasm_storage::PrefixedStorage;
use std::ops::Add;

use crate::constant::*;
use crate::errors::ContractError;
use crate::msg::{InstantiateMsg,ExecuteMsg, QueryMsg};
use crate::store::*;
use crate::types::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if !is_valid_name(&msg.name) {
        return Err(ContractError::InvalidNameFormat {});
    };
    if !is_valid_symbol(&msg.symbol) {
        return Err(ContractError::InvalidSymbolFormat {});
    }

    let mut config_store = PrefixedStorage::new(deps.storage, CONFIG);
    let state = to_vec(&State {
        name: msg.name,
        symbol: msg.symbol,
    })?;

    config_store.set(KEY_STATE, &state);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer {
            recipient,
            token_id,
        } => handle_transfer(deps, env, info.sender.to_string(), recipient, token_id),
        ExecuteMsg::TransferFrom {
            sender,
            recipient,
            token_id,
        } => handle_transfer_from(deps, env, sender, recipient, token_id),
        ExecuteMsg::Approve {
            recipient,
            token_id,
        } => handle_approve(deps, env, info.sender.to_string(), recipient, token_id),
        ExecuteMsg::ApproveForAll { opeartor, approved } => {
            handle_approve_for_all(deps, env, info.sender.to_string(), opeartor, approved)
        }
        ExecuteMsg::Mint { name, uri } => handle_mint(deps, env, info.sender.to_string(), name, uri),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => query_balance(deps, address),
        QueryMsg::Owner { token_id } => query_owner(deps, token_id),
        QueryMsg::Allowance { token_id } => query_allowance(deps, token_id),
        QueryMsg::Token { token_id } => query_token(deps, token_id),
    }
}

fn handle_transfer(
    deps: DepsMut,
    _env: Env,
    sender: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);

    execute_transfer(deps, sender, recipient, token_id)
}

fn handle_transfer_from(
    deps: DepsMut,
    _env: Env,
    sender: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);
    
    // validation allowance
    if !validate_allowance(deps.storage, &token_id, recipient.clone()) {
        return Err(ContractError::NotExistTokenAllowance {});
    }

    execute_transfer(deps, sender, recipient, token_id)
}

fn execute_transfer(
    deps: DepsMut,
    from: String,
    to: String,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    // validation token
    if !validate_token_id(deps.storage, &token_id) {
        return Err(ContractError::NotExistToken {});
    }

    // validation token owner
    let (_, is_owner) = validate_token_owner(deps.storage, &token_id, from.clone());
    if !is_owner {
        return Err(ContractError::InvalidTokenOwner {});
    }

    /* update owner_tokens_store */
    // for from addr
    update_owner_tokens_store(deps.storage, token_id.clone(), from.clone(), false)?;
    // for to addr
    update_owner_tokens_store(deps.storage, token_id.clone(), to.clone(), true)?;

    // update token_owner_store
    write_token_owner_store(deps.storage, token_id.clone(), to.clone())?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "transfer_from"),
            attr("sender", from),
            attr("recipient", to),
            attr("token_id", &token_id.as_string()),
        ],
        data: None,
    };
    Ok(res)
}

fn update_owner_tokens_store(
    store: &mut dyn Storage,
    token_id: TokenId,
    owner: String,
    received: bool,
) -> StdResult<()> {
    let mut token_id_set = read_owner_tokens_store(store, owner.clone())?;
    if received {
        token_id_set.push(token_id.clone());
        write_owner_tokens_store(store, owner.clone(), token_id_set)?;
    } else {
        let mut new_token_id_set: Vec<TokenId> = Vec::new();
        for elm in token_id_set.into_iter() {
            if token_id.equal(&elm) {
                continue;
            }
            new_token_id_set.push(elm);
        }
        write_owner_tokens_store(store, owner, new_token_id_set)?;
    }
    Ok(())
}

fn handle_approve(
    deps: DepsMut,
    _env: Env,
    sender: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);
    if !check_be_able_to_approve(deps.storage, &token_id, sender.clone()) {
        return Err(ContractError::CanNotApprove{});
    }

    write_token_approvals_store(deps.storage, token_id.clone(), recipient.clone())?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "approve"),
            attr("sender", sender),
            attr("recipient", recipient.as_str()),
            attr("token_id", &token_id.as_string()),
            ],
        data: None,
    };

    Ok(res)
}

fn handle_approve_for_all(
    deps: DepsMut,
    _env: Env,
    sender: String,
    operator: String,
    approved: bool,
) -> Result<Response, ContractError> {
    if sender.eq(&operator) {
        return Err(ContractError::InvalidAddress{});
    }
    write_operators_store(deps.storage, sender.clone(), operator.clone(), approved)?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "approve_for_all"),
            attr("sender", sender),
            attr("operator", operator),
            ],
        data: None,
    };

    Ok(res)
}

fn handle_mint(
    deps: DepsMut,
    _env: Env,
    owner: String,
    name: String,
    uri: String,
) -> Result<Response, ContractError> {
    // generate token
    let new_token_id = make_token_id(deps.storage)?;
    let new_token = Token::new(new_token_id.clone(), name, uri);

    write_token_store(deps.storage, new_token_id.clone(), new_token)?;

    write_token_owner_store(deps.storage, new_token_id.clone(), owner.clone())?;

    let mut token_id_set = read_owner_tokens_store(deps.storage, owner.clone())?;

    token_id_set.push(new_token_id.clone());
    write_minted_token_id_store(deps.storage, token_id_set.clone())?;
    write_owner_tokens_store(deps.storage, owner.clone(), token_id_set)?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "mint"),
            attr("token_id", &new_token_id.as_string()),
            ],
        data: None,
    };

    Ok(res)
}

fn query_balance(
    deps: Deps,
    address: String,
) -> StdResult<Binary> {
    let res = read_owner_tokens_store(deps.storage, address)?;
    Ok(to_binary(&res.len())?)
}

fn query_owner(
    deps: Deps,
    value: Uint128,
) -> StdResult<Binary> {
    let address = read_token_owner_store(deps.storage, TokenId::new(value))?;
    
    Ok(to_binary(&address)?)
}

fn query_allowance(
    deps: Deps,
    value: Uint128,
) -> StdResult<Binary> {
    let token_id = TokenId::new(value);
    let res = read_token_approvals_store(deps.storage, token_id)?;

    Ok(to_binary(&res)?)
}

fn query_token(
    deps: Deps,
    value: Uint128,
) -> StdResult<Binary> {
    let token_id = TokenId::new(value);
    let res = read_token_store(deps.storage, token_id)?;

    Ok(to_binary(&res)?)
}

fn make_token_id(store: &mut dyn Storage) -> StdResult<TokenId> {
    let new_token_id = match get_current_token_id(store)? {
        Some(v) => v.as_u128().add(Uint128(1)),
        None => Uint128(0),
    };

    Ok(TokenId::new(new_token_id))
}

fn get_current_token_id(store: &dyn Storage) -> StdResult<Option<TokenId>> {
    let token_id_set = match read_minted_token_id_store(store)? {
        Some(record) => record,
        _ => return Ok(None),
    };

    let last_token_id = token_id_set.last().unwrap().clone();
    Ok(Some(last_token_id))
}

fn validate_token_id(store: &dyn Storage, token_id: &TokenId) -> bool {
    let current_token_id = get_current_token_id(store).unwrap();
    if current_token_id.unwrap().as_u128() < token_id.as_u128() {
        return false;
    }
    return true
}

fn check_be_able_to_approve(
    store: &dyn Storage,
    token_id: &TokenId,
    sender: String,
) -> bool {

    let (token_owner, is_owner) = validate_token_owner(store, token_id, sender.clone());
    if is_owner {
        return true;
    }

    let op = read_operators_store(store, token_owner.unwrap(), sender).unwrap();
    if op.is_none() {
        return false;
    }
    if op.unwrap() {
        return true;
    }

    return false;
}

fn validate_token_owner(
    store: &dyn Storage,
    token_id: &TokenId,
    addr: String,
) -> (Option<String>, bool) {
    let token_owner: Option<String> = read_token_owner_store(store, token_id.clone()).unwrap();

    if token_owner.clone().is_none() {
        return (None, false);
    } 

    if !token_owner.clone().unwrap().eq(&addr) {
        return (token_owner, false);
    }

    return (token_owner, true);
}

fn validate_allowance(
    store: &dyn Storage,
    token_id: &TokenId,
    addr: String,
) -> bool {
    let token_approval = read_token_approvals_store(store, token_id.clone()).unwrap();
    if token_approval.is_none() {
        return false
    }

    if !token_approval.unwrap().eq(&addr) {
        return false;
    }
    return true;
}

fn is_valid_name(name: &str) -> bool {
    if name.chars().count() < 3 || name.chars().count() > 30 {
        return false;
    }
    return true;
}

fn is_valid_symbol(symbol: &str) -> bool {
    if symbol.chars().count() < 3 || symbol.chars().count() > 30 {
        return false;
    }
    return true;
}

#[callable_point]
fn transfer(deps: DepsMut, env: Env, sender: String, recipient: String, value: Uint128) {
    handle_transfer(deps, env, sender, recipient, value).unwrap();
}

#[callable_point]
fn transfer_from(deps: DepsMut, env: Env, sender: String, recipient: String, value: Uint128) {
    handle_transfer_from(deps, env, sender, recipient, value).unwrap();
}

#[callable_point]
fn approve(deps: DepsMut, env: Env, sender: String, recipient: String, value: Uint128) {
    handle_approve(deps, env, sender, recipient, value).unwrap();
}
