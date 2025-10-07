#![cfg(feature = "borsh")]

extern crate alloc;

use crate::enid::{Enid, Enid40, Enid80};
use borsh::io::{Error, ErrorKind, Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for Enid40 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl BorshSerialize for Enid80 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl BorshSerialize for Enid {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Self::Enid40(enid) => {
                writer.write_all(&[0])?;
                enid.serialize(writer)
            }
            Self::Enid80(enid) => {
                writer.write_all(&[1])?;
                enid.serialize(writer)
            }
        }
    }
}

impl BorshDeserialize for Enid40 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        <[u8; 5]>::deserialize_reader(reader).map(Self::from_bytes)
    }
}

impl BorshDeserialize for Enid80 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        <[u8; 10]>::deserialize_reader(reader).map(Self::from_bytes)
    }
}

impl BorshDeserialize for Enid {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        match u8::deserialize_reader(reader)? {
            0 => Enid40::deserialize_reader(reader).map(Self::Enid40),
            1 => Enid80::deserialize_reader(reader).map(Self::Enid80),
            value => Err(Error::new(
                ErrorKind::InvalidData,
                alloc::format!("Invalid VarEnid80 variant: {value}"),
            )),
        }
    }
}
