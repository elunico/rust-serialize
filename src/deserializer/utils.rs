use super::*;

pub(crate) fn read_field_count(data: &[u8], offset: &mut usize) -> u8 {
    let count = data[*offset];
    *offset += 1;
    count
}

pub(crate) fn ensure_separator(data: &[u8], offset: &mut usize) -> Result<(), DeserializeError> {
    if data[*offset] != 0 {
        return Err(DeserializeError::UnexpectedByte(data[*offset]));
    }
    if data[*offset + 1] != 0 {
        return Err(DeserializeError::UnexpectedByte(data[*offset + 1]));
    }
    *offset += 2;
    Ok(())
}

pub(crate) fn read_struct_name(
    data: &[u8],
    offset: &mut usize,
    expected_name: Option<&str>,
) -> Result<String, DeserializeError> {
    let name_length = data[*offset] as usize;
    *offset += 1;
    let name = data[*offset..*offset + name_length as usize]
        .iter()
        .map(|&c| c as char)
        .collect::<String>();
    if expected_name.is_some() && name != expected_name.unwrap() {
        return Err(DeserializeError::WrongName(name));
    }
    *offset += name_length;
    return Ok(name);
}

/// Returns the name of the field, its value and the new offset.
/// The value is deserialized using TSerializable::deserialize.
/// The new offset is the start of the next field. No bounds checking is done.
/// The offset should be given in and is returned in the number of RAW BYTES from the start of the data IT IS NOT the number of bytes from the start of the fields.
pub(crate) fn read_field<T>(
    data: &[u8],
    offset_bytes: &mut usize,
) -> Result<(String, T), DeserializeError>
where
    T: crate::TSerializable,
{
    let field_length = data[*offset_bytes];
    *offset_bytes += 1;
    let field_name =
        std::str::from_utf8(&data[*offset_bytes..*offset_bytes + field_length as usize]);
    if field_name.is_err() {
        return Err(DeserializeError::WrongName(format!(
            "{:?}",
            field_name.unwrap_err()
        )));
    }
    *offset_bytes += field_length as usize;
    let field_size =
        usize::from_be_bytes(data[*offset_bytes..*offset_bytes + 8].try_into().unwrap());
    *offset_bytes += 8;
    let x = T::deserialize(&data[*offset_bytes..*offset_bytes + field_size])?;
    *offset_bytes += field_size;
    Ok((field_name.unwrap().to_string(), x))
}
