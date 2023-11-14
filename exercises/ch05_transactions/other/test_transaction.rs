use programming_bitcoin_in_rust::*;

use bitcoin::transaction::Transaction;
use bitcoin::transaction::TransactionId;
use bitcoin::transaction::LocktimeType;
use bitcoin::script::ScriptBytes;
use util::byte_string::ByteString;
use util::byte_string::ByteVector;
use util::hexadecimal::hexadecimal_string;

pub fn run() {
    let txn = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

    let mut txn_bytes: Vec<u8> = vec![];

    txn_bytes.resize(txn.len() / 2, 0_u8);
    hexadecimal_string(txn, &mut txn_bytes).unwrap();

    let transaction = Transaction::of(&txn_bytes);

    assert_eq!(transaction.version.value(), 1);

    assert_eq!(transaction.inputs.len(), 4);

    let txid = "d37f9e7282f81b7fd3af0fde8b462a1c28024f1d83cf13637ec18d03f4518feb";
    let mut txid_bytes: Vec<u8> = vec![];

    txid_bytes.resize(txid.len() / 2, 0_u8);
    hexadecimal_string(txid, &mut txid_bytes).unwrap();

    let script = "47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
    let mut script_bytes: Vec<u8> = vec![];

    script_bytes.resize(script.len() / 2, 0_u8);
    hexadecimal_string(script, &mut script_bytes).unwrap();

    let input = &transaction.inputs[1];

    assert_eq!(input.txid, TransactionId::new(&txid_bytes));
    assert_eq!(input.utxo_index, 0);
    assert_eq!(input.script, ScriptBytes::of(&script_bytes));
    assert_eq!(input.sequence, 4294967294);

    assert_eq!(transaction.utxos.len(), 2);

    let script = "76a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac";
    let mut script_bytes: Vec<u8> = vec![];

    script_bytes.resize(script.len() / 2, 0_u8);
    hexadecimal_string(script, &mut script_bytes).unwrap();

    let utxo = &transaction.utxos[0];

    assert_eq!(utxo.amount, 1000273);
    assert_eq!(utxo.script, ScriptBytes::of(&script_bytes));

    assert_eq!(transaction.locktime.interpretation(), LocktimeType::BlockHeight);
    assert_eq!(transaction.locktime.block_height(), 410438);

    assert_eq!(transaction.bytes(), txn_bytes);
}
