//! Opcode implementations.

mod opcode;
mod constant;
mod data;
mod flow_control;
mod stack;
mod splice;
mod bitwise;
mod arithmetic;
mod cryptographic;
mod locktime;

pub use opcode::Opcode;
pub use opcode::ConstantOpcode;
pub use opcode::DataOpcode;
pub use opcode::FlowControlOpcode;
pub use opcode::StackOpcode;
pub use opcode::SpliceOpcode;
pub use opcode::BitwiseOpcode;
pub use opcode::ArithmeticOpcode;
pub use opcode::CryptographicOpcode;
pub use opcode::LocktimeOpcode;

pub use opcode::call_opcode;
pub use constant::opcode_constant;
pub use data::opcode_data;
pub use flow_control::opcode_flowcontrol;
pub use stack::opcode_stack;
pub use splice::opcode_splice;
pub use bitwise::opcode_bitwise;
pub use arithmetic::opcode_arithmetic;
pub use cryptographic::opcode_cryptographic;
pub use locktime::opcode_locktime;
