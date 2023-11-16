//! Opcodes that perform bitwise operations.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::stack::GetDataElement;
use crate::bitcoin::script::opcode::BitwiseOpcode;
use crate::bitcoin::script::opcode::FlowControlOpcode;
use crate::bitcoin::script::opcode::opcode_flowcontrol;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;

pub fn opcode_bitwise(
    stack: &mut Vec<DataElement>,
    script: &Script,
    instruction_pointer: usize,
    context: &ScriptExecutionContext,
    opcode: BitwiseOpcode
) -> Result<(), ScriptError> {
    match opcode {
        BitwiseOpcode::OpEqual => {
            let data_1 = stack.get_data_element()?;
            let data_2 = stack.get_data_element()?;

            match data_1.bytes() == data_2.bytes() {
                true => stack.push(DataElement::of(&[1_u8])),
                false => stack.push(DataElement::of(&[0_u8])),
            }
        },
        BitwiseOpcode::OpEqualVerify => {
            opcode_bitwise(stack, script, instruction_pointer, context, BitwiseOpcode::OpEqual)?;
            opcode_flowcontrol(stack, script, instruction_pointer, context, FlowControlOpcode::OpVerify)?;
        },
    }

    Ok(())
}
