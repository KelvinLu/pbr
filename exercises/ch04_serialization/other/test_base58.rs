use programming_bitcoin_in_rust::*;

use serialization::base58::Base58Encoding;
use util::hexadecimal::hexadecimal_string;

pub fn run() {
    let mut bytes = [0_u8; 32];

    let string = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";

    hexadecimal_string(string, &mut bytes).unwrap();

    let base58 = Base58Encoding::from_bytes(&bytes[0..(string.len() / 2)]);

    assert_eq!("9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6", base58.to_string());
}
