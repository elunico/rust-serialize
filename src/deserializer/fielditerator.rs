use crate::{DeserializeError, TSerializable};

use super::utils::read_field;

pub struct FieldIterator<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> FieldIterator<'a> {
    // type Item = Result<(String, T), DeserializeError>;

    pub fn new(data: &'a [u8], offset: usize) -> Self {
        FieldIterator { data, offset }
    }

    pub fn next<T>(&mut self) -> Result<(String, T), DeserializeError>
    where
        T: TSerializable,
    {
        read_field(&mut self.data, &mut self.offset)
    }

    pub fn has_next(&self) -> bool {
        self.offset < self.data.len()
    }
}
