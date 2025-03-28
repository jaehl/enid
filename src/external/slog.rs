#![cfg(feature = "slog")]

use crate::enid::{Enid, Enid40, Enid80};
use slog::{Key, Record, Result, Serializer, Value};

#[cfg_attr(docsrs, doc(cfg(feature = "slog")))]
impl Value for Enid40 {
    fn serialize(&self, _record: &Record, key: Key, serializer: &mut dyn Serializer) -> Result {
        serializer.emit_str(key, self.write_to_buffer(&mut [0; 8]))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "slog")))]
impl Value for Enid80 {
    fn serialize(&self, _record: &Record, key: Key, serializer: &mut dyn Serializer) -> Result {
        serializer.emit_str(key, self.write_to_buffer(&mut [0; 17]))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "slog")))]
impl Value for Enid {
    fn serialize(&self, record: &Record, key: Key, serializer: &mut dyn Serializer) -> Result {
        match self {
            Self::Enid40(enid) => enid.serialize(record, key, serializer),
            Self::Enid80(enid) => enid.serialize(record, key, serializer),
        }
    }
}
