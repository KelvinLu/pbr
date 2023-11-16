use programming_bitcoin_in_rust::*;

use bitcoin::script::Script;
use bitcoin::script::ScriptCreationError;
use bitcoin::script::Element;
use bitcoin::script::DataElement;
use bitcoin::script::Opcode;
use bitcoin::script::opcode::DataOpcode;
use bitcoin::script::opcode::ConstantOpcode;
use util::byte_string::ByteString;

pub fn run() {
    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::Literal(1))),
        Element::Opcode(Opcode::Constant(ConstantOpcode::OpTrue)),
    ];

    assert_eq!(Script::new(&script).err().unwrap(), ScriptCreationError);

    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::Literal(11))),
        Element::Data(DataElement::of(b"hello world")),
    ];

    assert_eq!(Script::new(&script).err(), None);

    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::Literal(1))),
        Element::Data(DataElement::of(b"hello world")),
    ];

    assert_eq!(Script::new(&script).err().unwrap(), ScriptCreationError);

    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::OpPushData1)),
        Element::Data(DataElement::of(&[0x00_u8; 300])),
    ];

    assert_eq!(Script::new(&script).err().unwrap(), ScriptCreationError);

    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::OpPushData4)),
        Element::Data(DataElement::of(&[0x00_u8; 300])),
    ];

    assert_eq!(Script::new(&script).err(), None);

    let script = [
        Element::Opcode(Opcode::Data(DataOpcode::Literal(11))),
        Element::Data(DataElement::of(b"hello world")),
        Element::Data(DataElement::of(b"goodbye")),
        Element::Opcode(Opcode::Data(DataOpcode::OpPushData4)),
        Element::Data(DataElement::of(&[0x00_u8; 300])),
    ];
    let script = Script::new(&script).unwrap();

    assert_eq!(script.elements().len(), 3);

    let Element::Data(element) = &script.elements()[0] else { panic!("unexpected result") };

    assert_eq!(element.bytes_with_opcode(), [11_u8].iter().copied().chain(b"hello world".iter().copied()).collect::<Vec<u8>>());

    let Element::Data(element) = &script.elements()[1] else { panic!("unexpected result") };

    assert_eq!(element.bytes_with_opcode(), [7_u8].iter().copied().chain(b"goodbye".iter().copied()).collect::<Vec<u8>>());

    let Element::Data(element) = &script.elements()[2] else { panic!("unexpected result") };

    assert_eq!(element.bytes_with_opcode(), [0x4e_u8, 0x2c_u8, 0x01_u8, 0x00_u8, 0x00_u8].iter().copied().chain([0x00_u8; 300].iter().copied()).collect::<Vec<u8>>());
}
