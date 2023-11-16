use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::transaction::Transaction;
use bitcoin::transaction::TransactionId;
use bitcoin::transaction::TransactionInput;
use bitcoin::transaction::UnspentTransactionOutput;
use bitcoin::transaction::Version;
use bitcoin::transaction::Locktime;
use bitcoin::segwit::transaction::SegWitTransaction;
use bitcoin::script::ScriptBytes;
use bitcoin::script::SigHashFlag;
use bitcoin::network::BitcoinNetworkType;
use crypto::secp256k1::Secp256k1Point;
use crypto::digest::hash_256;
use serialization::bitcoin_address::BitcoinAddress;
use serialization::point::CompressedPointSecFormatBytes;
use math::elliptic_curve::point::EllipticCurvePoint;
use util::byte_string::ByteString;
use util::byte_string::ByteVector;
use util::byte_value::ByteValue4;
use util::hexadecimal::hexadecimal_string;
use util::hexadecimal::hexadecimal_encode;
use util::number::U256;

use crate::util::bitcoin::script::context::*;

pub fn exercise() {
    prompt("Testnet faucet exercise");

    message("See chapter 4, exercise 9 for previously related exercise.");

    section("Our address");

    let secret_e = U256::from_be_bytes(hash_256(b"my little secret"));

    message("(SHA-256(SHA-256(<secret phrase>))) * <secp256k1 G>");

    let point: EllipticCurvePoint = (secret_e * Secp256k1Point::generator_point()).into();
    let point_bytes = CompressedPointSecFormatBytes::from(&point);
    let address = BitcoinAddress::for_compressed_point(BitcoinNetworkType::Testnet, point_bytes);

    show_display(&address);
    show_debug(&address.network());
    show_debug(&address.transaction_type());

    section("Target address");

    let target_address_base58 = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv";
    let target_address = BitcoinAddress::base58(&target_address_base58);

    show_display(&target_address);
    show_debug(&target_address.network());
    show_debug(&target_address.transaction_type());

    section("Prerequisite funding");

    message(&format!("Send some testnet faucet funds to {}", address));
    message("... and then acquire the funding transaction bytes");

    let funding_txn_utxo_index: u32 = 0;
    let amount_satoshi_send_to_target: i64 = 5000;
    let amount_satoshi_return_as_change: i64 = 2800;

    let funding_txn_hexadecimal = "02000000000101f272171ba585aeb3ff04104d379fe3afbec87da17cca75fb27e684ce17d8a30400000000171600141897d318265c6b0695001b1d6821c80169e66de7fdffffff0251210000000000001976a914205521139e3be3b50e31541ea08ddde1236415e088ac9a230900000000001976a914081cd6752551926cd6b8f94c570ae0c88d4d3c0e88ac024730440220386b2f67320ed73bc741379a4d603f3aa48ff62d61f24e56f8e6bab72ee2c4f302203bf0655f459e6075a1514d93545081b841b13301eedf09e48d9c3852d27af7df01210353a3175881e9e3efe57e9447bd82cbf9e08b160d45ab06d4d16329ed5303c14112c02600";
    let mut funding_txn_bytes: Vec<u8> = vec![];

    funding_txn_bytes.resize(funding_txn_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(&funding_txn_hexadecimal, &mut funding_txn_bytes).unwrap();

    let funding_txn = SegWitTransaction::of(&funding_txn_bytes);

    message("...");
    message("Funding transaction ID:");
    show_display(&funding_txn.txid());
    message("...");
    message("Funding transaction:");
    show_pretty_print(&funding_txn);

    section("Constructing a transaction");

    message(&format!("Then, use a transaction to send {} satoshi to {} ...", amount_satoshi_send_to_target, target_address));
    message(&format!("... and send the remaining {} satoshi as returned change to {}", amount_satoshi_return_as_change, address));

    let inputs: Vec<TransactionInput> = vec![
        TransactionInput::new(
            funding_txn.txid(),
            funding_txn_utxo_index,
            ScriptBytes::of(&[]),
            u32::MAX
        ),
    ];

    let utxos: Vec<UnspentTransactionOutput> = vec![
        UnspentTransactionOutput::new(
            amount_satoshi_send_to_target,
            ScriptBytes::locking_script_p2pkh_address(&target_address),
        ),
        UnspentTransactionOutput::new(
            amount_satoshi_return_as_change,
            ScriptBytes::locking_script_p2pkh_compressed_point(&point_bytes),
        ),
    ];

    let mut transaction = Transaction {
        version: Version::of(&[0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]),
        inputs: inputs,
        utxos: utxos,
        locktime: Locktime::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8])
    };

    let retrieve_funding_txn = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, funding_txn.txid());

        Some(&*funding_txn)
    };

    let input_script = transaction.signed_input_bytes_p2pkh(
        0,
        U256::from(secret_e),
        SigHashFlag::try_from(0x01_u8).unwrap(),
        retrieve_funding_txn
    );

    transaction.inputs[0].script = input_script.unwrap();

    message("...");
    message("Our transaction ID:");
    show_display(&transaction.txid());
    message("...");
    message("Our transaction:");
    show_pretty_print(&transaction);

    let retrieve_funding_txn = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, funding_txn.txid());

        Some(&*funding_txn)
    };

    assert!(transaction.verify(EXAMPLE_TIMESTAMP, EXAMPLE_BLOCK_HEIGHT, retrieve_funding_txn).unwrap());

    let transaction_bytes = &transaction.bytes();
    let mut transaction_hexadecimal: Vec<u8> = vec![];

    transaction_hexadecimal.resize(transaction_bytes.len() * 2, 0_u8);
    hexadecimal_encode(transaction_bytes, &mut transaction_hexadecimal).unwrap();

    message("...");
    message("Transaction bytes:");
    message(&std::str::from_utf8(&transaction_hexadecimal).unwrap());
}
