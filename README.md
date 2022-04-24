# Rust Serializing
A custom binary format for serializing structures in Rust

## Format description
The data is serialized as a sequence of bytes. We represent this usually using a slice like `&[u8]` or a Vec like `Vec<u8>`

The format is as follows
   -  Part A  -  `u8`: the number of fields in the struct
   -  Part B  -  `u8`: the length of the name of the struct
   -  Part C  -  Part B number of `u8`: The UTF-8 encoded name of the struct
   -  Part D  -  16 zero bits - Indicates separation between the header of the struct (Parts A to C) from the fields. This allows the header to be expanded in the future by adding more sections. Implementations should seek to find 16 continuous 0 bits before attempting to read fields as more data in the future may be slotted between Part C and the 16 zero bits indicating the start of the field
   -  Part E  -  `u8`: length of the name of the first field
   -  Part F  -  Part E number of `u8`: The UTF-8 encoded name of the first field of the struct
   -  Part G  - 8 x `u8`: The Big Endian size of the serialized field. This represents a usize in Big Endian format. It is the length of bytes that compose the serialized field
   -  Part H  -  Part G number of `u8`: The data composing the value. The value must conform to the `TSerializable` trait and have its own deserialize method.

  Parts E through H repeat for Part A repetitions for each field in the struct

## Implementing Serialization
Serializiation is accomplished through the implementation of the `TSerializable` trait.

You must implement `TSerializable::serialize(&self) -> Vec<u8>` and `TSerializable::unserialize(Vec<u8>) -> Self`. The implementation should follow the spec. This is not strictly necessary since the methods just take a vector of bytes, but in order to use the other tooling and functions in this library, you must conform to the protcol as specified.

## Serializing
Serializing can be done with the `Serializer` struct

Simply create the struct using `Serializer::new(String)` with the name of your struct. Then make a call to `Serializer::add_field(String, V)` for
each field in your struct (this will be the serialization and deserialization order, so it is recommended to add fields in declared order).
Finally call `Serializer::done()` to get the `Vec<u8>` result

## Deserializing
Deserializing can be done with the `Deserializer` struct.

This struct can be created from a `Vec<u8>` and it provides access to the relevant data such as struct name and field count.
It also provides the `fields()` method which returns a `FieldIterator`. This struct has a `next<T>() -> (String, T)` method which returns the
name of the field and the value that was serialized for it. This method goes in order of the fields serialization from first to last. It also has
the `has_next() -> bool` method that tells you when the fields are exhausted, though this should not be needed as all fields should be serialized and deserialized in the process of implementing the trait.


## Example
Below you can see an example of a Point class that
implements `TSerializable` in the recommended way

```rust
use rust_serialize::deserializer::Deserializer;
use rust_serialize::serializer::Serializer;
use rust_serialize::{DeserializeError, TSerializable};

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl TSerializable for Point {
    fn serialize(&self) -> Vec<u8> {
        let mut s = Serializer::new("Point");
        s.add_field("x", self.x);
        s.add_field("y", self.y);
        s.done()
    }

    fn deserialize(data: &[u8]) -> Result<Point, DeserializeError> {
        let mut d = Deserializer::new(data);

        // ensure the correct class and version is deserializing
        let name = d.struct_name();
        assert_eq!(name, "Point");
        let count = d.field_count();
        assert_eq!(count, 2);

        let mut iterator = d.fields()?;
        let (_name, x) = iterator.next()?;
        let (_name, y) = iterator.next()?;

        return Ok(Point { x, y });
    }
}
```
