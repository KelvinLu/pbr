use programming_bitcoin_in_rust::*;

use serialization::signature::SignatureDerFormatBytes;
use crypto::ecdsa::signature::Signature;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_encode;
use util::number::uint;

pub fn run() {
    let mut buffer_72_byte_hexadecimal_string = [0_u8; 72 * 2];
    let mut buffer_72_bytes = [0_u8; 72];

    for ((r_value, s_value), expected) in [
        (
            (uint![0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6_U256], uint![0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec_U256]),
            "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
        ),
        (
            (uint![0x420_U256], uint![0x69_U256]),
            "300702020420020169"
        ),
    ] {
        let signature = Signature { r: r_value, s: s_value };
        let signature_bytes = SignatureDerFormatBytes::from(&signature);

        let text_range = 0..(usize::from(signature_bytes.length()) * 2);
        let range = 0..usize::from(signature_bytes.length());

        assert_eq!(hexadecimal_encode(signature_bytes.bytes(), &mut buffer_72_byte_hexadecimal_string[text_range.clone()]).unwrap()[text_range.clone()], *expected.as_bytes());

        buffer_72_bytes[range.clone()].clone_from_slice(signature_bytes.bytes());

        let signature_bytes = SignatureDerFormatBytes::of(&buffer_72_bytes[range.clone()]);
        let signature_from_bytes = Signature::from(signature_bytes);

        assert_eq!(signature, signature_from_bytes);
    }
}
