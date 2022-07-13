//! A set of utilities, types, and constants to be used in the contract code, for all the colony chains based on WASM.
//!
//! Message-delivery related types (`LightClientUpdateMessage`, `FungibleTokenTransferMessage`, `NonFungibleTokenTransferMessage`, `CustomMessage`)
//! are en/decoded between the contract-relayer communication;
//! you should use the provided (in this crate) serialization method to decode the message given by the transaction,
//! as the matching serialization method will be used in the relayer side too, which depends on this crate as well.
//!
//! For the details of each message-delivery type, please refer to the `pbc-colony-common` crate.

pub mod light_client;

pub use light_client::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LightClientUpdateMessage {
    pub header: Header,
    pub proof: BlockFinalizationProof,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FungibleTokenTransferMessage {
    pub token_id: String,
    pub amount: Decimal,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NonFungibleTokenTransferMessage {
    pub collection_address: String,
    pub token_index: String,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomMessage {
    pub message: String,
    pub contract_sequence: u64,
}
