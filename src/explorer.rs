use std::collections::HashMap;

pub fn get_network_explorers() -> HashMap<u32, &'static str> {
    let mut network_explorers = HashMap::new();

    network_explorers.insert(1, "https://etherscan.io");
    network_explorers.insert(3, "https://ropsten.etherscan.io");
    network_explorers.insert(4, "https://rinkeby.etherscan.io");
    network_explorers.insert(5, "https://goerli.etherscan.io");
    network_explorers.insert(42, "https://kovan.etherscan.io");
    network_explorers.insert(137, "https://explorer-mainnet.maticvigil.com");
    network_explorers.insert(10, "https://optimistic.etherscan.io");
    network_explorers.insert(42161, "https://explorer.arbitrum.io");

    network_explorers
}
