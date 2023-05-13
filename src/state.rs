use cosmwasm_std::CosmosMsg;
use cw_storage_plus::{Deque, Item};

pub const NONCE: Item<u128> = Item::new("nonce");
pub const MSG_QUEUE: Deque<CosmosMsg> = Deque::new("msg_queue");