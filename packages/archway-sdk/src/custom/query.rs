use cosmwasm_schema::cw_serde;
use cosmwasm_std::CustomQuery;

use crate::types::{
    archwayrewardsv1beta1::{QueryContractMetadataRequest, QueryRewardsRecordsRequest},
    cosmosbasequeryv1beta1::PageRequest,
};

#[cw_serde]
pub enum ArchwayQuery {
    ContractMetadata(QueryContractMetadataRequest),
    RewardsRecords(QueryRewardsRecordsRequest),
}

impl CustomQuery for ArchwayQuery {}

impl ArchwayQuery {
    pub fn contract_metadata(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::ContractMetadata(QueryContractMetadataRequest {
            contract_address: contract_address.into(),
        })
    }

    pub fn rewards_records(rewards_address: impl Into<String>) -> Self {
        ArchwayQuery::RewardsRecords(QueryRewardsRecordsRequest {
            rewards_address: rewards_address.into(),
            pagination: None,
        })
    }

    pub fn rewards_records_with_pagination(
        rewards_address: impl Into<String>,
        pagination: PageRequest,
    ) -> Self {
        ArchwayQuery::RewardsRecords(QueryRewardsRecordsRequest {
            rewards_address: rewards_address.into(),
            pagination: Some(pagination),
        })
    }
}
