//! Opcodes that perform cryptographic operations.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::Opcode;
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

const OP_CHECKMULTISIG_ERROR: ScriptError = ScriptError::OpcodeFailed(Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisig));

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
                CryptographicOpcode::OpCheckMultisig => {
                    let mut public_keys: Vec<DataElement> = vec![];
                    let mut signatures: Vec<DataElement> = vec![];
                    let commitment = TransactionInputCommitment::ScriptCode(script, instruction_pointer);

                    let public_key_count = stack.get_data_element()?.number()?;

                    if public_key_count < 1 {
                        return Err(OP_CHECKMULTISIG_ERROR);
                    }

                    for _ in 0..public_key_count { public_keys.push(stack.get_data_element()?) }

                    let signature_count = stack.get_data_element()?.number()?;

                    if (signature_count < 1) || (signature_count > public_key_count) {
                        return Err(OP_CHECKMULTISIG_ERROR);
                    }

                    for _ in 0..signature_count { signatures.push(stack.get_data_element()?) }

                    // Consume the "OP_0" data element.
                    if !stack.get_data_element()?.bytes().is_empty() {
                        return Err(OP_CHECKMULTISIG_ERROR);
                    }

                    let mut n = 0; // Public key index

                    for signature in signatures {
                        loop {
                            let Some(public_key) = public_keys.get(n) else {
                                stack.push(DataElement::of(&[0_u8]));

                                return Ok(());
                            };

                            n += 1;

                            if signature_verification(
                                &signature,
                                &public_key,
                                &context.transaction,
                                context.input_index,
                                &commitment,
                                context.checksig_digest,
                            ).map_err(|_| OP_CHECKMULTISIG_ERROR)? {
                                break;
                            }
                        }
                    }

                    stack.push(DataElement::of(&[1_u8]));
                },
                CryptographicOpcode::OpCheckMultisigVerify => todo!(),
                _ => panic!("unexpected opcode")
            };
        },
        CryptographicOpcode::OpCodeSeparator => (),
        _ => todo!(),
    }

    Ok(())
}
