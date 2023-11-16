use programming_bitcoin_in_rust::*;

use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::opcode::DataOpcode;
use util::byte_string::ByteString;
use util::byte_string::ByteSlice;

pub fn run() {
    let data_element = DataElement::of(b"hello world");
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 12);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::Literal(11)));
    assert_eq!(bytes[0], u8::try_from(data_element.bytes().len()).unwrap());
    assert_eq!(bytes[1..], *data_element.bytes());

    let data_element = DataElement::from(DataOpcode::OpPushData1, b"hello world");
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 13);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::OpPushData1));
    assert_eq!(bytes[1], u8::try_from(data_element.bytes().len()).unwrap());
    assert_eq!(bytes[2..], *data_element.bytes());

    let data_element = DataElement::from(DataOpcode::OpPushData2, b"hello world");
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 14);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::OpPushData2));
    assert_eq!(bytes[1..=2], u16::try_from(data_element.bytes().len()).unwrap().to_le_bytes());
    assert_eq!(bytes[3..], *data_element.bytes());

    let data_element = DataElement::from(DataOpcode::OpPushData4, b"hello world");
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 16);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::OpPushData4));
    assert_eq!(bytes[1..=4], u32::try_from(data_element.bytes().len()).unwrap().to_le_bytes());
    assert_eq!(bytes[5..], *data_element.bytes());

    let data_element = DataElement::of(&[0x00_u8; 100]);
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 102);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::OpPushData1));
    assert_eq!(bytes[1], u8::try_from(data_element.bytes().len()).unwrap());
    assert_eq!(bytes[2..], *data_element.bytes());

    assert!(!data_element.compatible_opcode(DataOpcode::Literal(10)));
    assert!(!data_element.compatible_opcode(DataOpcode::Literal(200)));
    assert!(data_element.compatible_opcode(DataOpcode::OpPushData1));
    assert!(data_element.compatible_opcode(DataOpcode::OpPushData2));
    assert!(data_element.compatible_opcode(DataOpcode::OpPushData4));

    let data_element = DataElement::of(&[0x00_u8; 300]);
    let bytes = data_element.bytes_with_opcode();

    assert_eq!(bytes.len(), 303);
    assert_eq!(Opcode::try_from(bytes[0]).unwrap(), Opcode::Data(DataOpcode::OpPushData2));
    assert_eq!(bytes[1..=2], u16::try_from(data_element.bytes().len()).unwrap().to_le_bytes());
    assert_eq!(bytes[3..], *data_element.bytes());

    assert!(!data_element.compatible_opcode(DataOpcode::Literal(10)));
    assert!(!data_element.compatible_opcode(DataOpcode::OpPushData1));
    assert!(data_element.compatible_opcode(DataOpcode::OpPushData2));
    assert!(data_element.compatible_opcode(DataOpcode::OpPushData4));
}
