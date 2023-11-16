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
    prompt("Unlock the script");

    section("UTXO lock script");
    message("a.k.a. ScriptPubKey");

    let lock_script_bytes = ScriptBytes::of(&[0x76_u8, 0x93_u8, 0x56_u8, 0x87_u8]);
    let lock_script = Script::try_from(&lock_script_bytes).unwrap();

    show_display(&lock_script_bytes);
    show_display(&lock_script);

    section("Transaction input unlock script");
    message("a.k.a. ScriptSig");

    let unlock_script = [
        Element::Data(DataElement::of(&[0x03_u8])),
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

    let success = stack.evaluate().unwrap();

    message("Successful execution:");
    show_display(&success);
}
