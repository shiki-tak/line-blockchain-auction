#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    dynamic_link, to_binary, Addr, BankMsg, Binary, Contract, ContractResult, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, WasmMsg, Reply, ReplyOn, StdResult, StdError, SubcallResponse, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{auction, list_resolver, list_resolver_read, Auction, ListingToken};

use nft::InstantiateMsg as NftInstantiateMsg;
use nft::{ExecuteMsg::{Approve, Transfer, TransferFrom}};

use std::collections::HashMap;

pub const INSTANTIATE_REPLY_ID: u64 = 1;

#[derive(Contract)]
struct NftContract {
    address: Addr,
}

#[dynamic_link(NftContract)]
trait Nft: Contract {
    fn transfer(&self, sender: String, recipient: String, value: Uint128);
    fn transfer_from(&self, sender: String, recipient: String, value: Uint128);
    fn approve(&self, sender: String, recipient: String, value: Uint128);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut res = Response::new();
    match msg.auction_nft {
        Some(v) => {
            let instantiate_msg = WasmMsg::Instantiate {
                admin: Some(info.sender.to_string()),
                code_id: v.nft_code_id,
                msg: to_binary(&NftInstantiateMsg {
                    name: v.nft_contract_name,
                    symbol: v.nft_contract_symbol,
                })?,
                send: vec![],
                label: "auction-nft".to_string(),
            };
            res.add_submessage(INSTANTIATE_REPLY_ID, instantiate_msg, None, ReplyOn::Success);
        },
        None => {},
    }

    let config_state = Auction { nft_contract_address: None, limit_block_height: msg.auction_limit_block_height };
    auction(deps.storage).save(&config_state)?;
    Ok(res)
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
    nft_contract_address: Option<String>,
    id: Uint128,
    minimum_bid: Coin
) -> Result<Response, ContractError> {
    let contract_address: Addr;
    let auction_config = auction(deps.storage).load()?;

    match nft_contract_address {
        None => {
            contract_address = auction_config.nft_contract_address.unwrap();
        },
        Some(v) => {
            contract_address = deps.api.addr_validate(&v)?;
        }
    }

    let auction_limit = auction_config.limit_block_height;
    let listing_id = (contract_address.to_string().clone() + &id.to_string())[10..].to_string();

    let listing_token = ListingToken {
        listing_id: listing_id.clone(),
        token_id: id,
        contract_address: contract_address.clone(),
        seller: info.sender.clone(),
        max_bid: minimum_bid,
        max_bidder: env.contract.address.clone(),
        block_limit: env.block.height + auction_limit,
    };

    list_resolver(deps.storage).save(listing_id.as_bytes(), &listing_token)?;

    // use dynamic link
    // let nft_contract = NftContract {address: contract_address.clone()};
    // nft_contract.approve(
    //     env.contract.address.to_string(),
    //     env.contract.address.to_string(),
    //     id.clone()
    // );
    // nft_contract.transfer_from(
    //     info.sender.to_string().clone(),
    //     env.contract.address.to_string(),
    //     id.clone()
    // );
    

    let mut res = Response::new();
    res.add_attribute("action", "listing");
    res.add_attribute("listing_id", listing_id);

    // use message
    res.add_message(
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address.to_string().clone(),
            send: vec![],
            msg: to_binary(&Approve {
            recipient: env.contract.address.to_string(),
            token_id: id.clone(),
            })?,
        })
    );
    res.add_message(
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address.to_string(),
            send: vec![],
            msg: to_binary(&TransferFrom {
                sender: info.sender.to_string(),
                recipient: String::from(env.contract.address.as_str()),
                token_id: id,
            })?,
        }),
    );

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
        let mut res = Response::new();
        res.add_attribute("action", "bid");
        res.add_attribute("listing_id", listing_id);
        res.add_message(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: last_bidder.to_string(),
                amount: vec![last_bid],
            })
        );
        Ok(res)
    } else {
        let mut res = Response::new();
        res.add_attribute("action", "bid");
        res.add_attribute("listing_id", listing_id);
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
        let mut res = Response::new();
        res.add_attribute("action", "withdraw");
        res.add_attribute("status", "sold");
        res.add_attribute("listing_id", listing_id);
        res.add_message(
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: listing.contract_address.to_string(),
                send: vec![],
                msg: to_binary(&Transfer {
                    recipient: listing.max_bidder.to_string(),
                    token_id: listing.token_id,
                })?,
            })            
        );
        res.add_message(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: listing.max_bidder.to_string(),
                amount: vec![listing.max_bid],
            })            
        );
        Ok(res)
    } else {
        let mut res = Response::new();
        res.add_attribute("action", "withdraw");
        res.add_attribute("status", "unsold");
        res.add_attribute("listing_id", listing_id);
        res.add_message(
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: listing.contract_address.to_string(),
                send: vec![],
                msg: to_binary(&Transfer {
                    recipient: listing.seller.to_string(),
                    token_id: listing.token_id,
                })?,
            })            
        );
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    match (reply.id, reply.result) {
        (INSTANTIATE_REPLY_ID, ContractResult::Ok(response)) =>
        handle_instantiate_reply(deps, response),
        _ => Err(StdError::generic_err("invalid reply id or result")),
    }
}

fn handle_instantiate_reply(deps: DepsMut, responses: SubcallResponse) -> StdResult<Response> {
    let attrs: HashMap<_, _> = responses.events
        .into_iter()
        .find(|e| e.kind == "wasm")
        .map(|e| e.attributes)
        .ok_or(StdError::generic_err("wasm not found"))?
        .into_iter()
        .map(|a| (a.key, a.value))
        .collect();

    let contract_address = Addr::unchecked(
        attrs.get("contract_address").ok_or(StdError::generic_err("contract address not found"))?
    );
    
    let mut auction_config = auction(deps.storage).load()?;
    auction_config.nft_contract_address = Some(contract_address.clone());
    auction(deps.storage).save(&auction_config)?;

    let mut res = Response::new();
    res.add_attribute("nft_auction_contract_address", contract_address);
    Ok(res)
}
