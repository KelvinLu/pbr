use programming_bitcoin_in_rust::*;

use bitcoin::script::DataElement;
use bitcoin::script::ScriptError;
use util::byte_string::ByteString;

pub fn run() {
    let data_element = DataElement::of(b"more than four bytes");

    assert_eq!(data_element.number().err().unwrap(), ScriptError::ArithmeticInputOverflow);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[]);

    assert_eq!(data_element.number().err().unwrap(), ScriptError::EmptyDataElement);
    assert_eq!(bool::from(&data_element), false);

    let data_element = DataElement::of(&[0x00_u8; 32]);

    assert_eq!(data_element.number().unwrap(), 0);
    assert_eq!(bool::from(&data_element), false);

    let data_element = DataElement::of(&[0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]);

    assert_eq!(data_element.number().err().unwrap(), ScriptError::ArithmeticInputOverflow);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[0x00_u8, 0x01_u8, 0x00_u8, 0x02_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8]);

    assert_eq!(data_element.number().unwrap(), 33554688);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8]);

    assert_eq!(data_element.number().unwrap(), 256);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8]);
    assert_eq!(data_element.number().err().unwrap(), ScriptError::ArithmeticInputOverflow);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x80_u8, 0x00_u8]);
    assert_eq!(data_element.number().err().unwrap(), ScriptError::ArithmeticInputOverflow);
    assert_eq!(bool::from(&data_element), false);

    let data_element = DataElement::of(&[0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x80_u8, 0x00_u8]);
    assert_eq!(data_element.number().err().unwrap(), ScriptError::ArithmeticInputOverflow);
    assert_eq!(bool::from(&data_element), true);

    let data_element = DataElement::of(&[0x00_u8, 0x80_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8]);
    assert_eq!(data_element.number().unwrap(), 0);
    assert_eq!(bool::from(&data_element), false);

    let data_element = DataElement::of(&[0x00_u8, 0x81_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8]);
    assert_eq!(data_element.number().unwrap(), -256);
    assert_eq!(bool::from(&data_element), true);
}
