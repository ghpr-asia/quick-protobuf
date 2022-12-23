// Automatically generated rust module for 'data_types_import.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq, Clone)]
pub struct ImportedMessage {
    pub i: Option<bool>,
}


impl Default for ImportedMessage {
    fn default() -> Self {
        Self {
            i: None,
        }
    }
}

impl<'a> MessageRead<'a> for ImportedMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.i = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ImportedMessage {
    fn get_size(&self) -> usize {
        0
        + if self.i.is_some() { 1 + sizeof_varint((*(self.i.as_ref().unwrap())) as u64) } else { 0 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.i.is_some() { w.write_with_tag(8, |w| w.write_bool(*(self.i.as_ref().unwrap())))?; }
        Ok(())
    }
}

