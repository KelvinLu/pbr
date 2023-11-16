#![allow(dead_code)]

use programming_bitcoin_in_rust::*;

use bitcoin::script::ScriptExecutionContext;
use bitcoin::transaction::Transaction;
use bitcoin::transaction::Version;
use bitcoin::transaction::Locktime;
use util::byte_value::ByteValue4;

pub const EXAMPLE_TIMESTAMP: u64 = 1700000000;
pub const EXAMPLE_BLOCK_HEIGHT: u64 = 800000;

pub fn example_empty_transaction() -> Transaction {
    Transaction {
        version: Version::of(&[0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]),
        inputs: vec![],
        utxos: vec![],
        locktime: Locktime::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8]),
    }
}

pub fn example_script_execution_context<'a>(transaction: &'a Transaction) -> ScriptExecutionContext<'a> {
    ScriptExecutionContext::new(transaction, 0, EXAMPLE_TIMESTAMP, EXAMPLE_BLOCK_HEIGHT)
}
