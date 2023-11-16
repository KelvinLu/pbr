//! Opcodes that dictate flow control.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::Opcode;
use crate::bitcoin::script::stack::GetDataElement;
use crate::bitcoin::script::opcode::FlowControlOpcode;

pub fn opcode_flowcontrol(
    stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    opcode: FlowControlOpcode
) -> Result<(), ScriptError> {
    match opcode {
        FlowControlOpcode::OpVerify => {
            let data = stack.get_data_element()?;

            if !bool::from(&data) {
                return Err(ScriptError::OpcodeFailed(Opcode::FlowControl(FlowControlOpcode::OpVerify)));
            }
        },
        FlowControlOpcode::OpReturn=> {
            return Err(ScriptError::OpReturn);
        },
        _ => todo!(),
    }

    Ok(())
}
