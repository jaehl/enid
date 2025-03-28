#![cfg(feature = "serde")]

use crate::enid::{Enid, Enid40, Enid80};
use core::fmt::{self, Formatter};
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Enid40 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.write_to_buffer(&mut [0; 8]))
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Enid80 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.write_to_buffer(&mut [0; 17]))
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Enid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Enid::Enid40(enid) => enid.serialize(serializer),
            Enid::Enid80(enid) => enid.serialize(serializer),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Enid40 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid40;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a 40-bit ENID")
                }

                fn visit_str<E: de::Error>(self, s: &str) -> Result<Enid40, E> {
                    s.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Str(s), &self))
                }
            }

            deserializer.deserialize_str(EnidVisitor)
        } else {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid40;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a byte array of length 5")
                }

                fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Enid40, E> {
                    v.try_into()
                        .map(Enid40::from_bytes)
                        .map_err(|_| E::invalid_length(v.len(), &self))
                }
            }

            deserializer.deserialize_bytes(EnidVisitor)
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Enid80 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid80;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("an 80-bit ENID")
                }

                fn visit_str<E: de::Error>(self, s: &str) -> Result<Enid80, E> {
                    s.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Str(s), &self))
                }
            }

            deserializer.deserialize_str(EnidVisitor)
        } else {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid80;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a byte array of length 10")
                }

                fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Enid80, E> {
                    v.try_into()
                        .map(Enid80::from_bytes)
                        .map_err(|_| E::invalid_length(v.len(), &self))
                }
            }

            deserializer.deserialize_bytes(EnidVisitor)
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Enid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a 40-bit or 80-bit ENID")
                }

                fn visit_str<E: de::Error>(self, s: &str) -> Result<Enid, E> {
                    s.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Str(s), &self))
                }
            }

            deserializer.deserialize_str(EnidVisitor)
        } else {
            struct EnidVisitor;

            impl Visitor<'_> for EnidVisitor {
                type Value = Enid;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a byte array of length 5 or 10")
                }

                fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Enid, E> {
                    match v.len() {
                        5 => {
                            let bytes = v.try_into().unwrap();
                            Ok(Enid::Enid40(Enid40::from_bytes(bytes)))
                        }
                        10 => {
                            let bytes = v.try_into().unwrap();
                            Ok(Enid::Enid80(Enid80::from_bytes(bytes)))
                        }
                        n => Err(E::invalid_length(n, &self)),
                    }
                }
            }

            deserializer.deserialize_bytes(EnidVisitor)
        }
    }
}
