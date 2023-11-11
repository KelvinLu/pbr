use programming_bitcoin_in_rust::*;

use serialization::bitcoin_wif::BitcoinWif;
use bitcoin::network::BitcoinNetworkType;
use util::number::U256;
use util::number::uint;

pub fn run() {
    let secret_e = U256::from(2).pow(U256::from(256)) - U256::from(2).pow(U256::from(199));

    let wif = BitcoinWif::for_secret_e(BitcoinNetworkType::Mainnet, secret_e, true);

    assert_eq!("L5oLkpV3aqBJ4BgssVAsax1iRa77G5CVYnv9adQ6Z87te7TyUdSC", wif.to_string());

    let secret_e = U256::from(2).pow(U256::from(256)) - U256::from(2).pow(U256::from(201));

    let wif = BitcoinWif::for_secret_e(BitcoinNetworkType::Testnet, secret_e, false);

    assert_eq!("93XfLeifX7Jx7n7ELGMAf1SUR6f9kgQs8Xke8WStMwUtrDucMzn", wif.to_string());

    let secret_e = uint!(0x0dba685b4511dbd3d368e5c4358a1277de9486447af7b3604a69b8d9d8b7889d_U256);

    let wif = BitcoinWif::for_secret_e(BitcoinNetworkType::Mainnet, secret_e, false);

    assert_eq!("5HvLFPDVgFZRK9cd4C5jcWki5Skz6fmKqi1GQJf5ZoMofid2Dty", wif.to_string());

    let secret_e = uint!(0x1cca23de92fd1862fb5b76e5f4f50eb082165e5191e116c18ed1a6b24be6a53f_U256);

    let wif = BitcoinWif::for_secret_e(BitcoinNetworkType::Testnet, secret_e, true);

    assert_eq!("cNYfWuhDpbNM1JWc3c6JTrtrFVxU4AGhUKgw5f93NP2QaBqmxKkg", wif.to_string());
}
