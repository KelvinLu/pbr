use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::base58::Base58Encoding;
use util::hexadecimal::hexadecimal_string;

pub fn exercise() {
    prompt("Base58 encoding (plain binary content)");

    let mut bytes = [0_u8; 32];

    let string = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";

    hexadecimal_string(string, &mut bytes).unwrap();

    section(string);
    show_display(&Base58Encoding::from_bytes(&bytes[0..(string.len() / 2)]));

    let string = "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";

    hexadecimal_string(string, &mut bytes).unwrap();

    section(string);
    show_display(&Base58Encoding::from_bytes(&bytes[0..(string.len() / 2)]));

    let string = "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";

    hexadecimal_string(string, &mut bytes).unwrap();

    section(string);
    show_display(&Base58Encoding::from_bytes(&bytes[0..(string.len() / 2)]));
}
