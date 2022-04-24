pub mod deserializer;
pub mod serializer;
pub mod stdimpls;

pub trait TSerializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength(u8),
    InvalidDataType(u8),
    WrongDataType(u8),
    WrongNameLength(u8),
    WrongName(String),
    UnexpectedByte(u8),
    UnknownError(String),
    InvalidFieldDataConversion,
}

// OR WITH 128 to make reference
