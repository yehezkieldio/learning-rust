//! UUIDv47 - UUIDv7-in / UUIDv4-out (SipHash-masked timestamp)
//!
//! This library lets you store sortable UUIDv7 in your database while emitting
//! a UUIDv4-looking façade at your API boundary. It does this by XOR-masking
//! only the UUIDv7 timestamp field with a keyed SipHash-2-4 stream tied to
//! the UUID's own random bits.

use std::fmt;
use std::str::FromStr;

/// 128-bit UUID representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid128 {
    pub bytes: [u8; 16],
}

/// SipHash 128-bit key
#[derive(Debug, Clone, Copy)]
pub struct Uuidv47Key {
    pub k0: u64,
    pub k1: u64,
}

impl Uuid128 {
    /// Create a new UUID from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid128 { bytes }
    }

    /// Get the version field (bits 48-51)
    pub fn version(&self) -> u8 {
        (self.bytes[6] >> 4) & 0x0F
    }

    /// Set the version field
    pub fn set_version(&mut self, ver: u8) {
        self.bytes[6] = (self.bytes[6] & 0x0F) | ((ver & 0x0F) << 4);
    }

    /// Set the RFC4122 variant bits (10xxxxxx in byte 8)
    pub fn set_variant_rfc4122(&mut self) {
        self.bytes[8] = (self.bytes[8] & 0x3F) | 0x80;
    }

    /// Parse from canonical string format (8-4-4-4-12)
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        if s.len() != 36 {
            return Err(ParseError::InvalidLength);
        }

        let mut bytes = [0u8; 16];
        let mut byte_idx = 0;
        let mut char_idx = 0;

        for expected_dash in [8, 13, 18, 23] {
            // Parse hex digits before dash
            while char_idx < expected_dash {
                if byte_idx >= 16 {
                    return Err(ParseError::InvalidFormat);
                }

                let high = hex_char_to_nibble(s.chars().nth(char_idx).ok_or(ParseError::InvalidFormat)?)?;
                let low = hex_char_to_nibble(s.chars().nth(char_idx + 1).ok_or(ParseError::InvalidFormat)?)?;
                bytes[byte_idx] = (high << 4) | low;

                byte_idx += 1;
                char_idx += 2;
            }

            // Check for dash (except at the end)
            if char_idx < 36 {
                if s.chars().nth(char_idx) != Some('-') {
                    return Err(ParseError::InvalidFormat);
                }
                char_idx += 1;
            }
        }

        // Parse remaining hex digits
        while char_idx < 36 {
            if byte_idx >= 16 {
                return Err(ParseError::InvalidFormat);
            }

            let high = hex_char_to_nibble(s.chars().nth(char_idx).ok_or(ParseError::InvalidFormat)?)?;
            let low = hex_char_to_nibble(s.chars().nth(char_idx + 1).ok_or(ParseError::InvalidFormat)?)?;
            bytes[byte_idx] = (high << 4) | low;

            byte_idx += 1;
            char_idx += 2;
        }

        Ok(Uuid128 { bytes })
    }

    /// Format as canonical string (8-4-4-4-12)
    pub fn format(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }

    /// Create a v7 UUID from components (for testing)
    pub fn craft_v7(ts_ms_48: u64, rand_a_12: u16, rand_b_62: u64) -> Self {
        let mut u = Uuid128 { bytes: [0; 16] };

        // Write 48-bit timestamp (big-endian)
        let ts = ts_ms_48 & 0x0000_FFFF_FFFF_FFFF;
        write_48be(&mut u.bytes[0..6], ts);

        // Set version to 7
        u.set_version(7);

        // Write rand_a (12 bits)
        u.bytes[6] = (u.bytes[6] & 0xF0) | ((rand_a_12 >> 8) as u8 & 0x0F);
        u.bytes[7] = (rand_a_12 & 0xFF) as u8;

        // Set RFC variant
        u.set_variant_rfc4122();

        // Write rand_b (62 bits)
        let rand_b = rand_b_62 & ((1u64 << 62) - 1);
        u.bytes[8] = (u.bytes[8] & 0xC0) | ((rand_b >> 56) as u8 & 0x3F);
        for i in 0..7 {
            u.bytes[9 + i] = ((rand_b >> (8 * (6 - i))) & 0xFF) as u8;
        }

        u
    }
}

impl fmt::Display for Uuid128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl FromStr for Uuid128 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid128::parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLength,
    InvalidFormat,
    InvalidHexChar,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLength => write!(f, "Invalid UUID string length"),
            ParseError::InvalidFormat => write!(f, "Invalid UUID string format"),
            ParseError::InvalidHexChar => write!(f, "Invalid hexadecimal character"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Convert hex character to nibble
fn hex_char_to_nibble(c: char) -> Result<u8, ParseError> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(ParseError::InvalidHexChar),
    }
}

/// Write 48-bit value as big-endian
fn write_48be(dst: &mut [u8], v48: u64) {
    dst[0] = (v48 >> 40) as u8;
    dst[1] = (v48 >> 32) as u8;
    dst[2] = (v48 >> 24) as u8;
    dst[3] = (v48 >> 16) as u8;
    dst[4] = (v48 >> 8) as u8;
    dst[5] = v48 as u8;
}

/// Read 48-bit big-endian value
fn read_48be(src: &[u8]) -> u64 {
    ((src[0] as u64) << 40)
        | ((src[1] as u64) << 32)
        | ((src[2] as u64) << 24)
        | ((src[3] as u64) << 16)
        | ((src[4] as u64) << 8)
        | (src[5] as u64)
}

/// Read 64-bit little-endian value
fn read_64le(src: &[u8]) -> u64 {
    (src[0] as u64)
        | ((src[1] as u64) << 8)
        | ((src[2] as u64) << 16)
        | ((src[3] as u64) << 24)
        | ((src[4] as u64) << 32)
        | ((src[5] as u64) << 40)
        | ((src[6] as u64) << 48)
        | ((src[7] as u64) << 56)
}

/// SipHash-2-4 implementation
pub fn siphash24(input: &[u8], k0: u64, k1: u64) -> u64 {
    let mut v0 = 0x736f6d6570736575u64 ^ k0;
    let mut v1 = 0x646f72616e646f6du64 ^ k1;
    let mut v2 = 0x6c7967656e657261u64 ^ k0;
    let mut v3 = 0x7465646279746573u64 ^ k1;

    let len = input.len();
    let end = len & !7;

    // Process full 8-byte blocks
    for chunk in input[..end].chunks_exact(8) {
        let m = read_64le(chunk);
        v3 ^= m;

        // 2 compression rounds
        for _ in 0..2 {
            v0 = v0.wrapping_add(v1);
            v2 = v2.wrapping_add(v3);
            v1 = v1.rotate_left(13);
            v3 = v3.rotate_left(16);
            v1 ^= v0;
            v3 ^= v2;
            v0 = v0.rotate_left(32);
            v2 = v2.wrapping_add(v1);
            v0 = v0.wrapping_add(v3);
            v1 = v1.rotate_left(17);
            v3 = v3.rotate_left(21);
            v1 ^= v2;
            v3 ^= v0;
            v2 = v2.rotate_left(32);
        }

        v0 ^= m;
    }

    // Process remaining bytes
    let mut b = (len as u64) << 56;
    let remaining = &input[end..];

    match remaining.len() {
        7 => b |= (remaining[6] as u64) << 48
            | (remaining[5] as u64) << 40
            | (remaining[4] as u64) << 32
            | (remaining[3] as u64) << 24
            | (remaining[2] as u64) << 16
            | (remaining[1] as u64) << 8
            | remaining[0] as u64,
        6 => b |= (remaining[5] as u64) << 40
            | (remaining[4] as u64) << 32
            | (remaining[3] as u64) << 24
            | (remaining[2] as u64) << 16
            | (remaining[1] as u64) << 8
            | remaining[0] as u64,
        5 => b |= (remaining[4] as u64) << 32
            | (remaining[3] as u64) << 24
            | (remaining[2] as u64) << 16
            | (remaining[1] as u64) << 8
            | remaining[0] as u64,
        4 => b |= (remaining[3] as u64) << 24
            | (remaining[2] as u64) << 16
            | (remaining[1] as u64) << 8
            | remaining[0] as u64,
        3 => b |= (remaining[2] as u64) << 16
            | (remaining[1] as u64) << 8
            | remaining[0] as u64,
        2 => b |= (remaining[1] as u64) << 8
            | remaining[0] as u64,
        1 => b |= remaining[0] as u64,
        _ => {}
    }

    v3 ^= b;

    // 2 compression rounds
    for _ in 0..2 {
        v0 = v0.wrapping_add(v1);
        v2 = v2.wrapping_add(v3);
        v1 = v1.rotate_left(13);
        v3 = v3.rotate_left(16);
        v1 ^= v0;
        v3 ^= v2;
        v0 = v0.rotate_left(32);
        v2 = v2.wrapping_add(v1);
        v0 = v0.wrapping_add(v3);
        v1 = v1.rotate_left(17);
        v3 = v3.rotate_left(21);
        v1 ^= v2;
        v3 ^= v0;
        v2 = v2.rotate_left(32);
    }

    v0 ^= b;
    v2 ^= 0xff;

    // 4 finalization rounds
    for _ in 0..4 {
        v0 = v0.wrapping_add(v1);
        v2 = v2.wrapping_add(v3);
        v1 = v1.rotate_left(13);
        v3 = v3.rotate_left(16);
        v1 ^= v0;
        v3 ^= v2;
        v0 = v0.rotate_left(32);
        v2 = v2.wrapping_add(v1);
        v0 = v0.wrapping_add(v3);
        v1 = v1.rotate_left(17);
        v3 = v3.rotate_left(21);
        v1 ^= v2;
        v3 ^= v0;
        v2 = v2.rotate_left(32);
    }

    v0 ^ v1 ^ v2 ^ v3
}

/// Build SipHash input from UUID (works for both v7 and v4 facade)
fn build_sip_input(u: &Uuid128) -> [u8; 10] {
    let mut msg = [0u8; 10];
    // Take exactly the random bits: [low-nibble of b6][b7][b8&0x3F][b9..b15]
    msg[0] = u.bytes[6] & 0x0F;
    msg[1] = u.bytes[7];
    msg[2] = u.bytes[8] & 0x3F;
    msg[3..10].copy_from_slice(&u.bytes[9..16]);
    msg
}

/// Encode a UUIDv7 as a UUIDv4 facade
pub fn encode_v4_facade(v7: Uuid128, key: Uuidv47Key) -> Uuid128 {
    // 1) Generate mask from SipHash of random bits
    let sipmsg = build_sip_input(&v7);
    let mask48 = siphash24(&sipmsg, key.k0, key.k1) & 0x0000_FFFF_FFFF_FFFF;

    // 2) XOR timestamp with mask
    let ts48 = read_48be(&v7.bytes[0..6]);
    let enc_ts = ts48 ^ mask48;

    // 3) Build v4 facade
    let mut out = v7;
    write_48be(&mut out.bytes[0..6], enc_ts);
    out.set_version(4);
    out.set_variant_rfc4122();
    out
}

/// Decode a UUIDv4 facade back to UUIDv7
pub fn decode_v4_facade(v4_facade: Uuid128, key: Uuidv47Key) -> Uuid128 {
    // 1) Rebuild same SipHash input (identical bytes)
    let sipmsg = build_sip_input(&v4_facade);
    let mask48 = siphash24(&sipmsg, key.k0, key.k1) & 0x0000_FFFF_FFFF_FFFF;

    // 2) XOR encoded timestamp with mask to recover original
    let enc_ts = read_48be(&v4_facade.bytes[0..6]);
    let ts48 = enc_ts ^ mask48;

    // 3) Restore v7
    let mut out = v4_facade;
    write_48be(&mut out.bytes[0..6], ts48);
    out.set_version(7);
    out.set_variant_rfc4122();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_48() {
        let mut buf = [0u8; 6];
        let v = 0x0123456789ABu64;
        write_48be(&mut buf, v);
        let r = read_48be(&buf);
        assert_eq!(r, v);
    }

    #[test]
    fn test_uuid_parse_format_roundtrip() {
        let s = "00000000-0000-7000-8000-000000000000";
        let u = Uuid128::parse(s).unwrap();
        assert_eq!(u.version(), 7);

        let formatted = u.format();
        let u2 = Uuid128::parse(&formatted).unwrap();
        assert_eq!(u.bytes, u2.bytes);

        // Test bad format
        let bad = "zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz";
        assert!(Uuid128::parse(bad).is_err());
    }

    #[test]
    fn test_version_variant() {
        let mut u = Uuid128::from_bytes([0; 16]);
        u.set_version(7);
        assert_eq!(u.version(), 7);
        u.set_variant_rfc4122();
        assert_eq!(u.bytes[8] & 0xC0, 0x80);
    }

    #[test]
    fn test_siphash_vectors() {
        // Test vectors from reference implementation
        let k0 = 0x0706050403020100u64;
        let k1 = 0x0f0e0d0c0b0a0908u64;

        let vectors = [
            (0, 0x310e0edd47db6f72u64),
            (1, 0xfd67dc93c539f874u64),
            (2, 0x5a4fa9d909806c0du64),
            (3, 0x2d7efbd796666785u64),
            (4, 0xb7877127e09427cfu64),
            (5, 0x8da699cd64557618u64),
            (6, 0xcee3fe586e46c9cbu64),
            (7, 0x37d1018bf50002abu64),
            (8, 0x6224939a79f5f593u64),
            (9, 0xb0e4a90bdf82009eu64),
            (10, 0xf3b9dd94c5bb5d7au64),
        ];

        let mut msg = Vec::new();
        for i in 0..64 {
            msg.push(i as u8);
        }

        for (len, expected) in vectors.iter() {
            let result = siphash24(&msg[..*len], k0, k1);
            assert_eq!(result, *expected, "Failed for length {}", len);
        }
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let key = Uuidv47Key {
            k0: 0x0123456789abcdefu64,
            k1: 0xfedcba9876543210u64,
        };

        for i in 0..16 {
            let ts = (0x100000u64 * i as u64) + 123;
            let ra = ((0x0AAA ^ (i * 7)) & 0x0FFF) as u16;
            let rb = (0x0123456789ABCDEFu64 ^ (0x1111111111111111u64 * i as u64)) & ((1u64 << 62) - 1);

            let u7 = Uuid128::craft_v7(ts, ra, rb);

            let facade = encode_v4_facade(u7, key);
            assert_eq!(facade.version(), 4);
            assert_eq!(facade.bytes[8] & 0xC0, 0x80);

            let back = decode_v4_facade(facade, key);
            assert_eq!(u7.bytes, back.bytes);

            // Test with wrong key
            let wrong_key = Uuidv47Key {
                k0: key.k0 ^ 0xdeadbeef,
                k1: key.k1 ^ 0x1337,
            };
            let bad = decode_v4_facade(facade, wrong_key);
            assert_ne!(u7.bytes, bad.bytes);
        }
    }

    #[test]
    fn test_build_sip_input_stability() {
        let u7 = Uuid128::craft_v7(0x123456789ABC, 0x0ABC, 0x0123456789ABCDEF & ((1u64 << 62) - 1));
        let key = Uuidv47Key {
            k0: 0x0123456789abcdef,
            k1: 0xfedcba9876543210,
        };

        let facade = encode_v4_facade(u7, key);

        let m1 = build_sip_input(&u7);
        let m2 = build_sip_input(&facade);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_demo_example() {
        let s = "018f2d9f-9a2a-7def-8c3f-7b1a2c4d5e6f";
        let id_v7 = Uuid128::parse(s).unwrap();
        let key = Uuidv47Key {
            k0: 0x0123456789abcdef,
            k1: 0xfedcba9876543210,
        };

        let facade = encode_v4_facade(id_v7, key);
        let back = decode_v4_facade(facade, key);

        assert_eq!(id_v7.bytes, back.bytes);
        assert_eq!(facade.version(), 4);
        assert_eq!(back.version(), 7);
    }
}

// Demo program
#[cfg(feature = "demo")]
fn main() {
    let s = "018f2d9f-9a2a-7def-8c3f-7b1a2c4d5e6f";
    let id_v7 = Uuid128::parse(s).expect("Failed to parse UUID");

    let key = Uuidv47Key {
        k0: 0x0123456789abcdef,
        k1: 0xfedcba9876543210,
    };

    let facade = encode_v4_facade(id_v7, key);
    let back = decode_v4_facade(facade, key);

    println!("v7 in : {}", id_v7);
    println!("v4 out: {}", facade);
    println!("back  : {}", back);
}