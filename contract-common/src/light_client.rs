//! An adaptation of the Simperby light client, with an additional layer of (possible) data en/decoding.
//!
//! This will be commonly used by every light client contracts in each colony chain.
//!
//! Later You will need to import `simperby-consensus` crate to use the actual implementation of the light client.

/// TODO: replace this with the proper type.
pub type Header = String;
/// TODO: replace this with the proper type.
pub type BlockFinalizationProof = String;
/// TODO: replace this with the proper type.
pub type MerkleProof = String;

/// A light client.
///
/// NOTE: this is a dummy implementation.
pub struct LightClient {
    pub height: u64,
    pub last_header: Header,
}

impl LightClient {
    /// Intializes a new light client with the initial header.
    pub fn new(initial_header: Header) -> Self {
        Self {
            height: 0,
            last_header: initial_header,
        }
    }

    /// Updates the header by providing the next block and the proof of it. Returns `true` if succeeded.
    pub fn update(&mut self, header: Header, proof: BlockFinalizationProof) -> bool {
        if proof.as_str() == "valid" {
            self.height += 1;
            self.last_header = header;
            true
        } else {
            false
        }
    }

    /// Verifies the given data with its proof.
    ///
    /// The data is opaque here, because all requests for verification are from other contracts,
    /// (remind that the light client contract is standalone)
    /// so the communication between the contracts would be a binary packet exchange (not a Rust code-level invocation).
    pub fn verify_commitment(
        &self,
        _message: Vec<u8>,
        block_height: u64,
        proof: MerkleProof,
    ) -> bool {
        proof.as_str() == "valid" && block_height == self.height
    }
}
