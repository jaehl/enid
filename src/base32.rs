use crate::enid::EnidParseError;

const CHARS: [u8; 32] = *b"0123456789abcdefghjkmnpqrstvwxyz";

const VALUES: [u8; 256] = {
    let mut values = [0xff; 256];
    let mut i = 0;

    while i < CHARS.len() {
        let idx = CHARS[i] as usize;

        assert!(values[idx] == 0xff);
        values[idx] = i as u8;

        i += 1;
    }

    values
};

pub(crate) const fn encode(bytes: [u8; 5]) -> [u8; 8] {
    let bytes = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0];

    let mut bits = u64::from_be_bytes(bytes);
    let mut chars = [0; 8];

    let mut i = 0;
    while i < 8 {
        chars[i] = CHARS[(bits >> 59) as usize];
        bits <<= 5;

        i += 1;
    }

    chars
}

pub(crate) const fn decode(chars: [u8; 8]) -> Result<[u8; 5], EnidParseError> {
    let mut bits: u64 = 0;

    let mut i = 0;
    while i < 8 {
        let b = VALUES[chars[i] as usize];

        if b == 0xff {
            return Err(EnidParseError);
        }

        bits <<= 5;
        bits |= b as u64;
        i += 1;
    }

    let bytes = bits.to_be_bytes();

    Ok([bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}
