pub mod test_suite;

use pdao_beacon_chain_common::message as pbc_message;
use pdao_colony_contract_common::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_tc::*;
use std::collections::HashMap;
use thiserror::Error;

/// A contract type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ContractType {
    Treasury,
    LightClient,
    Custom { name: String, description: String },
}

/// Information of a contract.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ContractInfo {
    /// The address of the contract.
    pub address: String,
    /// The type of the contract.
    pub contract_type: ContractType,
    /// The increasing sequence that is for preventing the replay attack.
    ///
    /// A valid message from PBC MUST carry the same number with this,
    /// in order to succesfully convince the contract. (i.e., this number is something that the consensus should have finalized on-chain).
    ///
    /// - Note1: this is totally irrelevant to the account sequence.
    /// - Note2: the light client contract doesn't need this because the 'block height' provides the same safety guard.
    pub sequence: u64,
}

/// An error that can occur when interacting with the contract.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    /// When there is a problem to access to the full node.
    #[error("connection error: {0}")]
    ConnectionError(String),
    /// When the transaction is rejected by the full node, before it gets to the contract.
    #[error("transaction rejected: {0}")]
    TransactionRejected(String),
    /// When the contract fails to decode the input data.
    #[error("failed to parse the payload of the transaction")]
    FailedToParseTransactionPayload,
    /// When the given proof is invalid.
    #[error("invalid proof given: got merkle root of {0} but expected {1}")]
    InvalidProof(String, String),
    /// When the given message is well decoded and verified, but the message argument is invalid.
    #[error("invalid message argument given: {0}")]
    InvalidMessageArgument(String),
    /// When the relayer account has not enough balance to execute the transaction.
    #[error("not enough balance: got {0}")]
    NotEnoughBalance(u64),
    /// When the account sequence given in the transaction is invalid.
    #[error("invalid account sequence; expected {0} but got {1}")]
    InvalidAccountSequence(u64, u64),
    /// When the contract sequence given in the transaction is invalid.
    #[error("invalid contract sequence; expected {0} but got {1}")]
    InvalidContractSequence(u64, u64),
    /// When the contract fails to execute the transaction with its own error.
    #[error("internal contract error: {0}")]
    InternalContractError(String),
    /// When the contract is missing.
    #[error("no such contract: {0}")]
    NoSuchContract(String),
    /// An unknown error.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// An abstract information about a block.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    /// The height of the block.
    pub height: u64,
    /// The UNIX timestamp of the block in seconds.
    pub timestamp: u64,
}

/// An abstraction of a colony chain, which is viewed by the relayer and the explorer.
///
/// Every colony chains MUST have at least two types of contracts: the light client and the treasury.
/// This interface directly specifies the actions applicable to the two essential contracts,
/// whereas it is generalized as an opaque packet for those non-essential, chain-local and application-specific contracts (custom contracts).
///
/// One trivial implementation of this trait would carry the address of the full node and
/// the relayer account used to submit message delivering transactions.
#[serde_tc_full]
pub trait ColonyChain: Send + Sync {
    /// Returns the name of the chain.
    async fn get_chain_name(&self) -> String;

    /// Checks whether the chain is healthy and the full node is running.
    async fn check_connection(&self) -> Result<(), Error>;

    /// Getes the latest finalized block on the chain.
    async fn get_last_block(&self) -> Result<Block, Error>;

    /// Returns the list of the contracts that are available on the chain
    async fn get_contract_list(&self) -> Result<Vec<ContractInfo>, Error>;

    /// Returns the address and the current balance (which is used to pay the gas fee) of the relayer account in this chain.
    ///
    /// Note that there is no authority of the relayer account; it is just a convenient account to pay the gas fee
    /// (i.e., there is no need to check the transaction submitter by the contract).
    async fn get_relayer_account_info(&self) -> Result<(String, Decimal), Error>;

    /// Returns the latest header that the light client has verified.
    async fn get_light_client_header(&self) -> Result<Header, Error>;

    /// Returns the current balance of fungible tokens in the treasury contract.
    async fn get_treasury_fungible_token_balance(&self) -> Result<HashMap<String, Decimal>, Error>;

    /// Returns the current balance of non-fungible tokens in the treasury contract, identified as `(collection address, token index)`.
    ///
    /// Note that the representation of the token index is specific to the chain, so we just provide as a string.
    async fn get_treasury_non_fungible_token_balance(&self)
        -> Result<Vec<(String, String)>, Error>;

    /// Updates the light client state by providing the next, valid block header and its proof.
    ///
    /// This is one of the message delivery methods; a transaction that carries the given data will be submitted to the chain.
    async fn update_light_client(
        &self,
        header: light_client::Header,
        proof: light_client::BlockFinalizationProof,
    ) -> Result<(), Error>;

    /// Transfers a given amount of fungible tokens from the treasury contract to the destination address.
    ///
    /// This is one of the message delivery methods; a transaction that carries the given data and the proof will be submitted to the chain.
    async fn transfer_treasury_fungible_token(
        &self,
        message: pbc_message::FungibleTokenTransfer,
        block_height: u64,
        proof: MerkleProof,
    ) -> Result<(), Error>;

    /// Transfers an NFT from the treasury contract to the destination address.
    ///
    /// This is one of the message methods; a transaction that carries the given data and the proof will be submitted to the chain.
    async fn transfer_treasury_non_fungible_token(
        &self,
        message: pbc_message::NonFungibleTokenTransfer,
        block_height: u64,
        proof: MerkleProof,
    ) -> Result<(), Error>;

    /// Delivers an arbitrary message, which is an opaque `String`, to one of the 'custom' contracts in this chain except the light client and the treasury,
    ///
    /// This is one of the message delivery methods; a transaction that carries the given data and the proof will be submitted to the chain.
    /// The target contract is the one with the type of `ContractType::Custom` and the name of `contract_name`.
    async fn deliver_custom_order(
        &self,
        contract_name: &str,
        message: pbc_message::Custom,
        block_height: u64,
        proof: MerkleProof,
    ) -> Result<(), Error>;
}

impl serde_tc::http::HttpInterface for dyn ColonyChain {}
