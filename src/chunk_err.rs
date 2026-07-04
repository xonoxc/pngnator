use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("invalid chunk type")]
    InvalidChunkType,

    #[error("invalid data format")]
    InvalidDataFormat,

    #[error("invalid crc")]
    InvalidCrc,
}
