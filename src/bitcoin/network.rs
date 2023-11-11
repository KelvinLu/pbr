//! Bitcoin network.

/// Denotes various instances of the Bitcoin network/blockchain.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum BitcoinNetworkType {
    Mainnet,
    Testnet,
}
