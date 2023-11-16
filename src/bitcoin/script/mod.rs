//! The Bitcoin script language.

mod script_bytes;
mod script;
mod stack;
mod data_element;
mod signature_verification;

pub mod opcode;

pub use script_bytes::ScriptBytes;
pub use script::Script;
pub use script::ScriptExecutionContext;
pub use script::ScriptCreationError;
pub use script::ScriptError;
pub use script::Element;
pub use data_element::DataElement;
pub use opcode::Opcode;
pub use opcode::call_opcode;
pub use stack::Stack;
pub use stack::GetDataElement;
pub use signature_verification::signature_verification;
pub use signature_verification::signature_verification_hash;
pub use signature_verification::signature_signing_hash;
pub use signature_verification::TransactionInputCommitment;
pub use signature_verification::SigHashFlag;
pub use signature_verification::OpCheckSigDigest;
pub use signature_verification::DefaultOpCheckSigDigest;
