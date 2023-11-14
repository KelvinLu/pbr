use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::transaction::TransactionInput;
use bitcoin::transaction::TransactionId;
use bitcoin::script::ScriptBytes;
use util::byte_string::ByteString;
use util::hexadecimal::hexadecimal_string;

pub fn exercise() {
    prompt("[test] Transaction input parsing");

    let txn = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    let mut txn_bytes: Vec<u8> = vec![];

    txn_bytes.resize(txn.len() / 2, 0_u8);
    hexadecimal_string(txn, &mut txn_bytes).unwrap();

    let input = TransactionInput::of(&txn_bytes[5..153]);

    show_display(&input);

    let txid = "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81";
    let mut txid_bytes: Vec<u8> = vec![];

    txid_bytes.resize(txid.len() / 2, 0_u8);
    hexadecimal_string(txid, &mut txid_bytes).unwrap();

    let script = "483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a";
    let mut script_bytes: Vec<u8> = vec![];

    script_bytes.resize(script.len() / 2, 0_u8);
    hexadecimal_string(script, &mut script_bytes).unwrap();

    assert_eq!(input.txid, TransactionId::new(&txid_bytes));
    assert_eq!(input.utxo_index, 0);
    assert_eq!(input.script, ScriptBytes::of(&script_bytes));
    assert_eq!(input.sequence, 4294967294);
}
