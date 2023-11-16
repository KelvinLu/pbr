//! Bitcoin script transaction signing verification.

use crate::bitcoin::transaction::Transaction;
use crate::bitcoin::transaction::TransactionInput;
use crate::bitcoin::transaction::UnspentTransactionOutput;
use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptBytes;
use crate::bitcoin::script::Element;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::Opcode;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::ScriptCreationError;
use crate::bitcoin::script::opcode::CryptographicOpcode;
use crate::crypto::ecdsa::signature::Signature;
use crate::crypto::digest::hash_256;
use crate::serialization::signature::SignatureDerFormatBytes;
use crate::serialization::point::CompressedPointSecFormatBytes;
use crate::serialization::point::UncompressedPointSecFormatBytes;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteVector;
use crate::util::byte_string::ByteSlice;
use crate::util::number::U256;

/// Verifies a commitment claimed by the transaction input over an associated UTXO.
///
/// i.e.; `OP_CHECKSIG`.
pub fn signature_verification(
    signature_bytes: &DataElement,
    public_key_bytes: &DataElement,
    transaction: &Transaction,
    input_index: usize,
    commitment: &TransactionInputCommitment,
    checksig_digest: &dyn OpCheckSigDigest,
) -> Result<bool, ScriptError> {
    let signature_bytes = signature_bytes.bytes();
    let public_key_bytes = public_key_bytes.bytes();

    let sighash_byte = *signature_bytes.last()
       .ok_or(opchecksig_error())?;
    let sighash_flag = SigHashFlag::try_from(sighash_byte)
       .map_err(|_| opchecksig_error())?;

    let signature_format = SignatureDerFormatBytes::of(&signature_bytes[0..(signature_bytes.len() - 1)]);
    let signature = Signature::from(signature_format.clone());

    let point = match public_key_bytes.first().ok_or(opchecksig_error())? {
        0x02_u8 | 0x03_u8 => {
            CompressedPointSecFormatBytes::of(&public_key_bytes).elliptic_curve_point_secp256k1().map_err(|_| opchecksig_error())?
        },
        0x04_u8 => {
            UncompressedPointSecFormatBytes::of(&public_key_bytes).elliptic_curve_point_secp256k1().map_err(|_| opchecksig_error())?
        },
        _ => return Err(opchecksig_error()),
    };

    let digest = signature_verification_hash(
        transaction,
        input_index,
        commitment,
        sighash_flag,
        checksig_digest,
    ).map_err(|_| opchecksig_error())?;

    let data = U256::from_be_bytes(digest);

    Ok(signature.verify_point_secp256k1(data, point))
}

/// Provides a digest for `OP_CHECKSIG` to use.
pub trait OpCheckSigDigest {
    fn digest(&self, bytes: &[u8]) -> [u8; 32];
}

/// Default `OP_CHECKSIG` behavior (double SHA-256).
pub struct DefaultOpCheckSigDigest;

impl OpCheckSigDigest for DefaultOpCheckSigDigest {
    fn digest(&self, bytes: &[u8]) -> [u8; 32] {
        hash_256(bytes)
    }
}

/// Provides commitment material used in replacing a transaction input's script when the transaction
/// is modified as part of creating its `OP_CHECKSIG` signature.
pub enum TransactionInputCommitment<'a> {
    /// Parses the current script in the context of an actual execution.
    ///
    /// Applies `OP_CODESEPARATOR` truncation for already executed segments.
    ScriptCode(&'a Script, usize),

    /// The commitment is made against a UTXO's P2PKH locking script.
    P2pkhLockingScript(ScriptBytes),

    /// P2SH support. The commitment is made against the redeem script.
    RedeemScript(ScriptBytes),
}

/// Provides the "double SHA-256" digest (`hash_256`) for signature checking opcodes to verify
/// against.
///
/// This operation is performed for a given transaction input that signs a previous transaction's UTXO.
///
/// It will reconstruct a clone of the given transaction, applying any processing rules ...
///
/// - `SIGHASH` flags to modify commitments.
/// - `OP_CODESEPARATOR` segments for splicing scripts.
///
/// ... and then manipulate the cloned transaction to be used as the digest material.
///
/// Returns the digest taken against the cloned, modified transaction's byte representation.
pub fn signature_verification_hash(
    transaction: &Transaction,
    input_index: usize,
    commitment: &TransactionInputCommitment,
    sighash: SigHashFlag,
    checksig_digest: &dyn OpCheckSigDigest,
) -> Result<[u8; 32], ScriptCreationError> {
    // Return default hash on erroneous input index.
    if input_index > transaction.inputs.len() {
        return Ok(checksig_digest.digest(&U256::from(1).to_le_bytes::<32>()));
    }

    // Perform "OP_CODESEPARATOR" truncation, if necessary.
    let is_code_separator = |element: &Element| {
        match element {
            Element::Opcode(Opcode::Cryptographic(CryptographicOpcode::OpCodeSeparator)) => true,
            _ => false
        }
    };

    // Commitment bytes, used to overwrite the transaction input script when signing against a
    // transaction input.
    let (commitment_script_bytes, skip_elements) = match commitment {
        TransactionInputCommitment::ScriptCode(execution_script, instruction_pointer) => {
            if let Some(n) = execution_script.elements().iter().take(*instruction_pointer).rposition(is_code_separator) {
                (ScriptBytes::from(*execution_script), n + 1)
            } else {
                (ScriptBytes::from(*execution_script), 0)
            }
        },
        TransactionInputCommitment::P2pkhLockingScript(script_bytes) => (script_bytes.clone(), 0),
        TransactionInputCommitment::RedeemScript(script_bytes) => (script_bytes.clone(), 0),
    };

    let commitment_script = Script::try_from(&commitment_script_bytes)?;
    let commitment_script_without_code_separators = commitment_script
        .elements()
        .iter()
        .skip(skip_elements)
        .filter_map(|element| if is_code_separator(element) { None } else { Some(element) });

    let mut commitment_elements = vec![];

    for element in commitment_script_without_code_separators {
        commitment_elements.push(element.clone());
    }

    let commitment_bytes = ScriptBytes::from(&Script::new(&commitment_elements)?);

    // Initalize a copy of the transaction input(s).
    let sighash_inputs: Vec<TransactionInput> = if sighash.anyonecanpay() {
        // If "SIGHASH_ANYONECANPAY", select only the signing transaction input ...
        let mut input = transaction.inputs[input_index].clone();

        // Replace the signing transaction input script with the commitment script.
        // e.g; P2PKH UTXO locking script, P2SH redeem script ...
        input.script = commitment_bytes;

        [input].to_vec()
    } else {
        let sighash_type = sighash.sighash_type();

        // If not "SIGHASH_ANYONECANPAY", ...
        let clone_inputs = |(i, input): (usize, &TransactionInput)| {
            let mut input = input.clone();

            // ... for all non-signing transaction inputs ...
            if i != input_index {
                // ... assign an empty script.
                input.script = ScriptBytes::of(&[]);

                match sighash_type {
                    SigHashType::SigHashNone | SigHashType::SigHashSingle => {
                        // ... for "SIGHASH_NONE"/"SIGHASH_SINGLE", set sequence numbers to zero.
                        input.sequence = 0;
                    },
                    _ => (),
                }
            }

            input
        };

        let mut inputs: Vec<TransactionInput> = transaction.inputs.iter().enumerate().map(clone_inputs).collect();

        // Replace the signing transaction input script with the commitment script.
        // e.g; P2PKH UTXO locking script, P2SH redeem script ...
        inputs[input_index].script = commitment_bytes;

        inputs
    };

    // Initalize a copy of the transaction UTXO(s).
    let sighash_utxos: Vec<UnspentTransactionOutput> = match sighash.sighash_type() {
        SigHashType::SigHashAll => transaction.utxos.clone(), // "SIGHASH_ALL" -- commit all UTXOs.
        SigHashType::SigHashNone => vec![], // "SIGHASH_NONE" -- commit none.
        SigHashType::SigHashSingle => {
            // "SIGHASH_SINGLE" -- commit UTXO with matching signing transaction input index.
            if input_index > transaction.utxos.len() {
                return Ok(checksig_digest.digest(&U256::from(1).to_le_bytes::<32>()));
            }

            let mut utxos = transaction.utxos[0..=input_index].to_vec();

            // For non-signing transaction inputs, set amount to -1 and assign an empty script.
            for utxo in utxos[0..input_index].iter_mut() {
                *utxo = utxo.clone();

                (*utxo).amount = -1;
                (*utxo).script = ScriptBytes::of(&[]);
            }

            utxos
        },
    };

    // Compute the resulting bytes.
    let transaction_copy = Transaction {
        version: transaction.version,
        inputs: sighash_inputs,
        utxos: sighash_utxos,
        locktime: transaction.locktime,
    };

    let mut bytes = transaction_copy.bytes();

    // Append the four byte little-endian "SIGHASH".
    bytes.extend_from_slice(&sighash.to_le_bytes());

    Ok(checksig_digest.digest(&bytes))
}

/// Provides the "double SHA-256" digest (`hash_256`) for transaction input signing.
///
/// This operation is performed for a given transaction input that signs a commitment relevant to a
/// previous transaction's UTXO.
///
/// It will reconstruct a clone of the given transaction, applying any processing rules ...
///
/// - `SIGHASH` flags to modify commitments.
/// - `OP_CODESEPARATOR` segments for splicing scripts.
///
/// ... and then manipulate the cloned transaction to be used as the digest material.
///
/// Returns the digest taken against the cloned, modified transaction's byte representation.
pub fn signature_signing_hash(
    transaction: &Transaction,
    input_index: usize,
    commitment: &TransactionInputCommitment,
    sighash: SigHashFlag,
    checksig_digest: &dyn OpCheckSigDigest,
) -> Result<[u8; 32], ScriptCreationError> {
    signature_verification_hash(
        transaction,
        input_index,
        &commitment,
        sighash,
        checksig_digest,
    )
}

fn opchecksig_error() -> ScriptError {
    ScriptError::OpcodeFailed(Opcode::Cryptographic(CryptographicOpcode::OpCheckSig))
}

/// A `SIGHASH` flag, represented as a byte.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SigHashFlag {
    byte: u8
}

/// `SIGHASH` types.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum SigHashType {
    /// `0x01`: `ALL`
    SigHashAll,

    /// `0x02`: `NONE`
    SigHashNone,

    /// `0x03`: `SINGLE`
    SigHashSingle,
}

#[derive(Debug)]
pub struct SigHashFlagError;

impl TryFrom<u8> for SigHashFlag {
    type Error = SigHashFlagError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        // Check that the last two bits only represent the three SIGHASH types (0x01, 0x02, 0x03).
        // 0x03 = 0b00000011
        if byte & 0x03_u8 == 0x00_u8 { return Err(SigHashFlagError) }

        // Only the first bit, or the last two bits may be set.
        // 0x7c = 0b01111100
        if byte & 0x7c_u8 > 0x00_u8 { return Err(SigHashFlagError) }

        Ok(Self { byte: byte })
    }
}

impl SigHashFlag {
    /// Returns the byte of the `SIGHASH` flag.
    pub fn byte(&self) -> u8 {
        self.byte
    }

    /// Returns the little-endian four byte represenation of the `SIGHASH` flag.
    pub fn to_le_bytes(&self) -> [u8; 4] {
        [self.byte, 0_u8, 0_u8, 0_u8]
    }

    /// Returns the `SigHashType` for a given `SigHashFlag`.
    pub fn sighash_type(&self) -> SigHashType {
        match self.byte & 0x03_u8 {
            0x01_u8 => SigHashType::SigHashAll,
            0x02_u8 => SigHashType::SigHashNone,
            0x03_u8 => SigHashType::SigHashSingle,
            _ => panic!("unexpected bits")
        }
    }

    /// Returns a boolean denoting if modifier `ANYONECANPAY` is set or not.
    pub fn anyonecanpay(&self) -> bool {
        self.byte & 0x80_u8 == 0x80_u8
    }
}
