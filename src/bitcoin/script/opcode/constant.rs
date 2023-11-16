//! Opcodes that represent constant values to be placed on the stack.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::opcode::ConstantOpcode;
use crate::util::byte_string::ByteString;

pub fn opcode_constant(
    stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    opcode: ConstantOpcode
) -> Result<(), ScriptError> {
    match opcode {
        ConstantOpcode::OpFalse => stack.push(DataElement::of(&[])),
        ConstantOpcode::Op1Negate => stack.push(DataElement::from_i64(-1)?),
        ConstantOpcode::OpTrue => stack.push(DataElement::of(&[1_u8])),
        ConstantOpcode::Op2 => stack.push(DataElement::of(&[2_u8])),
        ConstantOpcode::Op3 => stack.push(DataElement::of(&[3_u8])),
        ConstantOpcode::Op4 => stack.push(DataElement::of(&[4_u8])),
        ConstantOpcode::Op5 => stack.push(DataElement::of(&[5_u8])),
        ConstantOpcode::Op6 => stack.push(DataElement::of(&[6_u8])),
        ConstantOpcode::Op7 => stack.push(DataElement::of(&[7_u8])),
        ConstantOpcode::Op8 => stack.push(DataElement::of(&[8_u8])),
        ConstantOpcode::Op9 => stack.push(DataElement::of(&[9_u8])),
        ConstantOpcode::Op10 => stack.push(DataElement::of(&[10_u8])),
        ConstantOpcode::Op11 => stack.push(DataElement::of(&[11_u8])),
        ConstantOpcode::Op12 => stack.push(DataElement::of(&[12_u8])),
        ConstantOpcode::Op13 => stack.push(DataElement::of(&[13_u8])),
        ConstantOpcode::Op14 => stack.push(DataElement::of(&[14_u8])),
        ConstantOpcode::Op15 => stack.push(DataElement::of(&[15_u8])),
        ConstantOpcode::Op16 => stack.push(DataElement::of(&[16_u8])),
    }

    Ok(())
}
