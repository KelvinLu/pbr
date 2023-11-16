//! Opcodes that manipulate the stack.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::stack::GetDataElement;
use crate::bitcoin::script::opcode::StackOpcode;

pub fn opcode_stack(
    stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    opcode: StackOpcode
) -> Result<(), ScriptError> {
    match opcode {
        StackOpcode::OpDup => {
            let data = stack.get_data_element()?;

            stack.push(data.clone());
            stack.push(data);
        },
        StackOpcode::Op2Dup => {
            let data_1 = stack.get_data_element()?;
            let data_2 = stack.get_data_element()?;

            stack.push(data_1.clone());
            stack.push(data_2.clone());
            stack.push(data_1);
            stack.push(data_2);
        },
        StackOpcode::OpSwap => {
            let data_1 = stack.get_data_element()?;
            let data_2 = stack.get_data_element()?;

            stack.push(data_1);
            stack.push(data_2);
        },
        _ => todo!(),
    }

    Ok(())
}
