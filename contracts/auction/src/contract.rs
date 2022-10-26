#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    dynamic_link, to_binary, Addr, BankMsg, Binary, Coin, Contract, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{auction, list_resolver, list_resolver_read, Auction, ListingToken};

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
        contract_address: contract_address.clone(),
        seller: info.sender.clone(),
        max_bid: minimum_bid,
        max_bidder: env.contract.address.clone(),
        block_limit: env.block.height + auction_limit,
    };

    list_resolver(deps.storage).save(listing_id.as_bytes(), &listing_token)?;

    let nft_contract = NftContract {address: contract_address.clone()};
    nft_contract.approve(env.contract.address.to_string(), env.contract.address.to_string(), id.clone());
    nft_contract.transfer_from(info.sender.to_string().clone(), env.contract.address.to_string(), id.clone());

    let mut res = Response::default();
    res.add_attribute("listing", listing_id);
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
        let mut res = Response::default();
        res.add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: last_bidder.to_string(),
            amount: vec![last_bid],
        }));
        res.add_attribute("bid", listing_id);
        Ok(res)
    } else {
        let mut res = Response::default();
        res.add_attribute("bid", listing_id);
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
        let nft_contract = NftContract {address: listing.contract_address.clone()};
        nft_contract.transfer(env.contract.address.to_string(), listing.max_bidder.to_string(), listing.token_id.clone());
        
        let mut res = Response::default();
        res.add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: listing.max_bidder.to_string(),
            amount: vec![listing.max_bid],
        }));
        res.add_attribute("listing_sold", listing_id);

        Ok(res)
    } else {
        let nft_contract = NftContract {address: listing.contract_address.clone()};
        nft_contract.transfer(env.contract.address.to_string(), listing.seller.to_string(), listing.token_id.clone());

        let mut res = Response::default();
        res.add_attribute("listing_unsold", listing_id);

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
