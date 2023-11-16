use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::signature_verification_hash;
use bitcoin::script::TransactionInputCommitment;
use bitcoin::script::DefaultOpCheckSigDigest;
use bitcoin::script::SigHashFlag;
use bitcoin::script::ScriptBytes;
use bitcoin::transaction::Transaction;
use util::byte_string::ByteString;
use util::hexadecimal::hexadecimal_string;
use util::hexadecimal::hexadecimal_encode;

pub fn exercise() {
    prompt("[test] OP_SIGHASH");

    let transaction_hexadecimal = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    let mut transaction_bytes: Vec<u8> = vec![];

    transaction_bytes.resize(transaction_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(transaction_hexadecimal, &mut transaction_bytes).unwrap();

    let transaction = Transaction::of(&transaction_bytes);
    let input_index = 0;

    show_pretty_print(&transaction);
    message("Transaction ID:");
    show_display(&transaction.txid());

    let utxo_script_hexadecimal = "76a914a802fc56c704ce87c42d7c92eb75e7896bdc41ae88ac";
    let mut utxo_script_bytes: Vec<u8> = vec![];

    utxo_script_bytes.resize(utxo_script_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(utxo_script_hexadecimal, &mut utxo_script_bytes).unwrap();

    let utxo_script_bytes = ScriptBytes::of(&utxo_script_bytes);
    let commitment = TransactionInputCommitment::P2pkhLockingScript(utxo_script_bytes);
    let sighash_flag = SigHashFlag::try_from(0x01_u8).unwrap();

    let expected_sighash_hexadecimal = "27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6";
    let mut expected_sighash_bytes: [u8; 32] = [0_u8; 32];

    hexadecimal_string(expected_sighash_hexadecimal, &mut expected_sighash_bytes).unwrap();

    let result = signature_verification_hash(&transaction, input_index, &commitment, sighash_flag, &DefaultOpCheckSigDigest {}).unwrap();
    let mut result_hexadecimal: [u8; 64] = [0_u8; 64];

    hexadecimal_encode(&result, &mut result_hexadecimal).unwrap();

    message("Modified transaction hash:");
    message(&std::str::from_utf8(&result_hexadecimal).unwrap());

    assert_eq!(result, expected_sighash_bytes)
}
