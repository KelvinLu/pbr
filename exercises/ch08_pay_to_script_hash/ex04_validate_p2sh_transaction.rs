use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use bitcoin::script::signature_verification_hash;
use bitcoin::script::TransactionInputCommitment;
use bitcoin::script::DefaultOpCheckSigDigest;
use bitcoin::script::SigHashFlag;
use bitcoin::script::Script;
use bitcoin::script::ScriptBytes;
use bitcoin::script::Element;
use bitcoin::transaction::Transaction;
use crypto::ecdsa::signature::Signature;
use serialization::signature::SignatureDerFormatBytes;
use serialization::point::CompressedPointSecFormatBytes;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;
use util::hexadecimal::hexadecimal_string;
use util::hexadecimal::hexadecimal_encode;
use util::number::U256;

pub fn exercise() {
    prompt("Validate P2SH transaction");

    section("Transaction");

    let transaction_hexadecimal = "0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000db00483045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701483045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c568700000000";
    let mut transaction_bytes: Vec<u8> = vec![];

    transaction_bytes.resize(transaction_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(transaction_hexadecimal, &mut transaction_bytes).unwrap();

    let transaction = Transaction::of(&transaction_bytes);
    let input_index = 0;

    show_pretty_print(&transaction);

    section("Redeem script");

    let input_script_hexadecimal = "475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152ae";
    let mut input_script_bytes: Vec<u8> = vec![];

    input_script_bytes.resize(input_script_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(input_script_hexadecimal, &mut input_script_bytes).unwrap();

    let input_script_bytes = ScriptBytes::of(&input_script_bytes);
    let input_script = Script::try_from(&input_script_bytes).unwrap();

    message("Input script (places redeem script onto stack as data element)");
    show_display(&input_script);

    let Element::Data(redeem_script_data_element) = &input_script.elements()[0] else { panic!("expected a data element") };

    let redeem_script_bytes = ScriptBytes::of(&redeem_script_data_element.bytes());
    let redeem_script = Script::try_from(&redeem_script_bytes).unwrap();

    message("The redeem script itself");
    show_display(&redeem_script);

    section("Signature and public key");

    let signature_hexadecimal = "3045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e754022";
    let mut signature_bytes: Vec<u8> = vec![];

    signature_bytes.resize(signature_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(signature_hexadecimal, &mut signature_bytes).unwrap();

    let point_hexadecimal = "03b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb71";
    let mut point_bytes: Vec<u8> = vec![];

    point_bytes.resize(point_hexadecimal.len() / 2, 0_u8);
    hexadecimal_string(point_hexadecimal, &mut point_bytes).unwrap();

    let signature_bytes = SignatureDerFormatBytes::of(&signature_bytes);
    let point_bytes = CompressedPointSecFormatBytes::of(&point_bytes);

    show_display(&signature_bytes);
    show_display(&point_bytes);

    let signature = Signature::from(signature_bytes);
    let point = point_bytes.elliptic_curve_point_secp256k1().unwrap();

    message("...");
    show_debug(&signature);
    show_debug(&point);

    section("Transaction signing with redeem script");

    let commitment = TransactionInputCommitment::RedeemScript(redeem_script_bytes);
    let sighash_flag = SigHashFlag::try_from(0x01_u8).unwrap();

    let transaction_sighash = signature_verification_hash(&transaction, input_index, &commitment, sighash_flag, &DefaultOpCheckSigDigest {}).unwrap();
    let data = U256::from_be_bytes(transaction_sighash);

    let mut transaction_sighash_hexadecimal = [0_u8; 64];
    hexadecimal_encode(&transaction_sighash, &mut transaction_sighash_hexadecimal).unwrap();

    message("Transaction signature hash");
    message(&std::str::from_utf8(&transaction_sighash_hexadecimal).unwrap());

    let result = signature.verify_point_secp256k1(data, point);

    message("Verified:");
    show_debug(&result);
}
