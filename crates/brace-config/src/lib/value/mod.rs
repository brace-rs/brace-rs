use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

use serde::de::{
    Deserialize, DeserializeOwned, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};
use serde::ser::{Serialize, Serializer};

use self::array::Array;
use self::de::{Error as DeError, ValueDeserializer};
use self::entry::Entry;
use self::ser::ValueSerializer;
use self::table::Table;

pub mod array;
pub mod de;
pub mod entry;
pub mod ser;
pub mod table;

pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(ValueDeserializer::new(&value)).map_err(Error::custom)
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(ValueSerializer).map_err(Error::custom)
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Entry(Entry),
    Array(Array),
    Table(Table),
}

impl Value {
    pub fn entry() -> Self {
        Value::Entry(Entry::new())
    }

    pub fn table() -> Self {
        Value::Table(Table::new())
    }

    pub fn array() -> Self {
        Value::Array(Array::new())
    }

    pub fn val<'de, T>(&'de self) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        T::deserialize(ValueDeserializer::new(self)).map_err(Error::custom)
    }

    pub fn get<'de, T>(&'de self, key: &str) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        match self {
            Value::Entry(_) => Err(Error::custom("cannot call `get` on entry")),
            Value::Array(array) => array.get(key),
            Value::Table(table) => table.get(key),
        }
    }

    pub fn get_value(&self, key: &str) -> Result<&Value, Error> {
        match self {
            Value::Entry(_) => Err(Error::custom("cannot call `get_value` on entry")),
            Value::Array(array) => array.get_value(key),
            Value::Table(table) => table.get_value(key),
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        match self {
            Value::Entry(_) => Err(Error::custom("cannot call `set` on entry variant")),
            Value::Array(array) => array.set(key, value),
            Value::Table(table) => table.set(key, value),
        }
    }

    pub fn set_value(&mut self, key: &str, value: Value) -> Result<(), Error> {
        match self {
            Value::Entry(_) => Err(Error::custom("cannot call `set_value` on entry variant")),
            Value::Array(array) => array.set_value(key, value),
            Value::Table(table) => table.set_value(key, value),
        }
    }

    pub fn merge(&mut self, value: &Self) -> Result<(), Error> {
        match self {
            Value::Entry(entry) => entry.merge_value(value),
            Value::Array(array) => array.merge_value(value),
            Value::Table(table) => table.merge_value(value),
        }
    }

    pub fn merge_value(&mut self, value: &Value) -> Result<(), Error> {
        self.merge(value)
    }

    pub fn as_entry(&self) -> Option<&Entry> {
        match self {
            Value::Entry(entry) => Some(entry),
            Value::Array(_) => None,
            Value::Table(_) => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Entry(_) => None,
            Value::Array(array) => Some(array),
            Value::Table(_) => None,
        }
    }

    pub fn as_table(&self) -> Option<&Table> {
        match self {
            Value::Entry(_) => None,
            Value::Array(_) => None,
            Value::Table(table) => Some(table),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Value::Entry(entry) => entry.fmt(formatter),
            Value::Array(array) => array.fmt(formatter),
            Value::Table(table) => table.fmt(formatter),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Entry(entry) => entry.serialize(serializer),
            Value::Array(array) => array.serialize(serializer),
            Value::Table(table) => table.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::from(value))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::from(vec))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut map = HashMap::new();

                while let Some(key) = visitor.next_key()? {
                    map.insert(key, visitor.next_value()?);
                }

                Ok(Value::from(map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'de> IntoDeserializer<'de, DeError> for &'de Value {
    type Deserializer = ValueDeserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer::new(self)
    }
}

impl From<Entry> for Value {
    fn from(value: Entry) -> Self {
        Value::Entry(value)
    }
}

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Value::Array(value)
    }
}

impl From<Table> for Value {
    fn from(value: Table) -> Self {
        Value::Table(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(Array::from(value))
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Table(Table::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl Error {
    pub fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "value error"
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use super::Value;

    #[test]
    fn test_boolean() {
        let mut table = Value::table();

        assert!(table.set("true", true).is_ok());
        assert!(table.set("false", false).is_ok());
        assert_eq!(table.get::<bool>("true").unwrap(), true);
        assert_eq!(table.get::<bool>("false").unwrap(), false);
    }

    #[test]
    fn test_integer_signed() {
        let mut table = Value::table();

        assert!(table.set("i8", 8 as i8).is_ok());
        assert!(table.set("i16", 16 as i16).is_ok());
        assert!(table.set("i32", 32 as i32).is_ok());
        assert!(table.set("i64", 64 as i64).is_ok());
        assert!(table.set("i128", 128 as i128).is_ok());

        assert_eq!(table.get::<i8>("i8").unwrap(), 8);
        assert_eq!(table.get::<i16>("i8").unwrap(), 8);
        assert_eq!(table.get::<i32>("i8").unwrap(), 8);
        assert_eq!(table.get::<i64>("i8").unwrap(), 8);
        assert_eq!(table.get::<i128>("i8").unwrap(), 8);
        assert_eq!(table.get::<String>("i8").unwrap(), "8");
    }

    #[test]
    fn test_integer_unsigned() {
        let mut table = Value::table();

        assert!(table.set("u8", 8 as u8).is_ok());
        assert!(table.set("u16", 16 as u16).is_ok());
        assert!(table.set("u32", 32 as u32).is_ok());
        assert!(table.set("u64", 64 as u64).is_ok());
        assert!(table.set("u128", 128 as u128).is_ok());

        assert_eq!(table.get::<u8>("u8").unwrap(), 8);
        assert_eq!(table.get::<u16>("u8").unwrap(), 8);
        assert_eq!(table.get::<u32>("u8").unwrap(), 8);
        assert_eq!(table.get::<u64>("u8").unwrap(), 8);
        assert_eq!(table.get::<u128>("u8").unwrap(), 8);
        assert_eq!(table.get::<String>("u8").unwrap(), "8");
    }

    #[test]
    fn test_float() {
        let mut table = Value::table();

        assert!(table.set("f32", 32 as f32).is_ok());
        assert!(table.set("f64", 64 as f64).is_ok());
    }

    #[test]
    fn test_text() {
        let mut table = Value::table();

        assert!(table.set("char", 'c').is_ok());
        assert!(table.set("str", "str").is_ok());
        assert!(table.set("string", String::from("string")).is_ok());

        assert_eq!(table.get::<char>("char").unwrap(), 'c');
        assert_eq!(table.get::<String>("char").unwrap(), "c".to_string());
    }

    #[test]
    fn test_tuple() {
        let mut table = Value::table();

        assert!(table.set("tuple", ('a', "bee", 7, false)).is_ok());
        assert!(table.get_value("tuple").unwrap().as_array().is_some());
        assert!(table.get::<(char, String, usize, bool)>("tuple").is_ok());
        assert!(table
            .get::<(String, String, String, String)>("tuple")
            .is_ok());
        assert_eq!(
            table
                .get::<(String, String, String, String)>("tuple")
                .unwrap(),
            (
                "a".to_string(),
                "bee".to_string(),
                "7".to_string(),
                "false".to_string()
            )
        );
    }

    #[test]
    fn test_struct() {
        let mut table = Value::table();

        #[derive(Serialize, Deserialize)]
        struct A {
            one: String,
            two: HashMap<String, Vec<String>>,
        }

        let mut map = HashMap::<String, Vec<String>>::new();

        map.insert(
            "a".to_string(),
            vec!["hello".to_string(), "world".to_string()],
        );
        map.insert("b".to_string(), Vec::new());

        let a = A {
            one: "one".to_string(),
            two: map,
        };

        assert!(table.set("struct", a).is_ok());
        assert!(table.get::<A>("struct").is_ok());
        assert_eq!(table.get::<A>("struct").unwrap().one, "one".to_string());
        assert_eq!(table.get::<A>("struct").unwrap().two.len(), 2);
    }

    #[test]
    fn test_unit() {
        let mut table = Value::table();

        #[derive(Serialize, Deserialize)]
        struct Unit;

        assert!(table.set("unit", ()).is_err());
        assert!(table.set("unit_struct", Unit).is_err());
    }

    #[test]
    fn test_table() {
        let mut table = Value::table();
        let mut m1 = HashMap::<String, HashMap<String, String>>::new();
        let mut m2 = HashMap::<String, String>::new();

        m2.insert("g".to_string(), "h".to_string());
        m1.insert("f".to_string(), m2);

        assert!(table.set("a.b.c", "d").is_ok());
        assert!(table.get::<HashMap<String, String>>("a.b").is_ok());
        assert_eq!(table.get::<String>("a.b.c").unwrap(), "d");

        assert!(table.set("e", m1).is_ok());
        assert!(table.get::<HashMap<String, String>>("e.f").is_ok());
        assert!(table
            .get::<HashMap<String, HashMap<String, String>>>("e")
            .is_ok());
        assert_eq!(table.get::<String>("e.f.g").unwrap(), "h");
    }

    #[test]
    fn test_array() {
        let mut table = Value::table();

        assert!(table.set("item.1", "a").is_err());
        assert!(table.set("item.0", "b").is_ok());
        assert!(table.set("item.1", "c").is_ok());

        let mut array = Value::array();

        assert!(array.set("1", "a").is_err());
        assert!(array.set("0", "b").is_ok());
        assert!(array.set("1", "c").is_ok());
        assert!(array.set("d", "d").is_err());

        assert!(table.set("array", array).is_ok());
        assert!(table.get::<Vec<String>>("array").is_ok());
        assert_eq!(table.get::<Vec<String>>("array").unwrap().len(), 2);
    }
}
