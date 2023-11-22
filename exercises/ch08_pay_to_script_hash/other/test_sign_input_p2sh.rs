use programming_bitcoin_in_rust::*;

use bitcoin::transaction::Transaction;
use bitcoin::transaction::TransactionId;
use bitcoin::transaction::Version;
use bitcoin::transaction::TransactionInput;
use bitcoin::transaction::UnspentTransactionOutput;
use bitcoin::transaction::Locktime;
use bitcoin::script::Script;
use bitcoin::script::ScriptBytes;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::SigHashFlag;
use bitcoin::script::opcode::ConstantOpcode;
use bitcoin::script::opcode::CryptographicOpcode;
use crypto::secp256k1::Secp256k1Point;
use crypto::digest::hash_256;
use math::elliptic_curve::point::EllipticCurvePoint;
use serialization::point::CompressedPointSecFormatBytes;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::byte_value::ByteValue4;
use util::hexadecimal::hexadecimal_string;
use util::number::U256;

use crate::util::bitcoin::script::context::*;

pub fn run() {
    let secret_e_1 = U256::from_be_bytes(hash_256(b"my little secret"));
    let secret_e_2 = U256::from_be_bytes(hash_256(b"https://www.youtube.com/watch?v=uc6f_2nPSX8&t=60s"));

    let point_1: EllipticCurvePoint = (secret_e_1 * Secp256k1Point::generator_point()).into();
    let point_bytes_1 = CompressedPointSecFormatBytes::from(&point_1);

    let point_2: EllipticCurvePoint = (secret_e_2 * Secp256k1Point::generator_point()).into();
    let point_bytes_2 = CompressedPointSecFormatBytes::from(&point_2);

    let funding_transaction_utxo_index = 1;
    let amount_satoshi_lock_by_redeem_script = 2600;
    let amount_satoshi_send_to_1 = 1100;
    let amount_satoshi_send_to_2 = 1100;

    let funding_transaction_hexadecimal = "01000000019143f7d73f3d962bec000467402fd6182a8a2494affccce089ffa42d917fdcc3000000006b483045022100c7093df1d571fc1a6722b2949c52e232f4d8cf181bd92f33e9c987614a58a11402200b12e9aae73b3ebbea2ca1cabdffd2c7000ac38bb7786390b90b643aac9968840121033dc532d6b802a7e1a3f21ca9ea664d6257e5644e681bdaeb18c787983b694589ffffffff0288130000000000001976a914ad346f8eb57dee9a37981716e498120ae80e44f788acf00a0000000000001976a914205521139e3be3b50e31541ea08ddde1236415e088ac00000000";
    let mut funding_transaction_bytes: Vec<u8> = vec![];

    funding_transaction_bytes.resize(funding_transaction_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(&funding_transaction_hexadecimal, &mut funding_transaction_bytes).unwrap();

    let funding_transaction = Transaction::of(&funding_transaction_bytes);

    let redeem_script = Script::new(&[
        Element::Opcode(Opcode::Constant(ConstantOpcode::Op2)),
        Element::Data(DataElement::of(&point_bytes_1.bytes())),
        Element::Data(DataElement::of(&point_bytes_2.bytes())),
        Element::Opcode(Opcode::Constant(ConstantOpcode::Op2)),
        Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisig)),
    ]).unwrap();
    let redeem_script_bytes = ScriptBytes::from(&redeem_script);

    let mut p2sh_locking_transaction = Transaction {
        version: Version::of(&[0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]),
        inputs: vec![
            TransactionInput::new(
                funding_transaction.txid(),
                funding_transaction_utxo_index,
                ScriptBytes::of(&[]),
                u32::MAX
            ),
        ],
        utxos: vec![
            UnspentTransactionOutput::new(
                amount_satoshi_lock_by_redeem_script,
                ScriptBytes::locking_script_p2sh_redeem_script(&redeem_script_bytes),
            ),
        ],
        locktime: Locktime::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8])
    };

    let retrieve_funding_txn = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, funding_transaction.txid());

        Some(&funding_transaction)
    };

    let input_script = p2sh_locking_transaction.signed_input_bytes_p2pkh(
        0,
        U256::from(secret_e_1),
        SigHashFlag::try_from(0x01_u8).unwrap(),
        retrieve_funding_txn
    );

    p2sh_locking_transaction.inputs[0].script = input_script.unwrap();

    let mut p2sh_unlocking_transaction = Transaction {
        version: Version::of(&[0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]),
        inputs: vec![
            TransactionInput::new(
                p2sh_locking_transaction.txid(),
                0,
                ScriptBytes::of(&[]),
                u32::MAX
            ),
        ],
        utxos: vec![
            UnspentTransactionOutput::new(
                amount_satoshi_send_to_1,
                ScriptBytes::locking_script_p2pkh_compressed_point(&point_bytes_1),
            ),
            UnspentTransactionOutput::new(
                amount_satoshi_send_to_2,
                ScriptBytes::locking_script_p2pkh_compressed_point(&point_bytes_2),
            ),
        ],
        locktime: Locktime::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8])
    };

    let signature_input_1 = p2sh_unlocking_transaction.signed_input_bytes_p2sh(
        0,
        secret_e_1,
        &redeem_script_bytes,
        SigHashFlag::try_from(0x01_u8).unwrap()
    ).unwrap();

    let signature_input_2 = p2sh_unlocking_transaction.signed_input_bytes_p2sh(
        0,
        secret_e_2,
        &redeem_script_bytes,
        SigHashFlag::try_from(0x01_u8).unwrap()
    ).unwrap();

    let reedem_script_arguments = Script::new(&[Element::Opcode(Opcode::Constant(ConstantOpcode::OpFalse))])
        .unwrap()
        .concatenate(&Script::try_from(&signature_input_1.concatenate(&signature_input_2)).unwrap())
        .unwrap();

    let retrieve_locking_txn = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, p2sh_locking_transaction.txid());

        Some(&p2sh_locking_transaction)
    };

    let input_script = p2sh_unlocking_transaction.redeem_script_input_bytes_p2sh(
        0,
        Some(&ScriptBytes::from(&reedem_script_arguments)),
        &redeem_script_bytes,
        retrieve_locking_txn
    );

    p2sh_unlocking_transaction.inputs[0].script = input_script.unwrap();

    let retrieve_locking_txn = |transaction_id: &TransactionId| {
        assert_eq!(*transaction_id, p2sh_locking_transaction.txid());

        Some(&p2sh_locking_transaction)
    };

    let result = p2sh_unlocking_transaction.verify(EXAMPLE_TIMESTAMP, EXAMPLE_BLOCK_HEIGHT, retrieve_locking_txn).unwrap();

    assert!(result);
}
