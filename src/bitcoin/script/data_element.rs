//! Bitcoin script data element.

use crate::bitcoin::script::Opcode;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::opcode::DataOpcode;
use crate::bitcoin::script::opcode::ConstantOpcode;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::hexadecimal::hexadecimal_encode;

/// Bitcoin script data element (bytes).
#[derive(Clone)]
pub struct DataElement {
    /// Byte vector.
    bytes: Vec<u8>,

    /// Optional data opcode associated with the data element.
    opcode: Option<DataOpcode>,
}

impl ByteString for DataElement {
    /// Create a new data element of some bytes.
    fn of(bytes: &[u8]) -> Self {
        Self { bytes: Vec::from(bytes), opcode: None }
    }
}

impl ByteSlice for DataElement {
    /// Return the bytes of the data element.
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl DataElement {
    /// Create a new data element of some bytes, also noting the data opcode associated with the
    /// data element.
    pub fn from(opcode: DataOpcode, bytes: &[u8]) -> Self {
        Self { bytes: Vec::from(bytes), opcode: Some(opcode) }
    }

    /// Return the bytes of the data element, alongside a data opcode (including byte length).
    ///
    /// If the representation was not initialized with such an opcode, one will be inferred from
    /// the data length.
    pub fn bytes_with_opcode(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let length = self.bytes.len();

        let opcode = match self.opcode {
            Some(opcode) => Opcode::Data(opcode),
            None => {
                match length {
                    0 => Opcode::Constant(ConstantOpcode::OpFalse),
                    length if (1..=75).contains(&length) => {
                        Opcode::Data(DataOpcode::Literal(u8::try_from(length).unwrap()))
                    }
                    76..=0xff => {
                        Opcode::Data(DataOpcode::OpPushData1)
                    },
                    0x100..=0xffff => {
                        Opcode::Data(DataOpcode::OpPushData2)
                    },
                    0x10000..=0xffffffff => {
                        Opcode::Data(DataOpcode::OpPushData4)
                    },
                    _ => panic!("data length above limit"),
                }
            }
        };

        bytes.push(u8::from(opcode));

        match opcode {
            Opcode::Constant(ConstantOpcode::OpFalse) => {
                assert_eq!(length, 0);
            },
            Opcode::Data(opcode) => {
                assert!(self.compatible_opcode(opcode));

                match opcode {
                    DataOpcode::OpPushData1 => bytes.push(u8::try_from(length).unwrap()),
                    DataOpcode::OpPushData2 => bytes.extend_from_slice(&u16::try_from(length).unwrap().to_le_bytes()),
                    DataOpcode::OpPushData4 => bytes.extend_from_slice(&u32::try_from(length).unwrap().to_le_bytes()),
                    DataOpcode::Literal(_) => (),
                }
            },
            _ => panic!("unexpected opcode"),
        }

        bytes.extend_from_slice(&self.bytes);
        bytes
    }

    /// Determines if a given data opcode can properly represent the data element length.
    pub fn compatible_opcode(&self, opcode: DataOpcode) -> bool {
        let length = self.bytes.len();

        match opcode {
            DataOpcode::Literal(n) => (1..=75).contains(&n) && length == n.into(),
            DataOpcode::OpPushData1 => (0..=0xff).contains(&length),
            DataOpcode::OpPushData2 => (0..=0xffff).contains(&length),
            DataOpcode::OpPushData4 => (0..=0xffffffff).contains(&length),
        }
    }
}

impl DataElement {
    /// Returns a data element interpreted as a number.
    ///
    /// Bytes are interpreted as variable length little-endian signed integers.
    ///
    /// The most significant bit signifies a negative number.
    ///
    /// Operations are limited to processing four byte (32 bit) numbers as inputs, although
    /// arithmetic overflow is allowed in outputs and may result with a five byte data element
    /// placed on the stack.
    pub fn from_i64(number: i64) -> Result<Self, ScriptError> {
        if number == 0 { return Ok(Self::of(&[0_u8])) }

        let negative = number.is_negative();
        let number: i64 = if negative { -number } else { number };
        let mut bytes = number.to_le_bytes();

        let mut zeroes: usize = 0;

        for (i, byte) in bytes.iter().rev().enumerate() {
            zeroes = i;

            if *byte != 0_u8 { break }
        }

        if (bytes[7 - zeroes] & 0x80_u8) == 0x80_u8 { return Err(ScriptError::ArithmeticInputOverflow) };
        if negative { bytes[7 - zeroes] = bytes[7 - zeroes] | 0x80_u8 }

        Ok(Self::of(&bytes[0..(8 - zeroes)]))
    }

    /// Returns the data element interpreted as a number.
    ///
    /// Bytes are interpreted as variable length little-endian signed integers.
    ///
    /// The most significant bit signifies a negative number.
    ///
    /// Operations are limited to processing four byte (32 bit) numbers as inputs, although
    /// arithmetic overflow is allowed in outputs and may result with a five byte data element
    /// placed on the stack.
    pub fn number(&self) -> Result<i32, ScriptError> {
        if self.bytes.len() == 0 { return Err(ScriptError::EmptyDataElement) }

        let mut bytes = self.bytes.clone();

        bytes.reverse();

        let mut bytes = bytes.iter().skip_while(|byte| **byte == 0_u8);

        let most_significant_byte = match bytes.next() {
            Some(byte) => *byte,
            None => return Ok(0),
        };

        let negative = most_significant_byte & 0x80_u8 > 0x00_u8;
        let mut accumulator: i32 = (most_significant_byte & 0x7f_u8).into();
        let mut n = 3;

        for byte in bytes {
            if n == 0 { return Err(ScriptError::ArithmeticInputOverflow) }
            n -= 1;

            accumulator <<= 8;
            accumulator += <u8 as Into<i32>>::into(*byte);
        }

        if negative { accumulator = -accumulator }

        Ok(accumulator)
    }

    /// Returns whether the data element, if interpreted as the top data element in an exeuction
    /// stack, connotes script success or failure.
    ///
    /// A data element is considered "non-zero" if it is not "zero", which occurs when ...
    ///
    /// - ... the data element is interpreted as a number equal to zero or negative zero.
    /// - ... the data element consists of empty bytes.
    ///
    /// This function will attempt to find a little-endian signed integer whose value is zero,
    /// regardless of byte overflow limits.
    fn nonzero(&self) -> bool {
        if self.bytes.len() == 0 { return false }

        let mut bytes = self.bytes.clone();

        bytes.reverse();

        let mut bytes = bytes.iter().skip_while(|byte| **byte == 0_u8);

        let most_significant_byte = match bytes.next() {
            Some(byte) => *byte,
            None => return false,
        };

        if most_significant_byte == 0x80_u8 {
            for byte in bytes { if *byte != 0_u8 { return true } }
            return false;
        }

        return true;
    }
}

impl From<&DataElement> for bool {
    fn from(data_element: &DataElement) -> Self {
        data_element.nonzero()
    }
}

impl std::fmt::Debug for DataElement {
    /// Displays the data bytes (in hexadecimal).
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = vec![0_u8; self.bytes.len() * 2];

        hexadecimal_encode(&self.bytes, &mut buffer).unwrap();

        match self.opcode {
            Some(opcode) => write!(f, "<data [{:?}] {}>", opcode, std::str::from_utf8(&buffer).unwrap()),
            None => write!(f, "<data {}>", std::str::from_utf8(&buffer).unwrap()),
        }
    }
}
