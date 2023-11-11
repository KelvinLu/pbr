use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use serialization::signature::SignatureDerFormatBytes;
use crypto::ecdsa::signature::Signature;
use util::number::uint;

pub fn exercise() {
    prompt("DER format");

    section("Signature");

    uint! {
        let signature = Signature {
            r: 0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6_U256,
            s: 0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec_U256,
        };
    }

    show_debug(&signature);

    let signature_bytes = SignatureDerFormatBytes::from(&signature);

    show_display(&signature_bytes);
}
