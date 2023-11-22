use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::network::BitcoinNetworkType;
use bitcoin::script_types::BitcoinTransactionType;
use serialization::bitcoin_address::BitcoinAddress;
use util::hexadecimal::hexadecimal_string;

pub fn exercise() {
    prompt("[test] P2SH Bitcoin address");

    let redeem_script_hash = "74d691da1574e6b3c192ecfb52cc8984ee7b6c56";
    let mut redeem_script_hash_bytes: [u8; 20] = [0_u8; 20];

    hexadecimal_string(&redeem_script_hash, &mut redeem_script_hash_bytes).unwrap();

    message(&redeem_script_hash);

    let address = BitcoinAddress::for_hash_bytes(
        BitcoinAddress::base58_encoding_type(BitcoinNetworkType::Mainnet, BitcoinTransactionType::P2sh),
        &redeem_script_hash_bytes
    );

    show_debug(&BitcoinNetworkType::Mainnet);
    show_display(&address);

    assert_eq!(address.to_string(), "3CLoMMyuoDQTPRD3XYZtCvgvkadrAdvdXh");

    let address = BitcoinAddress::for_hash_bytes(
        BitcoinAddress::base58_encoding_type(BitcoinNetworkType::Testnet, BitcoinTransactionType::P2sh),
        &redeem_script_hash_bytes
    );

    show_debug(&BitcoinNetworkType::Testnet);
    show_display(&address);

    assert_eq!(address.to_string(), "2N3u1R6uwQfuobCqbCgBkpsgBxvr1tZpe7B");
}
