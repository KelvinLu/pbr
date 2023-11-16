//! Segregated witness fields and items.

use crate::bitcoin::segwit::item::SegWitItem;
use crate::bitcoin::transaction::TransactionParsingError;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteVector;
use crate::util::byte_string::ByteSlice;
use crate::util::varint::read_varint_u64;
use crate::util::varint::varint_u64;

#[derive(Debug)]
#[derive(Clone)]
pub struct SegWitField {
    items: Vec<SegWitItem>
}

impl std::ops::Deref for SegWitField {
    type Target = Vec<SegWitItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl SegWitField {
    /// Parse segregated witness fields and items from a sequence of bytes.
    pub fn parse_bytes(bytes: &[u8]) -> Result<(Self, usize), TransactionParsingError> {
        let mut field: Vec<SegWitItem> = vec![];
        let mut index: usize = 0;

        // Parse the variable integer, given the leading byte.
        let (item_count, skip_bytes) =
            read_varint_u64(bytes[index..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        let item_count = usize::try_from(item_count)?;

        index += skip_bytes;

        for _ in 0..item_count {
            // Parse the variable integer, given the leading byte.
            let (item_length, skip_bytes) =
                read_varint_u64(bytes[index..].iter())
                    .ok_or(TransactionParsingError::VariableIntegerError)?;

            let item_length = usize::try_from(item_length)?;

            index += skip_bytes;

            field.push(SegWitItem::of(&bytes[index..(index + item_length)]));

            index += item_length;
        }

        Ok((Self { items: field }, index))
    }
}

impl ByteString for SegWitField {
    /// Parse a segregated witness field from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self {
        Self::parse_bytes(bytes).unwrap().0
    }
}

impl ByteVector for SegWitField {
    /// Return the sequence of bytes representing this segregated witness field.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // Variable integer for item count.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from(self.items.len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        for item in &self.items {
            // Variable integer for item count.
            let (varint_bytes, varint_length) = varint_u64(u64::try_from(item.bytes().len()).unwrap());

            bytes.extend_from_slice(&varint_bytes[0..varint_length]);

            // Item bytes.
            bytes.extend_from_slice(&item.bytes());
        }

        bytes
    }
}
