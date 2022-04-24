use crate::DeserializeError;

use self::fielditerator::FieldIterator;
use self::utils::{read_field_count, read_struct_name};

pub mod fielditerator;
pub(crate) mod utils;

pub struct Deserializer {
    data: Vec<u8>,
    offset_field_count: usize,
    offset_struct_name: usize,
    lazy_offset_blanks: Option<usize>,
    lazy_offset_fields_begin: Option<usize>,
    lazy_offset_moving: Option<usize>,
}

impl Deserializer {
    pub fn new(data: &[u8]) -> Self {
        Deserializer {
            data: data.to_vec(),
            offset_field_count: 0,
            offset_struct_name: 1,
            lazy_offset_blanks: None,
            lazy_offset_fields_begin: None,
            lazy_offset_moving: None,
        }
    }

    pub fn struct_name(&mut self) -> Result<String, DeserializeError> {
        let mut offset = self.offset_struct_name;
        match read_struct_name(&self.data, &mut offset, None) {
            Ok(s) => {
                self.lazy_offset_blanks = Some(offset);
                self.lazy_offset_fields_begin = Some(offset + 2);
                self.lazy_offset_moving = self.lazy_offset_fields_begin;
                Ok(s)
            }
            Err(e) => Err(e),
        }
    }

    pub fn field_count(&self) -> u8 {
        let mut offset = self.offset_field_count;
        read_field_count(&self.data, &mut offset)
    }

    pub fn fields(&mut self) -> Result<FieldIterator, DeserializeError> {
        if let Some(offset) = self.lazy_offset_fields_begin {
            Ok(FieldIterator::new(&self.data[offset..], 0))
        } else {
            let _ = self.struct_name();
            if let Some(offset) = self.lazy_offset_fields_begin {
                Ok(FieldIterator::new(&self.data[offset..], 0))
            } else {
                Err(DeserializeError::UnknownError(
                    "Could not calculator fields offset".to_owned(),
                ))
            }
        }
    }
}
