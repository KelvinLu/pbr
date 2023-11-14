use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::transaction::Locktime;
use bitcoin::transaction::LocktimeType;
use util::byte_value::ByteValue4;
use util::hexadecimal::hexadecimal_string;

pub fn exercise() {
    prompt("[test] Transaction locktime parsing");

    let txn = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    let mut txn_bytes: Vec<u8> = vec![];

    txn_bytes.resize(txn.len() / 2, 0_u8);
    hexadecimal_string(txn, &mut txn_bytes).unwrap();

    let locktime = Locktime::of(&txn_bytes[222..226]);

    show_display(&locktime);

    assert_eq!(locktime.interpretation(), LocktimeType::BlockHeight);
    assert_eq!(locktime.block_height(), 410393);
}
