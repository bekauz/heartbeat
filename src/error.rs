use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Queue is full. Process some messages before pushing more.")]
    QueueFullError {},

    #[error("Failed to retrieve message.")]
    QueuePopError {},
}