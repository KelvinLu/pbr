use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::transaction::Transaction;
use bitcoin::transaction::TransactionId;
use bitcoin::script::SigHashFlag;
use util::byte_string::ByteString;
use util::byte_string::ByteVector;
use util::hexadecimal::hexadecimal_string;
use util::hexadecimal::hexadecimal_encode;
use util::number::U256;

pub fn exercise() {
    prompt("[test] Sign transaction input");

    section("Signing secret");

    let secret_e = 8675309;

    message("e = ...");
    show_display(&secret_e);

    section("Transaction");

    let transaction_hexadecimal = "010000000199a24308080ab26e6fb65c4eccfadf76749bb5bfa8cb08f291320b3c21e56f0d0d00000000ffffffff02408af701000000001976a914d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f88ac80969800000000001976a914507b27411ccf7f16f10297de6cef3f291623eddf88ac00000000";
    let mut transaction_bytes: Vec<u8> = vec![];

    transaction_bytes.resize(transaction_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(transaction_hexadecimal, &mut transaction_bytes).unwrap();

    let mut transaction = Transaction::of(&transaction_bytes);

    show_pretty_print(&transaction);

    section("Previous transaction");

    let txn_0d6fe521 = "0100000001c847414138fc4e86c97bce0adfe0180d8716d0db7f43b955ebb7a80f3cbc2500000000006a47304402202f7e26dda5a70179eaa51e7a995b2fb6b3a705c59c792ae1fde3a4f4a58adaf60220406672081f8f2acfdfbeb327a5c618beb66ab226111da48ac9b150dad0d0ae52012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67ffffffff0e404b4c00000000001976a91477d946a68a9b95e851afa57006cf2d0c15ae8b3d88ac404b4c00000000001976a914325371fe093e259bdc7beca2c31f795e1b492b2088ac404b4c00000000001976a9144ccf8be232f0b1ee450a5edcc83cc4966703531388ac404b4c00000000001976a9146fe7d8cea1a39739508db7070b029d8497a0d85288ac404b4c00000000001976a91427813ea0d6e3439ffa3e30e47cd768c45bd27ab888ac404b4c00000000001976a914c16ac1981a4c73f1d51cc28f53d4757d3673a45c88ac404b4c00000000001976a9143a1806b04b0f3e14ab9b7c8cb045175d14014ac188ac404b4c00000000001976a914af39e20d8f115ecdbb3b96cda2710239e9259c5288ac404b4c00000000001976a914047357aff1cb49f6a26d71e48b88c1ba7c6ce92788ac404b4c00000000001976a9149637bebfa095f176b6cbffc79cec55fb55bf14de88ac404b4c00000000001976a9142dffa6b5f8ba2bf1ab487d1be1af9d9695350a4b88ac404b4c00000000001976a914fcf0cb53dccea9e4125a8472b8606e7f1769dad388ac404b4c00000000001976a9145a8398af0353464cf727d57a1dd79807eee50b1288ac00639f02000000001976a914d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f88ac00000000";
    let mut txn_0d6fe521_bytes: Vec<u8> = vec![];

    txn_0d6fe521_bytes.resize(txn_0d6fe521.len() / 2, 0_u8);
    hexadecimal_string(txn_0d6fe521, &mut txn_0d6fe521_bytes).unwrap();

    let transaction_0d6fe521 = Transaction::of(&txn_0d6fe521_bytes);

    message("Transaction ID:");
    show_display(&transaction_0d6fe521.txid());
    show_pretty_print(&transaction_0d6fe521);

    section("Signing the transaction");

    let retrieve_transaction_0d6fe521 = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, transaction_0d6fe521.txid());

        Some(&transaction_0d6fe521)
    };

    let input_script = transaction.signed_input_bytes_p2pkh(
        0,
        U256::from(secret_e),
        SigHashFlag::try_from(0x01_u8).unwrap(),
        retrieve_transaction_0d6fe521
    );

    transaction.inputs[0].script = input_script.unwrap();

    show_pretty_print(&transaction);

    let signed_transaction_bytes = transaction.bytes();

    let mut result_hexadecimal: Vec<u8> = vec![];

    result_hexadecimal.resize(signed_transaction_bytes.len() * 2, 0_u8);
    hexadecimal_encode(&signed_transaction_bytes, &mut result_hexadecimal).unwrap();

    message("Transaction hexadecimal:");
    message(std::str::from_utf8(&result_hexadecimal).unwrap());

    let expected_hexadecimal = "010000000199a24308080ab26e6fb65c4eccfadf76749bb5bfa8cb08f291320b3c21e56f0d0d0000006b4830450221008ed46aa2cf12d6d81065bfabe903670165b538f65ee9a3385e6327d80c66d3b502203124f804410527497329ec4715e18558082d489b218677bd029e7fa306a72236012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67ffffffff02408af701000000001976a914d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f88ac80969800000000001976a914507b27411ccf7f16f10297de6cef3f291623eddf88ac00000000";
    let mut expected_bytes: Vec<u8> = vec![];

    expected_bytes.resize(expected_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(&expected_hexadecimal, &mut expected_bytes).unwrap();

    assert_eq!(signed_transaction_bytes, expected_bytes);
}
