use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::Script;
use bitcoin::script::ScriptExecutionContext;
use bitcoin::script::Stack;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::OpCheckSigDigest;
use bitcoin::script::opcode::ConstantOpcode;
use bitcoin::script::opcode::CryptographicOpcode;
use bitcoin::network::BitcoinNetworkType;
use bitcoin::transaction::Transaction;
use crypto::ecdsa::signature::Signature;
use serialization::signature::SignatureDerFormatBytes;
use serialization::point::CompressedPointSecFormatBytes;
use serialization::bitcoin_address::BitcoinAddress;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_string;
use util::number::U256;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("[test] OP_CHECKMULTISIG");

    section("Signature and public key (bytes)");

    let signature_hexadecimal = "3045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a89937";
    let mut signature_bytes: Vec<u8> = vec![];

    signature_bytes.resize(signature_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(signature_hexadecimal, &mut signature_bytes).unwrap();

    let public_key_hexadecimal = "022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb70";
    let mut public_key_bytes: Vec<u8> = vec![];

    public_key_bytes.resize(public_key_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(public_key_hexadecimal, &mut public_key_bytes).unwrap();

    let signature_format_1 = SignatureDerFormatBytes::of(&signature_bytes);
    let public_key_1 = CompressedPointSecFormatBytes::of(&public_key_bytes);

    message("Signature and public key #1");
    show_display(&signature_format_1);
    show_display(&public_key_1);

    let signature_hexadecimal = "3045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e754022";
    let mut signature_bytes: Vec<u8> = vec![];

    signature_bytes.resize(signature_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(signature_hexadecimal, &mut signature_bytes).unwrap();

    let public_key_hexadecimal = "03b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb71";
    let mut public_key_bytes: Vec<u8> = vec![];

    public_key_bytes.resize(public_key_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(public_key_hexadecimal, &mut public_key_bytes).unwrap();

    let signature_format_2 = SignatureDerFormatBytes::of(&signature_bytes);
    let public_key_2 = CompressedPointSecFormatBytes::of(&public_key_bytes);

    message("Signature and public key #2");
    show_display(&signature_format_2);
    show_display(&public_key_2);

    section("Formatted signature values, point, and testnet address");

    let signature_1 = Signature::from(signature_format_1.clone());
    let point_1 = public_key_1.elliptic_curve_point_secp256k1().unwrap();

    message("Signature and public key #1");
    show_debug(&signature_1);
    show_display(&point_1);
    show_display(&BitcoinAddress::for_compressed_point(BitcoinNetworkType::Testnet, public_key_1));

    let signature_2 = Signature::from(signature_format_2.clone());
    let point_2 = public_key_2.elliptic_curve_point_secp256k1().unwrap();

    message("Signature and public key #2");
    show_debug(&signature_2);
    show_display(&point_2);
    show_display(&BitcoinAddress::for_compressed_point(BitcoinNetworkType::Testnet, public_key_2));

    section("Signed data");

    let data_hexadecimal = "e71bfa115715d6fd33796948126f40a8cdd39f187e4afb03896795189fe1423c";
    let mut data_bytes = [0_u8; 32];

    hexadecimal_string(data_hexadecimal, &mut data_bytes).unwrap();

    let data = U256::from_be_bytes(data_bytes);

    show_debug(&data);

    let verified_1 = signature_1.verify_point_secp256k1(data, point_1);
    let verified_2 = signature_2.verify_point_secp256k1(data, point_2);

    message("Data matches signature and point:");
    show_display(&verified_1);
    show_display(&verified_2);

    assert!(verified_1);
    assert!(verified_2);

    section("OP_MULTICHECKSIG script (2-of-2)");

    let sighash_byte = 0x01_u8;
    let mut signature_data_element_bytes_1 = signature_format_1.bytes().to_vec();
    let mut signature_data_element_bytes_2 = signature_format_2.bytes().to_vec();

    signature_data_element_bytes_1.push(sighash_byte);
    signature_data_element_bytes_2.push(sighash_byte);

    let script = [
        Element::Opcode(Opcode::Constant(ConstantOpcode::OpFalse)),
        Element::Data(DataElement::of(&signature_data_element_bytes_1)),
        Element::Data(DataElement::of(&signature_data_element_bytes_2)),
        Element::Opcode(Opcode::Constant(ConstantOpcode::Op2)),
        Element::Data(DataElement::of(&public_key_1.bytes())),
        Element::Data(DataElement::of(&public_key_2.bytes())),
        Element::Opcode(Opcode::Constant(ConstantOpcode::Op2)),
        Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisig)),
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
