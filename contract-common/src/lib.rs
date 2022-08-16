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
