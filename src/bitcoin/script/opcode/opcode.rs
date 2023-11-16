//! Bitcoin script opcode element.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::opcode::opcode_constant;
use crate::bitcoin::script::opcode::opcode_data;
use crate::bitcoin::script::opcode::opcode_flowcontrol;
use crate::bitcoin::script::opcode::opcode_stack;
use crate::bitcoin::script::opcode::opcode_splice;
use crate::bitcoin::script::opcode::opcode_bitwise;
use crate::bitcoin::script::opcode::opcode_arithmetic;
use crate::bitcoin::script::opcode::opcode_cryptographic;
use crate::bitcoin::script::opcode::opcode_locktime;

/// Bitcoin script operations (opcodes).
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Opcode {
    Constant(ConstantOpcode),
    Data(DataOpcode),
    FlowControl(FlowControlOpcode),
    Stack(StackOpcode),
    Splice(SpliceOpcode),
    Bitwise(BitwiseOpcode),
    Arithmetic(ArithmeticOpcode),
    Cryptographic(CryptographicOpcode),
    Locktime(LocktimeOpcode),
}

pub fn call_opcode(
    stack: &mut Vec<DataElement>,
    script: &Script,
    instruction_pointer: usize,
    context: &ScriptExecutionContext,
    opcode: Opcode
) -> Result<(), ScriptError> {
    match opcode {
        Opcode::Constant(opcode) => opcode_constant(stack, script, instruction_pointer, context, opcode),
        Opcode::Data(opcode) => opcode_data(stack, script, instruction_pointer, context, opcode),
        Opcode::FlowControl(opcode) => opcode_flowcontrol(stack, script, instruction_pointer, context, opcode),
        Opcode::Stack(opcode) => opcode_stack(stack, script, instruction_pointer, context, opcode),
        Opcode::Splice(opcode) => opcode_splice(stack, script, instruction_pointer, context, opcode),
        Opcode::Bitwise(opcode) => opcode_bitwise(stack, script, instruction_pointer, context, opcode),
        Opcode::Arithmetic(opcode) => opcode_arithmetic(stack, script, instruction_pointer, context, opcode),
        Opcode::Cryptographic(opcode) => opcode_cryptographic(stack, script, instruction_pointer, context, opcode),
        Opcode::Locktime(opcode) => opcode_locktime(stack, script, instruction_pointer, context, opcode),
    }
}

/// Opcodes that represent constant values to be placed on the stack.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum ConstantOpcode {
    /// 0: `OP_0`/`OP_FALSE`
    OpFalse,

    /// 79: `OP_1NEGATE`
    Op1Negate,

    /// 81: `OP_1`/`OP_TRUE`
    OpTrue,

    /// 82: `OP_2`
    Op2,

    /// 83: `OP_3`
    Op3,

    /// 84: `OP_4`
    Op4,

    /// 85: `OP_5`
    Op5,

    /// 86: `OP_6`
    Op6,

    /// 87: `OP_7`
    Op7,

    /// 88: `OP_8`
    Op8,

    /// 89: `OP_9`
    Op9,

    /// 90: `OP_10`
    Op10,

    /// 91: `OP_11`
    Op11,

    /// 92: `OP_12`
    Op12,

    /// 93: `OP_13`
    Op13,

    /// 94: `OP_14`
    Op14,

    /// 95: `OP_15`
    Op15,

    /// 96: `OP_16`
    Op16,
}

/// Opcodes that place data onto the stack.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum DataOpcode {
    /// 1-75: Opcodes 1-75 are used as literal values to denote a following number of bytes as data.
    Literal(u8),

    /// 76: `OP_PUSHDATA1`
    OpPushData1,

    /// 77: `OP_PUSHDATA2`
    OpPushData2,

    /// 78: `OP_PUSHDATA4`
    OpPushData4,
}

/// Opcodes that dictate flow control.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum FlowControlOpcode {
    /// 97: `OP_NOP`
    OpNop,

    /// 99: `OP_IF`
    OpIf,

    /// 100: `OP_NOTIF`
    OpNotIf,

    /// 103: `OP_ELSE`
    OpElse,

    /// 104: `OP_ENDIF`
    OpEndIf,

    /// 105: `OP_VERIFY`
    OpVerify,

    /// 106: `OP_RETURN`
    OpReturn,
}

/// Opcodes that manipulate the stack.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum StackOpcode {
    /// 107: `OP_TOALTSTACK`
    OpToAltStack,

    /// 108: `OP_FROMALTSTACK`
    OpFromAltStack,

    /// 115: `OP_IFDUP`
    OpIfDup,

    /// 116: `OP_DEPTH`
    OpDepth,

    /// 117: `OP_DROP`
    OpDrop,

    /// 118: `OP_DUP`
    OpDup,

    /// 119: `OP_NIP`
    OpNip,

    /// 120: `OP_OVER`
    OpOver,

    /// 121: `OP_PICK`
    OpPick,

    /// 122: `OP_ROLL`
    OpRoll,

    /// 123: `OP_ROT`
    OpRot,

    /// 124: `OP_SWAP`
    OpSwap,

    /// 125: `OP_TUCK`
    OpTuck,

    /// 109: `OP_2DROP`
    Op2Drop,

    /// 110: `OP_2DUP`
    Op2Dup,

    /// 111: `OP_3DUP`
    Op3Dup,

    /// 112: `OP_2OVER`
    Op2Over,

    /// 113: `OP_2ROT`
    Op2Rot,

    /// 114: `OP_2SWAP`
    Op2Swap,
}

/// Opcodes that work on data strings.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum SpliceOpcode {
    /// 130: `OP_SIZE`
    OpSize,
}

/// Opcodes that perform bitwise operations.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum BitwiseOpcode {
    /// 135: `OP_EQUAL`
    OpEqual,

    /// 136: `OP_EQUALVERIFY`
    OpEqualVerify,
}

/// Opcodes that perform arithmetic operations.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum ArithmeticOpcode {
    /// 139: `OP_1ADD`
    Op1Add,

    /// 140: `OP_1SUB`
    Op1Sub,

    /// 143: `OP_NEGATE`
    OpNegate,

    /// 144: `OP_ABS`
    OpAbs,

    /// 145: `OP_NOT`
    OpNot,

    /// 146: `OP_0NOTEQUAL`
    Op0NotEqual,

    /// 147: `OP_ADD`
    OpAdd,

    /// 148: `OP_SUB`
    OpSub,

    /// 154: `OP_BOOLAND`
    OpBoolAnd,

    /// 155: `OP_BOOLOR`
    OpBoolOr,

    /// 156: `OP_NUMEQUAL`
    OpNumEqual,

    /// 157: `OP_NUMEQUALVERIFY`
    OpNumEqualVerify,

    /// 158: `OP_NUMNOTEQUAL`
    OpNumNotEqual,

    /// 159: `OP_LESSTHAN`
    OpLessThan,

    /// 160: `OP_GREATERTHAN`
    OpGreaterThan,

    /// 161: `OP_LESSTHANOREQUAL`
    OpLessThanOrEqual,

    /// 162: `OP_GREATERTHANOREQUAL`
    OpGreaterThanOrEqual,

    /// 163: `OP_MIN`
    OpMin,

    /// 164: `OP_MAX`
    OpMax,

    /// 165: `OP_WITHIN`
    OpWithin,
}

/// Opcodes that perform cryptographic operations.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum CryptographicOpcode {
    /// 166: `OP_RIPEMD160`
    OpRipemd160,

    /// 167: `OP_SHA1`
    OpSha1,

    /// 168: `OP_SHA256`
    OpSha256,

    /// 169: `OP_HASH160`
    OpHash160,

    /// 170: `OP_HASH256`
    OpHash256,

    /// 171: `OP_CODESEPARATOR`
    OpCodeSeparator,

    /// 172: `OP_CHECKSIG`
    OpCheckSig,

    /// 173: `OP_CHECKSIGVERIFY`
    OpCheckSigVerify,

    /// 174: `OP_CHECKMULTISIG`
    OpCheckMultisig,

    /// 175: `OP_CHECKMULTISIGVERIFY`
    OpCheckMultisigVerify,

    /// 186: `OP_CHECKSIGADD`
    OpCheckSigAdd,
}

/// Opcodes that enforce time locks.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LocktimeOpcode {
    /// 177: `OP_CHECKLOCKTIMEVERIFY`
    OpCheckLocktimeVerify,

    /// 178: `OP_CHECKSEQUENCEVERIFY`
    OpCheckSequenceVerify,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct OpcodeNotFound {
    byte: u8
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeNotFound;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(
            match byte {
                0x00_u8 => Opcode::Constant(ConstantOpcode::OpFalse),

                0x01_u8..=0x4b_u8 => Opcode::Data(DataOpcode::Literal(byte)),

                0x4c_u8 => Opcode::Data(DataOpcode::OpPushData1),
                0x4d_u8 => Opcode::Data(DataOpcode::OpPushData2),
                0x4e_u8 => Opcode::Data(DataOpcode::OpPushData4),

                0x4f_u8 => Opcode::Constant(ConstantOpcode::Op1Negate),

                0x51_u8 => Opcode::Constant(ConstantOpcode::OpTrue),

                0x52_u8 => Opcode::Constant(ConstantOpcode::Op2),
                0x53_u8 => Opcode::Constant(ConstantOpcode::Op3),
                0x54_u8 => Opcode::Constant(ConstantOpcode::Op4),
                0x55_u8 => Opcode::Constant(ConstantOpcode::Op5),
                0x56_u8 => Opcode::Constant(ConstantOpcode::Op6),
                0x57_u8 => Opcode::Constant(ConstantOpcode::Op7),
                0x58_u8 => Opcode::Constant(ConstantOpcode::Op8),
                0x59_u8 => Opcode::Constant(ConstantOpcode::Op9),
                0x5a_u8 => Opcode::Constant(ConstantOpcode::Op10),
                0x5b_u8 => Opcode::Constant(ConstantOpcode::Op11),
                0x5c_u8 => Opcode::Constant(ConstantOpcode::Op12),
                0x5d_u8 => Opcode::Constant(ConstantOpcode::Op13),
                0x5e_u8 => Opcode::Constant(ConstantOpcode::Op14),
                0x5f_u8 => Opcode::Constant(ConstantOpcode::Op15),
                0x60_u8 => Opcode::Constant(ConstantOpcode::Op16),

                0x61_u8 => Opcode::FlowControl(FlowControlOpcode::OpNop),

                0x63_u8 => Opcode::FlowControl(FlowControlOpcode::OpIf),
                0x64_u8 => Opcode::FlowControl(FlowControlOpcode::OpNotIf),
                0x67_u8 => Opcode::FlowControl(FlowControlOpcode::OpElse),
                0x68_u8 => Opcode::FlowControl(FlowControlOpcode::OpEndIf),

                0x69_u8 => Opcode::FlowControl(FlowControlOpcode::OpVerify),

                0x6a_u8 => Opcode::FlowControl(FlowControlOpcode::OpReturn),

                0x6b_u8 => Opcode::Stack(StackOpcode::OpToAltStack),
                0x6c_u8 => Opcode::Stack(StackOpcode::OpFromAltStack),

                0x73_u8 => Opcode::Stack(StackOpcode::OpIfDup),
                0x74_u8 => Opcode::Stack(StackOpcode::OpDepth),
                0x75_u8 => Opcode::Stack(StackOpcode::OpDrop),
                0x76_u8 => Opcode::Stack(StackOpcode::OpDup),
                0x77_u8 => Opcode::Stack(StackOpcode::OpNip),
                0x78_u8 => Opcode::Stack(StackOpcode::OpOver),
                0x79_u8 => Opcode::Stack(StackOpcode::OpPick),
                0x7a_u8 => Opcode::Stack(StackOpcode::OpRoll),
                0x7b_u8 => Opcode::Stack(StackOpcode::OpRot),
                0x7c_u8 => Opcode::Stack(StackOpcode::OpSwap),
                0x7d_u8 => Opcode::Stack(StackOpcode::OpTuck),

                0x6d_u8 => Opcode::Stack(StackOpcode::Op2Drop),
                0x6e_u8 => Opcode::Stack(StackOpcode::Op2Dup),
                0x6f_u8 => Opcode::Stack(StackOpcode::Op3Dup),
                0x70_u8 => Opcode::Stack(StackOpcode::Op2Over),
                0x71_u8 => Opcode::Stack(StackOpcode::Op2Rot),
                0x72_u8 => Opcode::Stack(StackOpcode::Op2Swap),

                0x82_u8 => Opcode::Splice(SpliceOpcode::OpSize),

                0x87_u8 => Opcode::Bitwise(BitwiseOpcode::OpEqual),
                0x88_u8 => Opcode::Bitwise(BitwiseOpcode::OpEqualVerify),

                0x8b_u8 => Opcode::Arithmetic(ArithmeticOpcode::Op1Add),
                0x8c_u8 => Opcode::Arithmetic(ArithmeticOpcode::Op1Sub),

                0x8f_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpNegate),
                0x90_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpAbs),
                0x91_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpNot),
                0x92_u8 => Opcode::Arithmetic(ArithmeticOpcode::Op0NotEqual),
                0x93_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpAdd),
                0x94_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpSub),

                0x9a_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpBoolAnd),
                0x9b_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpBoolOr),
                0x9c_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpNumEqual),
                0x9d_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpNumEqualVerify),
                0x9e_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpNumNotEqual),
                0x9f_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpLessThan),
                0xa0_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpGreaterThan),
                0xa1_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpLessThanOrEqual),
                0xa2_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpGreaterThanOrEqual),
                0xa3_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpMin),
                0xa4_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpMax),
                0xa5_u8 => Opcode::Arithmetic(ArithmeticOpcode::OpWithin),

                0xa6_u8 => Opcode::Cryptographic(CryptographicOpcode::OpRipemd160),
                0xa7_u8 => Opcode::Cryptographic(CryptographicOpcode::OpSha1),
                0xa8_u8 => Opcode::Cryptographic(CryptographicOpcode::OpSha256),
                0xa9_u8 => Opcode::Cryptographic(CryptographicOpcode::OpHash160),
                0xaa_u8 => Opcode::Cryptographic(CryptographicOpcode::OpHash256),

                0xab_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCodeSeparator),

                0xac_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCheckSig),
                0xad_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCheckSigVerify),

                0xae_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisig),
                0xaf_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisigVerify),
                0xba_u8 => Opcode::Cryptographic(CryptographicOpcode::OpCheckSigAdd),

                0xb1_u8 => Opcode::Locktime(LocktimeOpcode::OpCheckLocktimeVerify),
                0xb2_u8 => Opcode::Locktime(LocktimeOpcode::OpCheckSequenceVerify),

                _ => Err(OpcodeNotFound { byte: byte })?,
            }
        )
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Constant(ConstantOpcode::OpFalse) => 0x00_u8,

            Opcode::Data(DataOpcode::Literal(byte)) => byte,

            Opcode::Data(DataOpcode::OpPushData1) => 0x4c_u8,
            Opcode::Data(DataOpcode::OpPushData2) => 0x4d_u8,
            Opcode::Data(DataOpcode::OpPushData4) => 0x4e_u8,

            Opcode::Constant(ConstantOpcode::Op1Negate) => 0x4f_u8,

            Opcode::Constant(ConstantOpcode::OpTrue) => 0x51_u8,

            Opcode::Constant(ConstantOpcode::Op2) => 0x52_u8,
            Opcode::Constant(ConstantOpcode::Op3) => 0x53_u8,
            Opcode::Constant(ConstantOpcode::Op4) => 0x54_u8,
            Opcode::Constant(ConstantOpcode::Op5) => 0x55_u8,
            Opcode::Constant(ConstantOpcode::Op6) => 0x56_u8,
            Opcode::Constant(ConstantOpcode::Op7) => 0x57_u8,
            Opcode::Constant(ConstantOpcode::Op8) => 0x58_u8,
            Opcode::Constant(ConstantOpcode::Op9) => 0x59_u8,
            Opcode::Constant(ConstantOpcode::Op10) => 0x5a_u8,
            Opcode::Constant(ConstantOpcode::Op11) => 0x5b_u8,
            Opcode::Constant(ConstantOpcode::Op12) => 0x5c_u8,
            Opcode::Constant(ConstantOpcode::Op13) => 0x5d_u8,
            Opcode::Constant(ConstantOpcode::Op14) => 0x5e_u8,
            Opcode::Constant(ConstantOpcode::Op15) => 0x5f_u8,
            Opcode::Constant(ConstantOpcode::Op16) => 0x60_u8,

            Opcode::FlowControl(FlowControlOpcode::OpNop) => 0x61_u8,

            Opcode::FlowControl(FlowControlOpcode::OpIf) => 0x63_u8,
            Opcode::FlowControl(FlowControlOpcode::OpNotIf) => 0x64_u8,
            Opcode::FlowControl(FlowControlOpcode::OpElse) => 0x67_u8,
            Opcode::FlowControl(FlowControlOpcode::OpEndIf) => 0x68_u8,

            Opcode::FlowControl(FlowControlOpcode::OpVerify) => 0x69_u8,

            Opcode::FlowControl(FlowControlOpcode::OpReturn) => 0x6a_u8,

            Opcode::Stack(StackOpcode::OpToAltStack) => 0x6b_u8,
            Opcode::Stack(StackOpcode::OpFromAltStack) => 0x6c_u8,

            Opcode::Stack(StackOpcode::OpIfDup) => 0x73_u8,
            Opcode::Stack(StackOpcode::OpDepth) => 0x74_u8,
            Opcode::Stack(StackOpcode::OpDrop) => 0x75_u8,
            Opcode::Stack(StackOpcode::OpDup) => 0x76_u8,
            Opcode::Stack(StackOpcode::OpNip) => 0x77_u8,
            Opcode::Stack(StackOpcode::OpOver) => 0x78_u8,
            Opcode::Stack(StackOpcode::OpPick) => 0x79_u8,
            Opcode::Stack(StackOpcode::OpRoll) => 0x7a_u8,
            Opcode::Stack(StackOpcode::OpRot) => 0x7b_u8,
            Opcode::Stack(StackOpcode::OpSwap) => 0x7c_u8,
            Opcode::Stack(StackOpcode::OpTuck) => 0x7d_u8,

            Opcode::Stack(StackOpcode::Op2Drop) => 0x6d_u8,
            Opcode::Stack(StackOpcode::Op2Dup) => 0x6e_u8,
            Opcode::Stack(StackOpcode::Op3Dup) => 0x6f_u8,
            Opcode::Stack(StackOpcode::Op2Over) => 0x70_u8,
            Opcode::Stack(StackOpcode::Op2Rot) => 0x71_u8,
            Opcode::Stack(StackOpcode::Op2Swap) => 0x72_u8,

            Opcode::Splice(SpliceOpcode::OpSize) => 0x82_u8,

            Opcode::Bitwise(BitwiseOpcode::OpEqual) => 0x87_u8,
            Opcode::Bitwise(BitwiseOpcode::OpEqualVerify) => 0x88_u8,

            Opcode::Arithmetic(ArithmeticOpcode::Op1Add) => 0x8b_u8,
            Opcode::Arithmetic(ArithmeticOpcode::Op1Sub) => 0x8c_u8,

            Opcode::Arithmetic(ArithmeticOpcode::OpNegate) => 0x8f_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpAbs) => 0x90_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpNot) => 0x91_u8,
            Opcode::Arithmetic(ArithmeticOpcode::Op0NotEqual) => 0x92_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpAdd) => 0x93_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpSub) => 0x94_u8,

            Opcode::Arithmetic(ArithmeticOpcode::OpBoolAnd) => 0x9a_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpBoolOr) => 0x9b_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpNumEqual) => 0x9c_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpNumEqualVerify) => 0x9d_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpNumNotEqual) => 0x9e_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpLessThan) => 0x9f_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpGreaterThan) => 0xa0_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpLessThanOrEqual) => 0xa1_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpGreaterThanOrEqual) => 0xa2_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpMin) => 0xa3_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpMax) => 0xa4_u8,
            Opcode::Arithmetic(ArithmeticOpcode::OpWithin) => 0xa5_u8,

            Opcode::Cryptographic(CryptographicOpcode::OpRipemd160) => 0xa6_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpSha1) => 0xa7_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpSha256) => 0xa8_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpHash160) => 0xa9_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpHash256) => 0xaa_u8,

            Opcode::Cryptographic(CryptographicOpcode::OpCodeSeparator) => 0xab_u8,

            Opcode::Cryptographic(CryptographicOpcode::OpCheckSig) => 0xac_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpCheckSigVerify) => 0xad_u8,

            Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisig) => 0xae_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpCheckMultisigVerify) => 0xaf_u8,
            Opcode::Cryptographic(CryptographicOpcode::OpCheckSigAdd) => 0xba_u8,

            Opcode::Locktime(LocktimeOpcode::OpCheckLocktimeVerify) => 0xb1_u8,
            Opcode::Locktime(LocktimeOpcode::OpCheckSequenceVerify) => 0xb2_u8,
        }
    }
}
