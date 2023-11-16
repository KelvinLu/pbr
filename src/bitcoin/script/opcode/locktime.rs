//! Opcodes that enforce time locks.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::opcode::LocktimeOpcode;

pub fn opcode_locktime(
    _stack: &mut Vec<DataElement>,
    _script: &Script,
    _instruction_pointer: usize,
    _context: &ScriptExecutionContext,
    _opcode: LocktimeOpcode
) -> Result<(), ScriptError> {
    todo!()
}
