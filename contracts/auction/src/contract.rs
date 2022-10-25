#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, WasmMsg, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{auction, list_resolver, list_resolver_read, Auction, ListingToken};

use nft::{ExecuteMsg::{Approve, Transfer, TransferFrom}};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let config_state = Auction { auction_limit_block_height: msg.auction_limit_block_height };
    auction(deps.storage).save(&config_state)?;
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
        ExecuteMsg::Listing { nft_contract_address, id, minimum_bid } => {
            execute_listing(deps, env, info, nft_contract_address, id, minimum_bid)
        },
        ExecuteMsg::Bid { listing_id } => {
            execute_bid(deps, env, info, listing_id)
        },
        ExecuteMsg::Withdraw { listing_id } => {
            execute_withdraw(deps, env, info, listing_id)
        }
    }
}

pub fn execute_listing(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_contract_address: String,
    id: Uint128,
    minimum_bid: Coin
) -> Result<Response, ContractError> {
    let contract_address = deps.api.addr_validate(&nft_contract_address)?;
    let auction_config = auction(deps.storage).load()?;
    let auction_limit = auction_config.auction_limit_block_height;
    let listing_id = (nft_contract_address.clone() + &id.to_string())[10..].to_string();

    let listing_token = ListingToken {
        listing_id: listing_id.clone(),
        token_id: id,
        contract_address: contract_address,
        seller: info.sender.clone(),
        max_bid: minimum_bid,
        max_bidder: env.contract.address.clone(),
        block_limit: env.block.height + auction_limit,
    };

    list_resolver(deps.storage).save(listing_id.as_bytes(), &listing_token)?;

    let res = Response {
        submessages: vec![],
        messages: vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: nft_contract_address.clone(),
                send: vec![],
                msg: to_binary(&Approve {
                    recipient: env.contract.address.to_string(),
                    token_id: id.clone(),
                })?,
            }),
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: nft_contract_address,
                send: vec![],
                msg: to_binary(&TransferFrom {
                    sender: info.sender.to_string(),
                    recipient: String::from(env.contract.address.as_str()),
                    token_id: id,
                })?,
            }),
        ],
        attributes: vec![
            attr("listing", listing_id),
        ],
        data: None,
    };
    Ok(res)
}

pub fn execute_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    listing_id: String,
) -> Result<Response, ContractError> {
    let key = listing_id.as_bytes();
    let mut listing = list_resolver_read(deps.storage).load(key)?;
    if listing.block_limit < env.block.height {
        return Err(ContractError::AuctionEnded {});
    }

    if info.funds.len() != 1 {
        return Err(ContractError::InvalidBid {});
    }

    let send_fund = info.funds[0].clone();
    if send_fund.amount <= listing.max_bid.amount || send_fund.denom != listing.max_bid.denom {
        return Err(ContractError::InvalidBid {});
    }
    let last_bid = listing.max_bid;
    let last_bidder = listing.max_bidder;

    listing.max_bidder = info.sender.clone();
    listing.max_bid = send_fund;
    list_resolver(deps.storage).save(key, &listing)?;

    if env.contract.address != last_bidder {
        let res = Response {
            submessages: vec![],
            messages: vec![
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: last_bidder.to_string(),
                    amount: vec![last_bid],
                }),
            ],
            attributes: vec![
                attr("bid", listing_id),
            ],
            data: None,
        };
        Ok(res)
    } else {
        let res = Response {
            submessages: vec![],
            messages: vec![],
            attributes: vec![
                attr("bid", listing_id),
            ],
            data: None,
        };
        Ok(res)
    }
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    listing_id: String,
) -> Result<Response, ContractError> {
    let key = listing_id.as_bytes();
    let listing = list_resolver_read(deps.storage).load(key)?;

    if listing.block_limit >= env.block.height {
        return Err(ContractError::AuctionNotEnded {});
    }
    list_resolver(deps.storage).remove(key);

    if env.contract.address != listing.max_bidder {
        let res = Response {
            submessages: vec![],
            messages: vec![
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: listing.contract_address.to_string(),
                    send: vec![],
                    msg: to_binary(&Transfer {
                        recipient: listing.max_bidder.to_string(),
                        token_id: listing.token_id,
                    })?,
                }),
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: listing.max_bidder.to_string(),
                    amount: vec![listing.max_bid],
                }),
            ],
            attributes: vec![
                attr("listing_sold", listing_id),
            ],
            data: None,
        };
        Ok(res)
    } else {
        let res = Response {
            submessages: vec![],
            messages: vec![
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: listing.contract_address.to_string(),
                    send: vec![],
                    msg: to_binary(&Transfer {
                        recipient: listing.seller.to_string(),
                        token_id: listing.token_id,
                    })?,
                })
            ],
            attributes: vec![
                attr("listing_unsold", listing_id),
            ],
            data: None,
        };
        Ok(res)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListingToken { listing_id } => to_binary(&query_listing_token(deps, listing_id)?),

    }
}

pub fn query_listing_token(deps: Deps, listing_id: String) -> StdResult<ListingToken> {
    let listing = list_resolver_read(deps.storage).load(listing_id.as_bytes())?;
    Ok(listing)
}
