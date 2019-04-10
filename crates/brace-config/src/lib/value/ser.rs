use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{self, Display};

use serde::ser::{
    Error as SerError, Impossible, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    Serializer,
};

use super::Value;

pub struct ValueSerializer;

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = ArraySeqSerializer;
    type SerializeTuple = ArraySeqSerializer;
    type SerializeTupleStruct = ArraySeqSerializer;
    type SerializeTupleVariant = ArraySeqMatrixSerializer;
    type SerializeMap = TableMapSerializer;
    type SerializeStruct = TableMapSerializer;
    type SerializeStructVariant = TableMapMatrixSerializer;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(value))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported value type: byte array"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported value type: none option"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported value type: unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported value type: unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(variant))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map = HashMap::new();

        map.insert(String::from(variant), value.serialize(self)?);

        Ok(Value::from(map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(ArraySeqSerializer {
            seq: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(ArraySeqMatrixSerializer {
            name: String::from(variant),
            seq: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(TableMapSerializer {
            map: HashMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(TableMapMatrixSerializer {
            name: String::from(variant),
            map: HashMap::new(),
        })
    }
}

pub struct ArraySeqSerializer {
    pub(crate) seq: Vec<Value>,
}

impl SerializeSeq for ArraySeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.seq.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::from(self.seq))
    }
}

impl SerializeTuple for ArraySeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.seq.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::from(self.seq))
    }
}

impl SerializeTupleStruct for ArraySeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.seq.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::from(self.seq))
    }
}

pub struct ArraySeqMatrixSerializer {
    pub(crate) name: String,
    pub(crate) seq: Vec<Value>,
}

impl SerializeTupleVariant for ArraySeqMatrixSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.seq.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        let mut map = HashMap::new();

        map.insert(self.name, Value::from(self.seq));

        Ok(Value::from(map))
    }
}

pub struct TableMapSerializer {
    pub(crate) map: HashMap<String, Value>,
    pub(crate) next_key: Option<String>,
}

impl SerializeMap for TableMapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.next_key = Some(key.serialize(TableKeySerializer)?);

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let key = self.next_key.take();
        let key = key.expect("serialize_value called before serialize_key");

        self.map.insert(key, value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::from(self.map))
    }
}

impl SerializeStruct for TableMapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        SerializeMap::serialize_key(self, key)?;
        SerializeMap::serialize_value(self, value)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

pub struct TableMapMatrixSerializer {
    pub(crate) name: String,
    pub(crate) map: HashMap<String, Value>,
}

impl SerializeStructVariant for TableMapMatrixSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.map
            .insert(String::from(key), value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        let mut map = HashMap::new();

        map.insert(self.name, Value::from(self.map));

        Ok(Value::from(map))
    }
}

pub struct TableKeySerializer;

impl Serializer for TableKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_owned())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: bool"))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: f32"))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: f64"))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: bytes"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: unit struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::custom("unsupported key type: newtype variant"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unsupported key type: none option"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::custom("unsupported key type: some option"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::custom("unsupported key type: seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::custom("unsupported key type: tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::custom("unsupported key type: tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::custom("unsupported key type: tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::custom("unsupported key type: map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::custom("unsupported key type: struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::custom("unsupported key type: variant"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "serialization error"
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}
