use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::Stack;
use bitcoin::script::Script;
use bitcoin::script::ScriptBytes;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use util::byte_string::ByteString;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("A puzzle script");
    message("What does it do?");

    section("UTXO lock script");

    let lock_script_bytes = ScriptBytes::of(&[0x6e_u8, 0x87_u8, 0x91_u8, 0x69_u8, 0xa7_u8, 0x7c_u8, 0xa7_u8, 0x87_u8]);
    let lock_script = Script::try_from(&lock_script_bytes).unwrap();

    show_display(&lock_script_bytes);
    show_display(&lock_script);

    section("Transaction input unlock script");
    message("Place two arbitrary but unequal values here ...");

    let unlock_script = [
        Element::Data(DataElement::of(&[0x01_u8])),
        Element::Data(DataElement::of(&[0x01_u8])),
    ];

    let unlock_script = Script::new(&unlock_script).unwrap();
    let unlock_script_bytes = ScriptBytes::from(&unlock_script);

    show_display(&unlock_script_bytes);
    show_display(&unlock_script);

    section("Combined script");

    let combined_script_bytes = unlock_script_bytes.concatenate(&lock_script_bytes);
    let combined_script = Script::try_from(&combined_script_bytes).unwrap();

    show_display(&combined_script_bytes);
    show_display(&combined_script);

    let transaction = example_empty_transaction();
    let context = example_script_execution_context(&transaction);
    let mut stack = Stack::new(&combined_script, &context);

    section("Execution:");

    loop {
        match stack.evaluate_element() {
            Some(Ok(opcode)) => {
                section(&format!("{:?}", &opcode));
                show_debug(&stack.stack());
            },
            Some(Err(_)) => {
                message("... execution finished (with error)");
                break;
            },
            None => {
                message("... execution finished");
                break;
            },
        };
    }

    show_debug(&stack.evaluate());
}
