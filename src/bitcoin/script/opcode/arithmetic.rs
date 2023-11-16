//! Opcodes that perform arithmetic operations.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::stack::GetDataElement;
use crate::bitcoin::script::opcode::ArithmeticOpcode;
use crate::util::byte_string::ByteString;

pub fn opcode_arithmetic(
    stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    opcode: ArithmeticOpcode
) -> Result<(), ScriptError> {
    match opcode {
        ArithmeticOpcode::OpAdd => {
            let data_1 = stack.get_data_element()?;
            let data_2 = stack.get_data_element()?;

            stack.push(DataElement::from_i64(i64::from(data_1.number()?) + i64::from(data_2.number()?))?);
        },
        ArithmeticOpcode::OpNot => {
            let data = stack.get_data_element()?;

            if data.number()? == 0 {
                stack.push(DataElement::of(&[1_u8]));
            } else {
                stack.push(DataElement::of(&[0_u8]));
            }
        },
        _ => todo!(),
    }

    Ok(())
}
