use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::Script;
use bitcoin::script::ScriptExecutionContext;
use bitcoin::script::Stack;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::OpCheckSigDigest;
use bitcoin::script::opcode::CryptographicOpcode;
use bitcoin::network::BitcoinNetworkType;
use bitcoin::transaction::Transaction;
use crypto::ecdsa::signature::Signature;
use serialization::signature::SignatureDerFormatBytes;
use serialization::point::UncompressedPointSecFormatBytes;
use serialization::bitcoin_address::BitcoinAddress;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_string;
use util::number::U256;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("[test] OP_CHECKSIG");

    section("Signature and public key (bytes)");

    let signature_hexadecimal = "3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";
    let mut signature_bytes: Vec<u8> = vec![];

    signature_bytes.resize(signature_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(signature_hexadecimal, &mut signature_bytes).unwrap();

    let public_key_hexadecimal = "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
    let mut public_key_bytes: Vec<u8> = vec![];

    public_key_bytes.resize(public_key_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(public_key_hexadecimal, &mut public_key_bytes).unwrap();

    let signature_format = SignatureDerFormatBytes::of(&signature_bytes);
    let public_key = UncompressedPointSecFormatBytes::of(&public_key_bytes);

    show_display(&signature_format);
    show_display(&public_key);

    section("Formatted signature values, point, and testnet address");

    let signature = Signature::from(signature_format.clone());
    let point = public_key.elliptic_curve_point_secp256k1().unwrap();

    show_debug(&signature);
    show_display(&point);
    show_display(&BitcoinAddress::for_uncompressed_point(BitcoinNetworkType::Testnet, public_key));

    section("Signed data");

    let data_hexadecimal = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
    let mut data_bytes = [0_u8; 32];

    hexadecimal_string(data_hexadecimal, &mut data_bytes).unwrap();

    let data = U256::from_be_bytes(data_bytes);

    show_debug(&data);

    let verified = signature.verify_point_secp256k1(data, point);

    message("Data matches signature and point:");
    show_display(&verified);

    assert!(verified);

    section("OP_CHECKSIG script");

    let sighash_byte = 0x01_u8;
    let mut signature_data_element_bytes = signature_format.bytes().to_vec();

    signature_data_element_bytes.push(sighash_byte);

    let script = [
        Element::Data(DataElement::of(&signature_data_element_bytes)),
        Element::Data(DataElement::of(&public_key.bytes())),
        Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpCheckSig)),
    ];

    let script = Script::new(&script).unwrap();

    show_display(&script);

    let transaction = Transaction::of(&[
        0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        0x01_u8,
        0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8,
        0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8, 0x11_u8,
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        0x01_u8, 0x01_u8,
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        0x00_u8,
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8
    ]);

    struct OpCheckSigDigestOverride {
        bytes: [u8; 32]
    }

    impl <'a> OpCheckSigDigest for OpCheckSigDigestOverride {
        fn digest(&self, _: &[u8]) -> [u8; 32] {
            self.bytes
        }
    }

    let context = ScriptExecutionContext {
        transaction: &transaction,
        input_index: 0,
        timestamp: EXAMPLE_TIMESTAMP,
        block_height: EXAMPLE_BLOCK_HEIGHT,
        checksig_digest: &OpCheckSigDigestOverride { bytes: data_bytes },
    };

    let mut stack = Stack::new(&script, &context);

    let success = stack.evaluate().unwrap();

    message("Successful execution:");
    show_display(&success);

    assert!(success);
}
