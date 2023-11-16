//! Opcodes that perform cryptographic operations.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::TransactionInputCommitment;
use crate::bitcoin::script::signature_verification;
use crate::bitcoin::script::stack::GetDataElement;
use crate::bitcoin::script::opcode::CryptographicOpcode;
use crate::bitcoin::script::opcode::FlowControlOpcode;
use crate::bitcoin::script::opcode::opcode_flowcontrol;
use crate::crypto::digest::sha_1;
use crate::crypto::digest::sha_256;
use crate::crypto::digest::hash_160;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;

pub fn opcode_cryptographic(
    stack: &mut Vec<DataElement>,
    script: &Script,
    instruction_pointer: usize,
    context: &ScriptExecutionContext,
    opcode: CryptographicOpcode
) -> Result<(), ScriptError> {
    match opcode {
        | CryptographicOpcode::OpRipemd160
        | CryptographicOpcode::OpSha1
        | CryptographicOpcode::OpSha256
        | CryptographicOpcode::OpHash160
        | CryptographicOpcode::OpHash256
        => {
            let data = stack.get_data_element()?;
            let mut digest: Vec<u8> = vec![];

            match opcode {
                CryptographicOpcode::OpRipemd160 => todo!(),
                CryptographicOpcode::OpSha1 => digest.extend_from_slice(&sha_1(data.bytes())),
                CryptographicOpcode::OpSha256 => digest.extend_from_slice(&sha_256(data.bytes())),
                CryptographicOpcode::OpHash160 => digest.extend_from_slice(&hash_160(data.bytes())),
                CryptographicOpcode::OpHash256 => todo!(),
                _ => panic!("unexpected opcode")
            }

            stack.push(DataElement::of(&digest));
        },
        | CryptographicOpcode::OpCheckSig
        | CryptographicOpcode::OpCheckSigVerify
        | CryptographicOpcode::OpCheckMultisig
        | CryptographicOpcode::OpCheckMultisigVerify
        => {
            match opcode {
                CryptographicOpcode::OpCheckSig => {
                    let public_key_bytes = stack.get_data_element()?;
                    let signature_bytes = stack.get_data_element()?;
                    let commitment = TransactionInputCommitment::ScriptCode(script, instruction_pointer);

                    match signature_verification(
                        &signature_bytes,
                        &public_key_bytes,
                        &context.transaction,
                        context.input_index,
                        &commitment,
                        context.checksig_digest,
                    )? {
                        true => stack.push(DataElement::of(&[1_u8])),
                        false => stack.push(DataElement::of(&[0_u8])),
                    }
                },
                CryptographicOpcode::OpCheckSigVerify => {
                    opcode_cryptographic(stack, script, instruction_pointer, context, CryptographicOpcode::OpCheckSig)?;
                    opcode_flowcontrol(stack, script, instruction_pointer, context, FlowControlOpcode::OpVerify)?;
                },
                CryptographicOpcode::OpCheckMultisig => todo!(),
                CryptographicOpcode::OpCheckMultisigVerify => todo!(),
                _ => panic!("unexpected opcode")
            };
        },
        CryptographicOpcode::OpCodeSeparator => (),
        _ => todo!(),
    }

    Ok(())
}
