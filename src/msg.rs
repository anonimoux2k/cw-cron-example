use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementBegin {},
    IncrementEnd {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    GetConfig {},
}

#[cw_serde]
pub enum SudoMsg {
    CronBeginBlock {},
    CronEndBlock {},
}
