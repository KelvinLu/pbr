use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::network::BitcoinNetworkType;
use bitcoin::script_types::BitcoinTransactionType;
use serialization::bitcoin_address::BitcoinAddress;
use util::hexadecimal::hexadecimal_string;

pub fn exercise() {
    prompt("[test] P2PKH Bitcoin address");

    let public_key_hash = "74d691da1574e6b3c192ecfb52cc8984ee7b6c56";
    let mut public_key_hash_bytes: [u8; 20] = [0_u8; 20];

    hexadecimal_string(&public_key_hash, &mut public_key_hash_bytes).unwrap();

    message(&public_key_hash);

    let address = BitcoinAddress::for_hash_bytes(
        BitcoinAddress::base58_encoding_type(BitcoinNetworkType::Mainnet, BitcoinTransactionType::P2pkh),
        &public_key_hash_bytes
    );

    show_debug(&BitcoinNetworkType::Mainnet);
    show_display(&address);

    assert_eq!(address.to_string(), "1BenRpVUFK65JFWcQSuHnJKzc4M8ZP8Eqa");

    let address = BitcoinAddress::for_hash_bytes(
        BitcoinAddress::base58_encoding_type(BitcoinNetworkType::Testnet, BitcoinTransactionType::P2pkh),
        &public_key_hash_bytes
    );

    show_debug(&BitcoinNetworkType::Testnet);
    show_display(&address);

    assert_eq!(address.to_string(), "mrAjisaT4LXL5MzE81sfcDYKU3wqWSvf9q");
}
