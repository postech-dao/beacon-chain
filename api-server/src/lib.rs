pub mod dummy;

use pdao_colony_common::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Runs a web server that provides useful information about the colony chains and PBC.
pub async fn run(
    port: u16,
    mut colony_chains: HashMap<String, Arc<dyn ColonyChain>>,
    _pbc_api: Box<dyn pdao_pbc_node::PbcApi>,
) {
    colony_chains.insert("dummy".to_owned(), Arc::new(dummy::Dummy {}));
    let colony_chains: HashMap<String, Arc<dyn serde_tc::http::HttpInterface>> = colony_chains
        .into_iter()
        .map(|(name, chain)| (name, serde_tc::http::create_http_object(chain)))
        .collect();
    serde_tc::http::run_server(port, colony_chains).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_connection() {
        tokio::spawn(run(12000, HashMap::new(), Box::new(dummy::Dummy {})));

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:12000/dummy")
            .header("content-type", "application/json")
            .body(r#"{"method": "get_chain_name", "params": {}}"#)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        assert_eq!(response.json::<String>().await.unwrap(), "dummy");
    }
}
