use cosmwasm_schema::cw_serde;
use cosmwasm_std::CustomQuery;

use super::types::PageRequest;

#[cw_serde]
pub enum ArchwayQuery {
    ContractMetadata {
        contract_address: String,
    },
    RewardsRecords {
        rewards_address: String,
        pagination: Option<PageRequest>,
    },
}

impl CustomQuery for ArchwayQuery {}

impl ArchwayQuery {
    pub fn contract_metadata(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::ContractMetadata {
            contract_address: contract_address.into(),
        }
    }

    pub fn rewards_records(rewards_address: impl Into<String>) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: None,
        }
    }

    pub fn rewards_records_with_pagination(
        rewards_address: impl Into<String>,
        pagination: PageRequest,
    ) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: Some(pagination),
        }
    }
}
