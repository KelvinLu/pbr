//! Bitcoin script.

use crate::bitcoin::script::Opcode;
use crate::bitcoin::script::opcode::DataOpcode;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script::ScriptBytes;
use crate::bitcoin::script::signature_verification::OpCheckSigDigest;
use crate::bitcoin::script::signature_verification::DefaultOpCheckSigDigest;
use crate::bitcoin::transaction::Transaction;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;

/// A Bitcoin script, an ordered series of elements (operations and data).
#[derive(Debug)]
#[derive(Clone)]
pub struct Script {
    elements: Vec<Element>,
}

/// A Bitcoin script element.
#[derive(Debug)]
#[derive(Clone)]
pub enum Element {
    /// Operations.
    Opcode(Opcode),

    /// Data.
    Data(DataElement),
}

impl Script {
    /// Return an element by its index within the script, if it exists.
    pub fn get(&self, n: usize) -> Option<&Element> {
        self.elements.get(n)
    }

    /// Create a new script from a series of elements.
    ///
    /// Data elements specified for this function are implicitly assumed to include the opcode (and
    /// any length bytes) which places the data onto the stack; those opcodes may be omitted.
    ///
    /// If a `DataOpcode` is specified, the following element must be a `DataElement` whose byte
    /// length is compatible. The opcode is omitted afterwards.
    pub fn new(elements: &[Element]) -> Result<Self, ScriptCreationError> {
        let mut script_elements: Vec<Element> = vec![];
        let mut data_opcode: Option<DataOpcode> = None;

        for pair in elements.windows(2) {
            if let Element::Opcode(Opcode::Data(opcode)) = pair[0] {
                if let Element::Data(element) = &pair[1] {
                    if element.compatible_opcode(opcode) {
                        data_opcode = Some(opcode);
                    } else {
                        return Err(ScriptCreationError);
                    }
                } else {
                    return Err(ScriptCreationError);
                }
            } else {
                let element = pair[0].clone();

                Self::augment_data_element(&mut script_elements, &mut data_opcode, element);
            }
        }

        if elements.len() > 0 {
            Self::augment_data_element(&mut script_elements, &mut data_opcode, elements.last().unwrap().clone());
        }

        Ok(Self { elements: script_elements })
    }

    fn augment_data_element(script_elements: &mut Vec<Element>, data_opcode: &mut Option<DataOpcode>, element: Element) {
        if let Some(opcode) = data_opcode {
            if let Element::Data(element) = element {
                script_elements.push(Element::Data(DataElement::from(*opcode, element.bytes())));

                *data_opcode = None;
            } else {
                panic!("expected data element");
            }
        } else {
            script_elements.push(element);
        }
    }
}

impl std::fmt::Display for Script {
    /// Displays the script.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<script {:?}>", self.elements)
    }
}

impl TryFrom<&ScriptBytes> for Script {
    type Error = ScriptCreationError;

    fn try_from(bytes: &ScriptBytes) -> Result<Self, Self::Error> {
        let bytes = bytes.bytes();
        let mut elements: Vec<Element> = vec![];

        let bytes_length = bytes.len();
        let mut byte: Option<&u8> = bytes.first();
        let mut i: usize = 0;

        while byte.is_some() {
            i += 1;

            let opcode = Opcode::try_from(*byte.unwrap()).map_err(|_| ScriptCreationError)?;

            match opcode {
                Opcode::Data(opcode) => {
                    let length: usize = match opcode {
                        DataOpcode::Literal(n) => {
                            n.into()
                        },
                        DataOpcode::OpPushData1 => {
                            let Some(length_byte) = bytes.get(i) else { return Err(ScriptCreationError) };

                            i += 1;
                            (*length_byte).into()
                        },
                        DataOpcode::OpPushData2 => {
                            let mut length_bytes = [0_u8; 2];

                            if (i + 2) >= bytes_length { return Err(ScriptCreationError) }

                            length_bytes.clone_from_slice(&bytes[i..(i + 2)]);

                            i += 2;
                            u16::from_le_bytes(length_bytes).into()
                        },
                        DataOpcode::OpPushData4 => {
                            let mut length_bytes = [0_u8; 4];

                            if (i + 4) >= bytes_length { return Err(ScriptCreationError) }

                            length_bytes.clone_from_slice(&bytes[i..(i + 4)]);

                            i += 4;
                            usize::try_from(u32::from_le_bytes(length_bytes)).map_err(|_| ScriptCreationError)?
                        },
                    };

                    if (i + length) > bytes_length { return Err(ScriptCreationError) }

                    let data_bytes = bytes[i..(i + length)].to_vec();

                    i += length;
                    elements.push(Element::Data(DataElement::from(opcode, &data_bytes)));
                },
                _ => {
                    elements.push(Element::Opcode(opcode));
                },
            }

            byte = bytes.get(i);
        }

        Self::new(&elements)
    }
}

impl From<&Script> for ScriptBytes {
    fn from(script: &Script) -> Self {
        let mut bytes: Vec<u8> = vec![];

        for element in script.elements() {
            match element {
                Element::Opcode(opcode) => bytes.push(u8::from(*opcode)),
                Element::Data(data_element) => bytes.extend_from_slice(&data_element.bytes_with_opcode()),
            }
        }

        Self::of(&bytes)
    }
}

impl Script {
    /// Concatenate two scripts and reevaluate the byte representation.
    pub fn concatenate(&self, rhs: &Self) -> Result<Self, ScriptCreationError> {
        let bytes = ScriptBytes::from(self).concatenate(&ScriptBytes::from(rhs));

        Ok(Self::try_from(&bytes)?)
    }

    /// Return the elements that this script consists of.
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }
}

/// Contains values that may be used by various opcodes during execution.
pub struct ScriptExecutionContext<'a> {
    /// A transaction that signs previous transactions' UTXOs.
    ///
    /// See `OP_CHECKSIG`.
    pub transaction: &'a Transaction,

    /// The index of the transaction input which provides a signature to be verified.
    ///
    /// See `OP_CHECKSIG`.
    pub input_index: usize,

    /// 256 bit digest function used for the `OP_CHECKSIG` operation.
    ///
    /// See `OP_CHECKSIG`.
    pub checksig_digest: &'a dyn OpCheckSigDigest,

    /// Current UNIX timestamp.
    ///
    /// See `OP_CHECKLOCKTIMEVERIFY`.
    pub timestamp: u64,

    /// Block height.
    ///
    /// See `OP_CHECKLOCKTIMEVERIFY`.
    pub block_height: u64,
}

impl <'a> ScriptExecutionContext<'a> {
    pub fn new(
        transaction: &'a Transaction,
        input_index: usize,
        timestamp: u64,
        block_height: u64,
    ) -> Self {
        Self {
            transaction: transaction,
            input_index: input_index,
            checksig_digest: &DefaultOpCheckSigDigest {},
            timestamp: timestamp,
            block_height: block_height,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ScriptCreationError;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum ScriptError {
    /// An opcode has failed; transaction invalid.
    OpcodeFailed(Opcode),

    /// `OP_RETURN`; transaction invalid.
    OpReturn,

    /// An arithmetic input results in an overflow error.
    ArithmeticInputOverflow,

    /// The stack was empty after execution.
    EmptyStack,

    /// The data element consists of empty bytes.
    EmptyDataElement,

    /// An `OP_IF`, `OP_NOTIF`, `OP_ELSE`, or `OP_ENDIF` is mismatched; all conditional blocks must
    /// begin and end.
    ConditionalBlockMismatched,
}
