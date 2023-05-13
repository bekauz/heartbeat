use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Beat {},
    Schedule { msg: CosmosMsg },
}

#[cw_serde]
pub enum QueryMsg {
    
}