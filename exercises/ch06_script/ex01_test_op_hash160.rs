use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::Script;
use bitcoin::script::Stack;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::opcode::CryptographicOpcode;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_string;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("[test] OP_HASH160");

    let transaction = example_empty_transaction();
    let context = example_script_execution_context(&transaction);

    let script = [
        Element::Data(DataElement::of(b"hello world")),
        Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpHash160)),
    ];

    let script = Script::new(&script).unwrap();

    section("Script");

    show_display(&script);

    section("Execution");

    for opcode in Stack::new(&script, &context) {
        show_debug(&opcode);
    }

    section("Result");

    let mut stack = Stack::new(&script, &context);

    stack.evaluate_element();

    show_debug(&stack.peek());

    let expected_digest = "d7d5ee7824ff93f94c3055af9382c86c68b5ca92";
    let mut digest_bytes: Vec<u8> = vec![];

    digest_bytes.resize(expected_digest.len() / 2, 0_u8);
    hexadecimal_string(expected_digest, &mut digest_bytes).unwrap();

    assert_eq!(stack.peek().unwrap().bytes(), digest_bytes);
}
