//! Opcodes that work on data strings.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::opcode::SpliceOpcode;

pub fn opcode_splice(
    _stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    _opcode: SpliceOpcode
) -> Result<(), ScriptError> {
    todo!()
}
