use crc::crc32::checksum_ieee;

use crate::{chunk_type::ChunkType, Error};
use std::{convert::TryFrom, fmt::Display, string::FromUtf8Error};

pub struct Chunk(Vec<u8>);

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: &[u8]) -> Self {
        let data_length = data.len() as u32;
        let crc = chunk_type
            .bytes()
            .iter()
            .chain(data)
            .copied()
            .collect::<Vec<u8>>();

        Self(
            data_length
                .to_be_bytes()
                .iter()
                .chain(chunk_type.to_string().as_bytes())
                .chain(data)
                .chain(checksum_ieee(&crc).to_be_bytes().iter())
                .copied()
                .collect(),
        )
    }

    fn length(&self) -> u32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.0[0..4]);
        u32::from_be_bytes(bytes)
    }

    pub fn chunk_type(&self) -> ChunkType {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.0[4..8]);
        ChunkType::try_from(bytes).expect("Invalid chunk type")
    }

    fn data(&self) -> &[u8] {
        let len = self.length() as usize + 8;
        &self.0[8..len]
    }

    fn crc(&self) -> u32 {
        let len = self.length() as usize + 8;
        checksum_ieee(&self.0[4..len])
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data().to_vec())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let (remainder, crc) = bytes.split_at(bytes.len() - 4);
        let mut buffer = [0; 4];
        buffer.copy_from_slice(crc);
        let crc = u32::from_be_bytes(buffer);
        if checksum_ieee(&remainder[4..]) == crc {
            Ok(Chunk(bytes.iter().copied().collect()))
        } else {
            Err("Invalid crc checks")?
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
