use cosmwasm_schema::cw_serde;
use cosmwasm_std::IbcMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Tick {},
    Schedule { msg: IbcMsg },
}

#[cw_serde]
pub enum QueryMsg {
    
}