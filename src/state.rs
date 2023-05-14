use cosmwasm_std::CosmosMsg;
use cw_storage_plus::{Deque, Item, Map};

pub const NONCE: Item<u128> = Item::new("nonce");
pub const MSG_QUEUE: Deque<CosmosMsg> = Deque::new("msg_queue");


// ibc

/// (channel_id) -> count. Reset on channel closure.
pub const CONNECTION_COUNTS: Map<String, u32> = Map::new("connection_counts");
/// (channel_id) -> timeout_count. Reset on channel closure.
pub const TIMEOUT_COUNTS: Map<String, u32> = Map::new("timeout_count");