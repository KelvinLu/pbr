use programming_bitcoin_in_rust::*;

use serialization::bitcoin_address::BitcoinAddress;
use serialization::point::CompressedPointSecFormatBytes;
use serialization::point::UncompressedPointSecFormatBytes;
use bitcoin::network::BitcoinNetworkType;
use bitcoin::script_types::BitcoinTransactionType;
use crypto::secp256k1::Secp256k1Point;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::number::U256;

pub fn run() {
    let secret_e = U256::from(888).pow(U256::from(3));

    let point: EllipticCurvePoint = (secret_e * Secp256k1Point::generator_point()).into();
    let point_bytes = CompressedPointSecFormatBytes::from(&point);

    let address = BitcoinAddress::for_compressed_point(BitcoinNetworkType::Mainnet, point_bytes);

    assert_eq!("148dY81A9BmdpMhvYEVznrM45kWN32vSCN", address.to_string());

    let address = BitcoinAddress::for_compressed_point(BitcoinNetworkType::Testnet, point_bytes);

    assert_eq!("mieaqB68xDCtbUBYFoUNcmZNwk74xcBfTP", address.to_string());

    let secret_e = U256::from(321);

    let point: EllipticCurvePoint = (secret_e * Secp256k1Point::generator_point()).into();
    let point_bytes = UncompressedPointSecFormatBytes::from(&point);

    let address = BitcoinAddress::for_uncompressed_point(BitcoinNetworkType::Mainnet, point_bytes);

    assert_eq!("1S6g2xBJSED7Qr9CYZib5f4PYVhHZiVfj", address.to_string());

    let address = BitcoinAddress::for_uncompressed_point(BitcoinNetworkType::Testnet, point_bytes);

    assert_eq!("mfx3y63A7TfTtXKkv7Y6QzsPFY6QCBCXiP", address.to_string());

    let secret_e = U256::from(4242424242_u32);

    let point: EllipticCurvePoint = (secret_e * Secp256k1Point::generator_point()).into();
    let point_bytes = UncompressedPointSecFormatBytes::from(&point);

    let address = BitcoinAddress::for_uncompressed_point(BitcoinNetworkType::Mainnet, point_bytes);

    assert_eq!("1226JSptcStqn4Yq9aAmNXdwdc2ixuH9nb", address.to_string());

    let address = BitcoinAddress::for_uncompressed_point(BitcoinNetworkType::Testnet, point_bytes);

    assert_eq!("mgY3bVusRUL6ZB2Ss999CSrGVbdRwVpM8s", address.to_string());

    let address = BitcoinAddress::base58("mgY3bVusRUL6ZB2Ss999CSrGVbdRwVpM8s");

    assert_eq!(address.network(), BitcoinNetworkType::Testnet);
    assert_eq!(address.transaction_type(), BitcoinTransactionType::P2pkh);

    let address = BitcoinAddress::base58("148dY81A9BmdpMhvYEVznrM45kWN32vSCN");

    assert_eq!(address.network(), BitcoinNetworkType::Mainnet);
    assert_eq!(address.transaction_type(), BitcoinTransactionType::P2pkh);
}
