use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Err invalid name format")]
    InvalidNameFormat {},

    #[error("Err invalid symbol format")]
    InvalidSymbolFormat {},

    #[error("Err invalid token owner")]
    InvalidTokenOwner {},

    #[error("Err token not exist")]
    NotExistToken {},

    #[error("Err token owner not exist")]
    NotExistTokenOwner {},

    #[error("Err token allowance not exist")]
    NotExistTokenAllowance {},

    #[error("Err invalid token allowance")]
    InvalidTokenAllowance {},

    #[error("Err can not approve")]
    CanNotApprove {},

    #[error("Err invalid address")]
    InvalidAddress {},

}
