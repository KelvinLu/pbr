//! Opcodes that work on data strings.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::opcode::DataOpcode;

pub fn opcode_data(
    _stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    _opcode: DataOpcode
) -> Result<(), ScriptError> {
    todo!()
}
