use pdao_colony_common::*;
use std::collections::HashMap;

pub async fn run(
    _colony_chains: HashMap<String, Box<dyn ColonyChain>>,
    _pbc_api: Box<dyn pdao_pbc_node::PbcApi>,
) {
    // Periodically checks updates in PBC, and feeds the updates to the colony chains.
    unimplemented!()
}
