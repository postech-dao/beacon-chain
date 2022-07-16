//! A set of tests that can be used to test an implementation of `trait ColonyChainExt`.
//!
//! The actual implementer of the trait would use the following tests with their own instance.

use super::*;

/// An extension of `ColonyChain` that adds some utility methods for various operations useful for testing.
pub trait ColonyChainExt: ColonyChain {
    // TODO
}

pub async fn check_connection(chain: &dyn ColonyChainExt) {
    chain.check_connection().await.unwrap();
}

pub async fn get_contract_list(chain: &dyn ColonyChainExt) {
    chain.get_contract_list().await.unwrap();
}

pub async fn get_relayer_account_info(chain: &dyn ColonyChainExt) {
    chain.get_relayer_account_info().await.unwrap();
}

pub async fn get_light_client_header(chain: &dyn ColonyChainExt) {
    chain.get_light_client_header().await.unwrap();
}

pub async fn get_treasury_fungible_token_balance(chain: &dyn ColonyChainExt) {
    chain.get_treasury_fungible_token_balance().await.unwrap();
}

pub async fn get_treasury_non_fungible_token_balance(chain: &dyn ColonyChainExt) {
    chain
        .get_treasury_non_fungible_token_balance()
        .await
        .unwrap();
}
