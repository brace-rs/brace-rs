use std::fmt::{self, Debug};

use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use super::{Error, Value};

#[derive(Clone, PartialEq)]
pub struct Entry(pub(crate) String);

impl Entry {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn merge(&mut self, entry: &Self) -> Result<(), Error> {
        self.0 = entry.0.clone();

        Ok(())
    }

    pub fn merge_value(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Array(_) => Err(Error::custom("cannot merge array with entry")),
            Value::Table(_) => Err(Error::custom("cannot merge table with entry")),
            Value::Entry(entry) => self.merge(entry),
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Entry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(formatter)
    }
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct EntryVisitor;

        impl<'de> Visitor<'de> for EntryVisitor {
            type Value = Entry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid entry")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_char<E>(self, value: char) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(Entry::from(value))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }
        }

        deserializer.deserialize_any(EntryVisitor)
    }
}

impl From<bool> for Entry {
    fn from(value: bool) -> Self {
        Entry(value.to_string())
    }
}

impl From<i8> for Entry {
    fn from(value: i8) -> Self {
        Entry(value.to_string())
    }
}

impl From<i16> for Entry {
    fn from(value: i16) -> Self {
        Entry(value.to_string())
    }
}

impl From<i32> for Entry {
    fn from(value: i32) -> Self {
        Entry(value.to_string())
    }
}

impl From<i64> for Entry {
    fn from(value: i64) -> Self {
        Entry(value.to_string())
    }
}

impl From<i128> for Entry {
    fn from(value: i128) -> Self {
        Entry(value.to_string())
    }
}

impl From<u8> for Entry {
    fn from(value: u8) -> Self {
        Entry(value.to_string())
    }
}

impl From<u16> for Entry {
    fn from(value: u16) -> Self {
        Entry(value.to_string())
    }
}

impl From<u32> for Entry {
    fn from(value: u32) -> Self {
        Entry(value.to_string())
    }
}

impl From<u64> for Entry {
    fn from(value: u64) -> Self {
        Entry(value.to_string())
    }
}

impl From<u128> for Entry {
    fn from(value: u128) -> Self {
        Entry(value.to_string())
    }
}

impl From<f32> for Entry {
    fn from(value: f32) -> Self {
        Entry(value.to_string())
    }
}

impl From<f64> for Entry {
    fn from(value: f64) -> Self {
        Entry(value.to_string())
    }
}

impl From<char> for Entry {
    fn from(value: char) -> Self {
        Entry(value.to_string())
    }
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        Entry(value.to_string())
    }
}

impl From<String> for Entry {
    fn from(value: String) -> Self {
        Entry(value)
    }
}
