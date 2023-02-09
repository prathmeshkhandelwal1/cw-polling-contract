use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorised")]
    Unauthorised {},

    #[error("Too many poll options")]
    TooManyOptions {},
}
