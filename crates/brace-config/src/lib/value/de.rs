use std::error::Error as StdError;
use std::fmt::{self, Display};

use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{Deserializer, Error as DeError, Visitor};
use serde::forward_to_deserialize_any;

use super::Value;
use crate::value::array::Array;
use crate::value::entry::Entry;
use crate::value::table::Table;

pub struct ValueDeserializer<'de>(&'de Value);

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: &'de Value) -> Self {
        Self(value)
    }

    pub fn deserialize_entry<V>(self, entry: &'de Entry, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&entry.0)
    }

    pub fn deserialize_array<V>(self, array: &'de Array, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let mut deserializer = SeqDeserializer::new(array.into_iter());
        let seq = visitor.visit_seq(&mut deserializer)?;

        deserializer.end()?;

        Ok(seq)
    }

    pub fn deserialize_table<V>(self, table: &'de Table, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let iter = table
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value));
        let mut deserializer = MapDeserializer::new(iter);
        let map = visitor.visit_map(&mut deserializer)?;

        deserializer.end()?;

        Ok(map)
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Entry(entry) => self.deserialize_entry(entry, visitor),
            Value::Array(array) => self.deserialize_array(array, visitor),
            Value::Table(table) => self.deserialize_table(table, visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as bool")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as bool")),
            Value::Entry(entry) => match entry.0.parse::<bool>() {
                Ok(value) => visitor.visit_bool(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i8")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i8")),
            Value::Entry(entry) => match entry.0.parse::<i8>() {
                Ok(value) => visitor.visit_i8(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i16")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i16")),
            Value::Entry(entry) => match entry.0.parse::<i16>() {
                Ok(value) => visitor.visit_i16(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i32")),
            Value::Entry(entry) => match entry.0.parse::<i32>() {
                Ok(value) => visitor.visit_i32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i64")),
            Value::Entry(entry) => match entry.0.parse::<i64>() {
                Ok(value) => visitor.visit_i64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i128")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i128")),
            Value::Entry(entry) => match entry.0.parse::<i128>() {
                Ok(value) => visitor.visit_i128(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u8")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u8")),
            Value::Entry(entry) => match entry.0.parse::<u8>() {
                Ok(value) => visitor.visit_u8(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u16")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u16")),
            Value::Entry(entry) => match entry.0.parse::<u16>() {
                Ok(value) => visitor.visit_u16(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u32")),
            Value::Entry(entry) => match entry.0.parse::<u32>() {
                Ok(value) => visitor.visit_u32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u64")),
            Value::Entry(entry) => match entry.0.parse::<u64>() {
                Ok(value) => visitor.visit_u64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u128")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u128")),
            Value::Entry(entry) => match entry.0.parse::<u128>() {
                Ok(value) => visitor.visit_u128(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as f32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as f32")),
            Value::Entry(entry) => match entry.0.parse::<f32>() {
                Ok(value) => visitor.visit_f32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as f64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as f64")),
            Value::Entry(entry) => match entry.0.parse::<f64>() {
                Ok(value) => visitor.visit_f64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as char")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as char")),
            Value::Entry(entry) => match entry.0.parse::<char>() {
                Ok(value) => visitor.visit_char(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as str")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as str")),
            Value::Entry(entry) => visitor.visit_str(&entry.0),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as string")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as string")),
            Value::Entry(entry) => visitor.visit_str(&entry.0),
        }
    }

    forward_to_deserialize_any! {
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
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
        "deserialization error"
    }
}

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}
