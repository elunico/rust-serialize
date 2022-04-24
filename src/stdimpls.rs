pub use super::{DeserializeError, TSerializable};

impl TSerializable for f64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(s);
        Ok(f64::from_be_bytes(buf))
    }
}

impl TSerializable for f32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(s);
        Ok(f32::from_be_bytes(buf))
    }
}

impl TSerializable for u8 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 1];
        buf.copy_from_slice(s);
        Ok(u8::from_be_bytes(buf))
    }
}

impl TSerializable for u16 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(s);
        Ok(u16::from_be_bytes(buf))
    }
}

impl TSerializable for u32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(s);
        Ok(u32::from_be_bytes(buf))
    }
}

impl TSerializable for u64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(s);
        Ok(u64::from_be_bytes(buf))
    }
}

impl TSerializable for u128 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(s);
        Ok(u128::from_be_bytes(buf))
    }
}

impl TSerializable for i8 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 1];
        buf.copy_from_slice(s);
        Ok(i8::from_be_bytes(buf))
    }
}

impl TSerializable for i16 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(s);
        Ok(i16::from_be_bytes(buf))
    }
}

impl TSerializable for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(s);
        Ok(i32::from_be_bytes(buf))
    }
}

impl TSerializable for i64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(s);
        Ok(i64::from_be_bytes(buf))
    }
}

impl TSerializable for i128 {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(s);
        Ok(i128::from_be_bytes(buf))
    }
}

impl TSerializable for bool {
    fn serialize(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(s[0] != 0)
    }
}

impl TSerializable for usize {
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(s);
        Ok(usize::from_be_bytes(buf))
    }
}

impl TSerializable for String {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.len().serialize()[..]);
        buf.extend_from_slice(self.as_bytes());
        buf
    }

    fn deserialize(s: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let len = u32::deserialize(&s[..4])?;
        let mut buf = Vec::new();
        buf.extend_from_slice(&s[4..4 + len as usize]);
        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(_) => Err(DeserializeError::UnknownError(
                "Invalid ut8 string".to_string(),
            )),
        }
    }
}
