use cosmwasm_schema::cw_serde;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Coins = Vec<cosmwasm_std::Coin>;

#[cw_serde]
pub struct WithdrawRewardsResponse {
    pub records_num: u64,
    pub total_rewards: Coins,
}

#[cw_serde]
pub struct ContractMetadataResponse {
    pub owner_address: String,
    pub rewards_address: String,
}

#[cw_serde]
pub struct RewardsRecordsResponse {
    pub records: Vec<RewardsRecord>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct RewardsRecord {
    pub id: u64,
    pub rewards_address: String,
    pub rewards: Coins,
    pub calculated_height: i64,
    pub calculated_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    key: Option<String>,
    offset: Option<u64>,
    limit: Option<u64>,
    count_total: Option<bool>,
    reverse: Option<bool>,
}

impl PageRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn count_total(mut self) -> Self {
        self.count_total = Some(true);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = Some(true);
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    pub next_key: Option<String>,
    pub total: Option<u64>,
}
