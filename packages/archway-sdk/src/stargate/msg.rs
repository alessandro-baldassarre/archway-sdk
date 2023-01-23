use crate::types::archwayrewardsv1beta1::{
    msg_withdraw_rewards::{Mode, RecordIDs, RecordsLimit},
    ContractMetadata, MsgSetContractMetadata, MsgSetContractMetadataResponse, MsgWithdrawRewards,
    MsgWithdrawRewardsResponse,
};

impl MsgSetContractMetadata {
    /// Helper function to define set contract metadata message
    /// * **sender_address**  is the msg sender address (bech32 encoded).
    /// * **contract_address** defines the contract address (bech32 encoded).
    /// * **owner_address**  is the contract owner address that can modify contract reward options (bech32 encoded).
    /// That could be the contract admin or the contract itself.
    /// If owner_address is set to contract address, contract can modify the metadata on its own
    /// using WASM bindings.
    /// * **rewards_address** is an address to distribute rewards to (bech32 encoded).
    /// If not set (empty), rewards are not distributed for this contract.
    pub fn new(
        sender_address: impl Into<String>,
        contract_address: impl Into<String>,
        owner_address: impl Into<String>,
        rewards_address: impl Into<String>,
    ) -> Self {
        MsgSetContractMetadata {
            sender_address: sender_address.into(),
            metadata: Some(ContractMetadata {
                contract_address: contract_address.into(),
                owner_address: owner_address.into(),
                rewards_address: rewards_address.into(),
            }),
        }
    }
}

impl MsgWithdrawRewards {
    /// Helper function to define withraw rewards message with records limit mode.
    /// * **rewards_address** is the address to distribute rewards to (bech32 encoded).
    /// * **records_limit** defines the maximum number of RewardsRecord objects to process.
    /// If provided limit is 0, the default limit is used.
    pub fn records_limit(rewards_address: impl Into<String>, records_limit: u64) -> Self {
        MsgWithdrawRewards {
            rewards_address: rewards_address.into(),
            mode: Some(Mode::RecordsLimit(RecordsLimit {
                limit: records_limit,
            })),
        }
    }

    /// Helper function to define withraw rewards message with records limit mode.
    /// * **rewards_address** is the address to distribute rewards to (bech32 encoded).
    /// * **record_ids** defines specific RewardsRecord object IDs to process.
    pub fn record_ids(rewards_address: impl Into<String>, record_ids: Vec<u64>) -> Self {
        MsgWithdrawRewards {
            rewards_address: rewards_address.into(),
            mode: Some(Mode::RecordIds(RecordIDs { ids: record_ids })),
        }
    }
}

// Implement trait From to convert structs messages to binary
impl From<MsgSetContractMetadata> for cosmwasm_std::Binary {
    fn from(value: MsgSetContractMetadata) -> Self {
        let mut bytes = Vec::new();
        prost::Message::encode(&value, &mut bytes).expect("Message encoding should work");
        cosmwasm_std::Binary(bytes)
    }
}

impl From<MsgWithdrawRewards> for cosmwasm_std::Binary {
    fn from(value: MsgWithdrawRewards) -> Self {
        let mut bytes = Vec::new();
        prost::Message::encode(&value, &mut bytes).expect("Message encoding should work");
        cosmwasm_std::Binary(bytes)
    }
}

// Implement trait From to convert structs messages to stargate cosmos message
impl<T> From<MsgSetContractMetadata> for cosmwasm_std::CosmosMsg<T> {
    fn from(value: MsgSetContractMetadata) -> Self {
        cosmwasm_std::CosmosMsg::<T>::Stargate {
            type_url: "/archway.rewards.v1beta1.MsgSetContractMetadata".to_owned(),
            value: value.into(),
        }
    }
}

impl<T> From<MsgWithdrawRewards> for cosmwasm_std::CosmosMsg<T> {
    fn from(value: MsgWithdrawRewards) -> Self {
        cosmwasm_std::CosmosMsg::<T>::Stargate {
            type_url: "/archway.rewards.v1beta1.MsgWithdrawRewards".to_owned(),
            value: value.into(),
        }
    }
}

// Implement trait From to convert binary data to messages response structs
impl TryFrom<cosmwasm_std::Binary> for MsgSetContractMetadataResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(binary: cosmwasm_std::Binary) -> Result<Self, Self::Error> {
        use ::prost::Message;
        Self::decode(&binary[..]).map_err(|e| cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(#ident).to_string(),
            msg: format!(
                "Unable to decode binary: \n  - base64: {}\n  - bytes array: {:?}\n\n{:?}",
                binary,
                binary.to_vec(),
                e
            ),
        })
    }
}

impl TryFrom<cosmwasm_std::Binary> for MsgWithdrawRewardsResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(binary: cosmwasm_std::Binary) -> Result<Self, Self::Error> {
        use ::prost::Message;
        Self::decode(&binary[..]).map_err(|e| cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(#ident).to_string(),
            msg: format!(
                "Unable to decode binary: \n  - base64: {}\n  - bytes array: {:?}\n\n{:?}",
                binary,
                binary.to_vec(),
                e
            ),
        })
    }
}

// Implement trait From to convert submessages results to messages response structs
impl TryFrom<cosmwasm_std::SubMsgResult> for MsgSetContractMetadataResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(result: cosmwasm_std::SubMsgResult) -> Result<Self, Self::Error> {
        result
            .into_result()
            .map_err(|e| cosmwasm_std::StdError::GenericErr { msg: e })?
            .data
            .ok_or_else(|| cosmwasm_std::StdError::NotFound {
                kind: "cosmwasm_std::SubMsgResult::<T>".to_string(),
            })?
            .try_into()
    }
}

impl TryFrom<cosmwasm_std::SubMsgResult> for MsgWithdrawRewardsResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(result: cosmwasm_std::SubMsgResult) -> Result<Self, Self::Error> {
        result
            .into_result()
            .map_err(|e| cosmwasm_std::StdError::GenericErr { msg: e })?
            .data
            .ok_or_else(|| cosmwasm_std::StdError::NotFound {
                kind: "cosmwasm_std::SubMsgResult::<T>".to_string(),
            })?
            .try_into()
    }
}
