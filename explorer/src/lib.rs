use pbc_colony_common::*;
use std::collections::HashMap;

pub async fn run(
    _colony_chains: HashMap<String, Box<dyn ColonyChain>>,
    _pbc_api: Box<dyn pbc_node::PbcApi>,
) {
    // Runs a web server that provides useful information about the colony chains and PBC.
    unimplemented!()
}
