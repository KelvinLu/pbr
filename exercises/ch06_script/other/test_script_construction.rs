use programming_bitcoin_in_rust::*;

use bitcoin::script::Script;
use bitcoin::script::ScriptBytes;
use bitcoin::script::Stack;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::opcode::CryptographicOpcode;
use bitcoin::script::opcode::BitwiseOpcode;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_string;

use crate::util::bitcoin::script::context::*;

pub fn run() {
    let digest = "ac8ad88e070e14816e1c8ba4ad44c31f192b7879570e759af5f341bb6c7e25f6";
    let mut digest_bytes: Vec<u8> = vec![];

    digest_bytes.resize(digest.len() / 2, 0_u8);
    hexadecimal_string(digest, &mut digest_bytes).unwrap();

    let unlock_script = [
        Element::Data(DataElement::of(b"please is the magic word")),
    ];
    let unlock_script = Script::new(&unlock_script).unwrap();

    let lock_script = [
        Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpSha256)),
        Element::Data(DataElement::of(&digest_bytes)),
        Element::Opcode(Opcode::Bitwise(BitwiseOpcode::OpEqual)),
    ];
    let lock_script = Script::new(&lock_script).unwrap();

    let script = unlock_script.concatenate(&lock_script).unwrap();
    let script_bytes = ScriptBytes::from(&script);

    assert_eq!(script.elements().len(), 4);

    let expected = "18706c6561736520697320746865206d6167696320776f7264a820ac8ad88e070e14816e1c8ba4ad44c31f192b7879570e759af5f341bb6c7e25f687";
    let mut expected_bytes: Vec<u8> = vec![];

    expected_bytes.resize(expected.len() / 2, 0_u8);
    hexadecimal_string(expected, &mut expected_bytes).unwrap();

    assert_eq!(script_bytes.bytes(), expected_bytes);

    let transaction = example_empty_transaction();
    let context = example_script_execution_context(&transaction);

    let mut stack = Stack::new(&script, &context);
    let success = stack.evaluate().unwrap();

    assert!(success);
}
