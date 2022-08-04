use pdao_pbc_api_server::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "80".to_owned())
        .parse::<u16>()
        .unwrap();
    println!("RUN ON PORT {}", port);
    run(port, HashMap::new(), Box::new(dummy::Dummy {})).await;
}
