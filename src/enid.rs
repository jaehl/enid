use crate::base32;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::str::{self, FromStr};

/// An error returned when parsing an invalid ENID string.
///
/// This error is used as the error type for the [`FromStr`] implementation for
/// [`Enid40`], [`Enid80`], and [`Enid`].
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EnidParseError;

impl Display for EnidParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("invalid ENID syntax")
    }
}

impl Error for EnidParseError {}

/// A 40-bit ENID.
///
/// # Examples
///
/// ```
/// # use enid::Enid40;
/// # fn main() -> Result<(), enid::EnidParseError> {
/// let enid = Enid40::parse_str("m6sc7n75")?;
///
/// assert_eq!(enid.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Enid40([u8; 5]);

impl Enid40 {
    /// Creates an ENID from the given bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid40;
    /// let bytes = [0xa1, 0xb2, 0xc3, 0xd4, 0xe5];
    /// let enid = Enid40::from_bytes(bytes);
    ///
    /// assert_eq!(enid.to_string(), "m6sc7n75");
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_bytes(bytes: [u8; 5]) -> Self {
        Self(bytes)
    }

    /// Attempts to create an ENID from the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid40;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid = Enid40::parse_str("m6sc7n75")?;
    ///
    /// assert_eq!(enid.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    #[inline]
    pub const fn parse_str(s: &str) -> Result<Self, EnidParseError> {
        Self::parse_str_ascii(s.as_bytes())
    }

    /// Attempts to create an ENID from a string of ASCII characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid40;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid = Enid40::parse_str_ascii(b"m6sc7n75")?;
    ///
    /// assert_eq!(enid.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    pub const fn parse_str_ascii(s: &[u8]) -> Result<Self, EnidParseError> {
        if s.len() != 8 {
            return Err(EnidParseError);
        }

        match base32::decode(*s.first_chunk().unwrap()) {
            Ok(bytes) => Ok(Self(bytes)),
            Err(e) => Err(e),
        }
    }

    /// Returns a reference to the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid40;
    /// let enid = enid40!("m6sc7n75");
    ///
    /// assert_eq!(enid.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 5] {
        &self.0
    }

    /// Returns the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid40;
    /// let enid = enid40!("m6sc7n75");
    ///
    /// assert_eq!(enid.into_bytes(), [0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn into_bytes(self) -> [u8; 5] {
        self.0
    }

    // TODO: use std::ascii::Char - https://github.com/rust-lang/rust/issues/110998
    pub(crate) const fn write_to_buffer<'a>(&self, buf: &'a mut [u8; 8]) -> &'a str {
        *buf = base32::encode(self.0);

        unsafe { str::from_utf8_unchecked(buf) }
    }
}

impl Debug for Enid40 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Enid40 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.write_to_buffer(&mut [0; 8]))
    }
}

impl FromStr for Enid40 {
    type Err = EnidParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl From<[u8; 5]> for Enid40 {
    #[inline]
    fn from(bytes: [u8; 5]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<Enid40> for [u8; 5] {
    #[inline]
    fn from(enid: Enid40) -> Self {
        enid.into_bytes()
    }
}

/// An 80-bit ENID.
///
/// # Examples
///
/// ```
/// # use enid::Enid80;
/// # fn main() -> Result<(), enid::EnidParseError> {
/// let enid = Enid80::parse_str("y3gx5gxm-mpb8ey39")?;
///
/// assert_eq!(enid.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Enid80([u8; 10]);

impl Enid80 {
    /// Creates an ENID from the given bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid80;
    /// let bytes = [0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69];
    /// let enid = Enid80::from_bytes(bytes);
    ///
    /// assert_eq!(enid.to_string(), "y3gx5gxm-mpb8ey39");
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_bytes(bytes: [u8; 10]) -> Self {
        Self(bytes)
    }

    /// Attempts to create an ENID from the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid80;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid = Enid80::parse_str("y3gx5gxm-mpb8ey39")?;
    ///
    /// assert_eq!(enid.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    #[inline]
    pub const fn parse_str(s: &str) -> Result<Self, EnidParseError> {
        Self::parse_str_ascii(s.as_bytes())
    }

    /// Attempts to create an ENID from a string of ASCII characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid80;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid = Enid80::parse_str_ascii(b"y3gx5gxm-mpb8ey39")?;
    ///
    /// assert_eq!(enid.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    pub const fn parse_str_ascii(s: &[u8]) -> Result<Self, EnidParseError> {
        if s.len() != 17 {
            return Err(EnidParseError);
        }

        let mut bytes = [0; 10];

        match base32::decode(*s.first_chunk().unwrap()) {
            Ok(chunk) => *bytes.first_chunk_mut().unwrap() = chunk,
            Err(e) => return Err(e),
        };

        if s[8] != b'-' {
            return Err(EnidParseError);
        };

        match base32::decode(*s.last_chunk().unwrap()) {
            Ok(chunk) => *bytes.last_chunk_mut().unwrap() = chunk,
            Err(e) => return Err(e),
        };

        Ok(Self(bytes))
    }

    /// Returns a reference to the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid80;
    /// let enid = enid80!("y3gx5gxm-mpb8ey39");
    ///
    /// assert_eq!(enid.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 10] {
        &self.0
    }

    /// Returns the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid80;
    /// let enid = enid80!("y3gx5gxm-mpb8ey39");
    ///
    /// assert_eq!(enid.into_bytes(), [0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn into_bytes(self) -> [u8; 10] {
        self.0
    }

    // TODO: use std::ascii::Char - https://github.com/rust-lang/rust/issues/110998
    pub(crate) const fn write_to_buffer<'a>(&self, buf: &'a mut [u8; 17]) -> &'a str {
        *buf.first_chunk_mut().unwrap() = base32::encode(*self.0.first_chunk().unwrap());

        buf[8] = b'-';

        *buf.last_chunk_mut().unwrap() = base32::encode(*self.0.last_chunk().unwrap());

        unsafe { str::from_utf8_unchecked(buf) }
    }
}

impl Debug for Enid80 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Enid80 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.write_to_buffer(&mut [0; 17]))
    }
}

impl FromStr for Enid80 {
    type Err = EnidParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl From<[u8; 10]> for Enid80 {
    #[inline]
    fn from(bytes: [u8; 10]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<Enid80> for [u8; 10] {
    #[inline]
    fn from(enid: Enid80) -> Self {
        enid.into_bytes()
    }
}

/// An ENID, either 40 or 80 bits.
///
/// # Examples
///
/// ```
/// # use enid::{enid40, enid80, Enid};
/// let enid40 = Enid::Enid40(enid40!("m6sc7n75"));
/// let enid80 = Enid::Enid80(enid80!("y3gx5gxm-mpb8ey39"));
///
/// assert_eq!(enid40.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
/// assert_eq!(enid80.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Enid {
    /// A 40-bit ENID.
    Enid40(Enid40),

    /// An 80-bit ENID.
    Enid80(Enid80),
}

impl Enid {
    /// Attempts to create an ENID from the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid40 = Enid::parse_str("m6sc7n75")?;
    /// let enid80 = Enid::parse_str("y3gx5gxm-mpb8ey39")?;
    ///
    /// assert_eq!(enid40.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// assert_eq!(enid80.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    #[inline]
    pub const fn parse_str(s: &str) -> Result<Self, EnidParseError> {
        Self::parse_str_ascii(s.as_bytes())
    }

    /// Attempts to create an ENID from a string of ASCII characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::Enid;
    /// # fn main() -> Result<(), enid::EnidParseError> {
    /// let enid40 = Enid::parse_str_ascii(b"m6sc7n75")?;
    /// let enid80 = Enid::parse_str_ascii(b"y3gx5gxm-mpb8ey39")?;
    ///
    /// assert_eq!(enid40.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// assert_eq!(enid80.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`EnidParseError`] if the string is not a valid ENID.
    #[inline]
    pub const fn parse_str_ascii(s: &[u8]) -> Result<Self, EnidParseError> {
        if s.len() == 8 {
            match Enid40::parse_str_ascii(s) {
                Ok(enid) => Ok(Self::Enid40(enid)),
                Err(e) => Err(e),
            }
        } else {
            match Enid80::parse_str_ascii(s) {
                Ok(enid) => Ok(Self::Enid80(enid)),
                Err(e) => Err(e),
            }
        }
    }

    /// Returns a reference to the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid;
    /// let enid40 = enid!("m6sc7n75");
    /// let enid80 = enid!("y3gx5gxm-mpb8ey39");
    ///
    /// assert_eq!(enid40.as_bytes(), &[0xa1, 0xb2, 0xc3, 0xd4, 0xe5]);
    /// assert_eq!(enid80.as_bytes(), &[0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69]);
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        match self {
            Enid::Enid40(enid) => enid.as_bytes(),
            Enid::Enid80(enid) => enid.as_bytes(),
        }
    }

    /// Returns true if this is a 40-bit ENID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid;
    /// let enid40 = enid!("m6sc7n75");
    /// let enid80 = enid!("y3gx5gxm-mpb8ey39");
    ///
    /// assert_eq!(enid40.is_enid40(), true);
    /// assert_eq!(enid80.is_enid40(), false);
    ///
    #[must_use]
    #[inline]
    pub const fn is_enid40(&self) -> bool {
        matches!(self, Enid::Enid40(_))
    }

    /// Returns true if this is an 80-bit ENID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enid::enid;
    /// let enid40 = enid!("m6sc7n75");
    /// let enid80 = enid!("y3gx5gxm-mpb8ey39");
    ///
    /// assert_eq!(enid40.is_enid80(), false);
    /// assert_eq!(enid80.is_enid80(), true);
    ///
    #[must_use]
    #[inline]
    pub const fn is_enid80(&self) -> bool {
        matches!(self, Enid::Enid80(_))
    }
}

impl Debug for Enid {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Enid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Enid::Enid40(enid) => Display::fmt(enid, f),
            Enid::Enid80(enid) => Display::fmt(enid, f),
        }
    }
}

impl FromStr for Enid {
    type Err = EnidParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl From<Enid40> for Enid {
    #[inline]
    fn from(enid: Enid40) -> Self {
        Self::Enid40(enid)
    }
}

impl From<Enid80> for Enid {
    #[inline]
    fn from(enid: Enid80) -> Self {
        Self::Enid80(enid)
    }
}

impl From<[u8; 5]> for Enid {
    #[inline]
    fn from(bytes: [u8; 5]) -> Self {
        Self::Enid40(Enid40::from_bytes(bytes))
    }
}

impl From<[u8; 10]> for Enid {
    #[inline]
    fn from(bytes: [u8; 10]) -> Self {
        Self::Enid80(Enid80::from_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn enid40() {
        fn assert_valid(bytes: [u8; 5], string: &str) {
            let enid = Enid40(bytes);
            assert_eq!(enid.to_string(), string);
            assert_eq!(Enid40::from_str(string), Ok(enid));
        }

        assert_valid([0; 5], "00000000");
        assert_valid([0xff; 5], "zzzzzzzz");
        assert_valid([0, 0, 0, 0, 1], "00000001");
        assert_valid([0, 0, 0, 0, 31], "0000000z");
        assert_valid([0, 0, 0, 0, 32], "00000010");
        assert_valid([230, 41, 6, 32, 128], "wrmgc840");
        assert_valid([240, 225, 210, 195, 180], "y3gx5gxm");

        fn assert_invalid(string: &str) {
            assert_eq!(Enid40::from_str(string), Err(EnidParseError));
        }

        assert_invalid("");
        assert_invalid("0000000");
        assert_invalid("000000000");
        assert_invalid("0000-0000");
        assert_invalid("-00000000");
        assert_invalid("00000000-");
        assert_invalid("0000000i");
        assert_invalid("000000l0");
        assert_invalid("00000o00");
        assert_invalid("0000u000");
        assert_invalid("00000000-00000000");
    }

    #[test]
    fn enid80() {
        fn assert_valid(bytes: [u8; 10], string: &str) {
            let enid = Enid80(bytes);
            assert_eq!(enid.to_string(), string);
            assert_eq!(Enid80::from_str(string), Ok(enid));
        }

        assert_valid([0; 10], "00000000-00000000");
        assert_valid([0xff; 10], "zzzzzzzz-zzzzzzzz");
        assert_valid([0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "00000000-00000001");
        assert_valid([0, 0, 0, 0, 0, 0, 0, 0, 0, 31], "00000000-0000000z");
        assert_valid([0, 0, 0, 0, 64, 0, 0, 0, 0, 32], "00000020-00000010");
        assert_valid(
            [247, 53, 139, 82, 80, 115, 20, 131, 16, 64],
            "ywtrpmjg-eca86420",
        );
        assert_valid(
            [240, 225, 210, 195, 180, 165, 150, 135, 120, 105],
            "y3gx5gxm-mpb8ey39",
        );

        fn assert_invalid(string: &str) {
            assert_eq!(Enid80::from_str(string), Err(EnidParseError));
        }

        assert_invalid("");
        assert_invalid("0000000000000000");
        assert_invalid("0000000-00000000");
        assert_invalid("0000000-000000000");
        assert_invalid("000000000-0000000");
        assert_invalid("00000000-000000000");
        assert_invalid("0000-0000-00000000");
        assert_invalid("00000000-0000000i");
        assert_invalid("00000000-000000l0");
        assert_invalid("00000000-00000o00");
        assert_invalid("00000000-0000u000");
        assert_invalid("00000000");
    }

    #[test]
    fn enid_var() {
        fn assert_valid(bytes: &[u8], string: &str) {
            let enid = if bytes.len() == 5 {
                Enid::Enid40(Enid40(bytes.try_into().unwrap()))
            } else {
                Enid::Enid80(Enid80(bytes.try_into().unwrap()))
            };

            assert_eq!(enid.to_string(), string);
            assert_eq!(Enid::from_str(string), Ok(enid));
        }

        assert_valid(&[0; 5], "00000000");
        assert_valid(&[0xff; 5], "zzzzzzzz");
        assert_valid(&[0, 0, 0, 0, 1], "00000001");
        assert_valid(&[0, 0, 0, 0, 31], "0000000z");
        assert_valid(&[0, 0, 0, 0, 32], "00000010");
        assert_valid(&[230, 41, 6, 32, 128], "wrmgc840");
        assert_valid(&[240, 225, 210, 195, 180], "y3gx5gxm");

        assert_valid(&[0; 10], "00000000-00000000");
        assert_valid(&[0xff; 10], "zzzzzzzz-zzzzzzzz");
        assert_valid(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "00000000-00000001");
        assert_valid(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 31], "00000000-0000000z");
        assert_valid(&[0, 0, 0, 0, 64, 0, 0, 0, 0, 32], "00000020-00000010");
        assert_valid(
            &[247, 53, 139, 82, 80, 115, 20, 131, 16, 64],
            "ywtrpmjg-eca86420",
        );
        assert_valid(
            &[240, 225, 210, 195, 180, 165, 150, 135, 120, 105],
            "y3gx5gxm-mpb8ey39",
        );

        fn assert_invalid(string: &str) {
            assert_eq!(Enid40::from_str(string), Err(EnidParseError));
        }

        assert_invalid("");
        assert_invalid("0000000");
        assert_invalid("000000000");
        assert_invalid("0000-0000");
        assert_invalid("-00000000");
        assert_invalid("00000000-");
        assert_invalid("0000000i");
        assert_invalid("000000l0");
        assert_invalid("00000o00");
        assert_invalid("0000u000");
        assert_invalid("0000000000000000");
        assert_invalid("0000000-00000000");
        assert_invalid("0000000-000000000");
        assert_invalid("000000000-0000000");
        assert_invalid("00000000-000000000");
        assert_invalid("0000-0000-00000000");
        assert_invalid("00000000-0000000i");
        assert_invalid("00000000-000000l0");
        assert_invalid("00000000-00000o00");
        assert_invalid("00000000-0000u000");
    }
}
