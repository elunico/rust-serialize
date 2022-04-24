use crate::TSerializable;

pub struct Serializer {
    data: Vec<u8>,
    field_count: u8,
}

impl Serializer {
    pub fn new(struct_name: &str) -> Self {
        let mut s = Serializer {
            data: Vec::new(),
            field_count: 0,
        };
        s.data.push(0);
        s.data.push((struct_name.len() & 0xFF).try_into().unwrap());
        s.data.append(&mut struct_name.as_bytes().to_vec());
        s.data.push(0);
        s.data.push(0);
        s
    }

    pub fn add_field<V>(&mut self, name: &str, value: V)
    where
        V: TSerializable,
    {
        self.data.push(name.len() as u8);
        self.data.append(&mut name.as_bytes().to_vec());
        let mut field_data = value.serialize();
        let field_size = field_data.len();
        self.data.append(&mut field_size.to_be_bytes().to_vec());
        self.data.append(&mut field_data);
        self.field_count += 1;
    }

    pub fn done(self) -> Vec<u8> {
        let mut data = self.data;
        data[0] = self.field_count;
        data
    }
}
