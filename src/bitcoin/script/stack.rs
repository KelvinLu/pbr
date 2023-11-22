//! Bitcoin script stack.

use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptExecutionContext;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::Element;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::Opcode;
use crate::bitcoin::script::call_opcode;

/// Bitcoin script stack.
///
/// Used to evaluate scripts.
pub struct Stack<'a> {
    /// The contents of a stack, containing data elements processed by a script.
    stack: Vec<DataElement>,

    /// Alternative stack (`OP_TOALTSTACK`, `OP_FROMALTSTACK`).
    #[allow(dead_code)]
    altstack: Vec<DataElement>,

    /// A reference to the script being processed.
    script: &'a Script,

    /// Instruction pointer.
    n: usize,

    /// Denotes whether the script has finished execution.
    end: bool,

    /// An error the script has encountered.
    error: Option<ScriptError>,

    /// Provides information for various operations.
    context: &'a ScriptExecutionContext<'a>,
}

impl <'a> Stack<'a> {
    /// Create a new stack for a given script.
    pub fn new(script: &'a Script, context: &'a ScriptExecutionContext) -> Self {
        Self {
            stack: vec![],
            altstack: vec![],
            script: script,
            n: 0,
            end: false,
            error: None,
            context: context,
        }
    }

    /// Create a clone of the stack, after fully executing the current script, and adjoin another
    /// script to subsequently execute in the same context.
    pub fn adjoin(&mut self, script: &'a Script) -> Result<Self, ScriptError> {
        self.evaluate()?;

        Ok(
            Self {
                stack: self.stack.clone(),
                altstack: self.altstack.clone(),
                script: script,
                n: 0,
                end: false,
                error: None,
                context: self.context,
            }
        )
    }

    /// Evaluate the script.
    ///
    /// Returns true when the top data element of the stack is non-zero.
    ///
    /// Returns false when the top data element of the stack is zero, negative zero, or empty.
    ///
    /// Returns any errors encountered while evaluating the script.
    pub fn evaluate(&mut self) -> Result<bool, ScriptError> {
        loop {
            match self.evaluate_element() {
                Some(result) => {
                    match result {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                },
                None => {
                    if let Some(error) = self.error { return Err(error) }

                    return Ok(bool::from(self.peek().ok_or(ScriptError::EmptyStack)?));
                },
            }
        }
    }

    /// End execution of the script.
    pub fn end(&mut self) {
        self.end = true;
    }

    /// End execution of the script (via error).
    pub fn end_error(&mut self, error: ScriptError) {
        self.error = Some(error);
        self.end();
    }

    /// Process script elements and call a single opcode from the script sequence.
    pub fn evaluate_element(&mut self) -> Option<Result<Opcode, ScriptError>> {
        if self.end { return None }

        loop {
            let element = self.script.get(self.n);

            match element {
                Some(Element::Opcode(opcode)) => {
                    let result = match call_opcode(
                        &mut self.stack,
                        self.script,
                        self.n,
                        &self.context,
                        *opcode
                    ) {
                        Ok(_) => Some(Ok(*opcode)),
                        Err(error) => {
                            self.end_error(error);

                            return Some(Err(error));
                        },
                    };

                    self.n += 1;

                    return result;
                },
                Some(Element::Data(data_element)) => {
                    self.stack.push(data_element.clone());
                },
                None => {
                    self.end();

                    return None;
                },
            }

            self.n += 1;
        }
    }

    /// Return a reference the data element at the top of the stack, if any.
    pub fn peek(&self) -> Option<&DataElement> {
        self.stack.last()
    }

    /// Return a reference to the stack.
    pub fn stack(&self) -> &Vec<DataElement> {
        &self.stack
    }

    /// Drop the data element at the top of the stack, if any.
    pub fn drop(&mut self) -> Option<DataElement> {
        self.stack.pop()
    }
}

impl Iterator for Stack<'_> {
    type Item = Result<Opcode, ScriptError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.evaluate_element()
    }
}

pub trait GetDataElement {
    fn get_data_element(&mut self) -> Result<DataElement, ScriptError>;
}

impl GetDataElement for Vec<DataElement> {
    fn get_data_element(&mut self) -> Result<DataElement, ScriptError> {
        self.pop().ok_or(ScriptError::EmptyStack)
    }
}
