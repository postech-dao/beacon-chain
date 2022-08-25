//! A set of messages that can be recorded in PBC, and sent to colony chains.
//!
//! Note that it's not mandatory to use the following types directly in
//! `trait ColonyChain` -> Contract communication, but the contract has to convert the message
//! into `DeliverableMessage` anyway, so that it can request a verification of it to the light client.
//! (because the light client needs the very exact string format that PBC has actually recorded)
//!
//! Messages will be JSON-encoded.

use serde::{Deserialize, Serialize};

/// A unit of a PBC-recored message that wraps the actual data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "polkadot",
    derive(scale::Encode, scale::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(
    feature = "near",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct MessageDeliveryRecord {
    pub chain: String,
    pub message: DeliverableMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "polkadot",
    derive(scale::Encode, scale::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(
    feature = "near",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub enum DeliverableMessage {
    FungibleTokenTransfer(FungibleTokenTransfer),
    NonFungibleTokenTransfer(NonFungibleTokenTransfer),
    Custom(Custom),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "polkadot",
    derive(scale::Encode, scale::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(
    feature = "near",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct FungibleTokenTransfer {
    pub token_id: String,
    pub amount: u128,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "polkadot",
    derive(scale::Encode, scale::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(
    feature = "near",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct NonFungibleTokenTransfer {
    pub collection_address: String,
    pub token_index: String,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "polkadot",
    derive(scale::Encode, scale::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(
    feature = "near",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct Custom {
    pub message: String,
    pub contract_sequence: u64,
}
