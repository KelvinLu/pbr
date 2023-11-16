use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::transaction::Transaction;
use bitcoin::transaction::TransactionId;
use util::byte_string::ByteString;
use util::hexadecimal::hexadecimal_string;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("[test] Verify P2PKH");

    section("Transaction");

    let transaction_hexadecimal = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    let mut transaction_bytes: Vec<u8> = vec![];

    transaction_bytes.resize(transaction_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(transaction_hexadecimal, &mut transaction_bytes).unwrap();

    let transaction = Transaction::of(&transaction_bytes);

    show_pretty_print(&transaction);

    section("Previous transaction");

    let txn_d1c789a9 = "0100000002137c53f0fb48f83666fcfd2fe9f12d13e94ee109c5aeabbfa32bb9e02538f4cb000000006a47304402207e6009ad86367fc4b166bc80bf10cf1e78832a01e9bb491c6d126ee8aa436cb502200e29e6dd7708ed419cd5ba798981c960f0cc811b24e894bff072fea8074a7c4c012103bc9e7397f739c70f424aa7dcce9d2e521eb228b0ccba619cd6a0b9691da796a1ffffffff517472e77bc29ae59a914f55211f05024556812a2dd7d8df293265acd8330159010000006b483045022100f4bfdb0b3185c778cf28acbaf115376352f091ad9e27225e6f3f350b847579c702200d69177773cd2bb993a816a5ae08e77a6270cf46b33f8f79d45b0cd1244d9c4c0121031c0b0b95b522805ea9d0225b1946ecaeb1727c0b36c7e34165769fd8ed860bf5ffffffff027a958802000000001976a914a802fc56c704ce87c42d7c92eb75e7896bdc41ae88aca5515e00000000001976a914e82bd75c9c662c3f5700b33fec8a676b6e9391d588ac00000000";
    let mut txn_d1c789a9_bytes: Vec<u8> = vec![];

    txn_d1c789a9_bytes.resize(txn_d1c789a9.len() / 2, 0_u8);
    hexadecimal_string(txn_d1c789a9, &mut txn_d1c789a9_bytes).unwrap();

    let transaction_d1c789a9 = Transaction::of(&txn_d1c789a9_bytes);

    message("Transaction ID:");
    show_display(&transaction_d1c789a9.txid());
    show_pretty_print(&transaction_d1c789a9);

    let retrieve_transaction_d1c789a9 = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, transaction_d1c789a9.txid());

        Some(&transaction_d1c789a9)
    };

    section("Verification result");

    let result = transaction.verify(EXAMPLE_TIMESTAMP, EXAMPLE_BLOCK_HEIGHT, retrieve_transaction_d1c789a9).unwrap();

    show_debug(&result);

    assert!(result);
}
