use std::{convert::TryFrom, fmt::Display, fmt::Formatter, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    // Bit 5 of the byte, which is 32 (2 ^ 5)
    const BIT_5: u8 = 0b0010_0000;

    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        self.bytes()[0] & ChunkType::BIT_5 == 0
    }

    fn is_public(&self) -> bool {
        self.bytes()[1] & ChunkType::BIT_5 == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2] & ChunkType::BIT_5 == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3] & ChunkType::BIT_5 != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self(value))
        } else {
            Err("type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z)")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let mut arr: [u8; 4] = Default::default();
            arr.copy_from_slice(s.as_bytes());
            Self::try_from(arr)
        } else {
            Err("type codes are restricted to 4 characters")
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
