use std::collections::hash_map::{HashMap, IntoIter, Iter, IterMut};
use std::fmt::{self, Debug};

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use super::{ser::ValueSerializer, Error, Value};

#[derive(Clone, PartialEq)]
pub struct Table(pub(crate) HashMap<String, Value>);

impl Table {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get<'de, T>(&'de self, key: &str) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.is_empty() {
            return Err(Error::custom("empty key"));
        }

        if keys.len() == 1 {
            match self.0.get(key) {
                Some(value) => value.val(),
                None => Err(Error::custom(format!("missing value for '{}'", key))),
            }
        } else {
            match self.0.get(keys[0]) {
                Some(value) => value.get(keys[1]),
                None => Err(Error::custom(format!("missing value for '{}'", key))),
            }
        }
    }

    pub fn get_value(&self, key: &str) -> Result<&Value, Error> {
        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.is_empty() {
            Err(Error::custom("empty key"))
        } else if keys.len() == 1 {
            match self.0.get(key) {
                Some(value) => Ok(value),
                None => Err(Error::custom(format!("missing value for '{}'", key))),
            }
        } else {
            match self.0.get(keys[0]) {
                Some(value) => value.get_value(keys[1]),
                None => Err(Error::custom(format!("missing value for '{}'", key))),
            }
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let keys: Vec<&str> = key.splitn(3, '.').collect();

        if keys.is_empty() {
            return Err(Error::custom("empty key"));
        }

        let key = keys[0].to_string();

        if keys.len() == 1 {
            self.0.insert(
                key,
                value.serialize(ValueSerializer).map_err(Error::custom)?,
            );

            return Ok(());
        }

        let peek = keys[1].to_string();
        let mut tail = keys[1].to_string();

        if keys.len() == 3 {
            tail.push('.');
            tail.push_str(keys[2]);
        }

        match self.0.get_mut(keys[0]) {
            Some(val) => match val {
                Value::Entry(_) => Err(Error::custom("cannot insert into entry")),
                Value::Array(array) => array.set(&tail, value),
                Value::Table(table) => table.set(&tail, value),
            },
            None => {
                match peek.parse::<usize>() {
                    Ok(_) => {
                        let mut array = Value::array();

                        array.set(&tail, value)?;
                        self.0.insert(key, array);
                    }
                    Err(_) => {
                        let mut table = Value::table();

                        table.set(&tail, value)?;
                        self.0.insert(key, table);
                    }
                }

                Ok(())
            }
        }
    }

    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        if self.get_value(key).is_err() {
            self.set(key, value)?;
        }

        Ok(())
    }

    pub fn set_value(&mut self, key: &str, value: Value) -> Result<(), Error> {
        let keys: Vec<&str> = key.splitn(3, '.').collect();

        if keys.is_empty() {
            return Err(Error::custom("empty key"));
        }

        let key = keys[0].to_string();

        if keys.len() == 1 {
            self.0.insert(key, value);

            return Ok(());
        }

        let peek = keys[1].to_string();
        let mut tail = keys[1].to_string();

        if keys.len() == 3 {
            tail.push_str(keys[2]);
        }

        match self.0.get_mut(&key) {
            Some(val) => val.set_value(keys[1], value),
            None => {
                match peek.parse::<usize>() {
                    Ok(_) => {
                        let mut array = Value::array();

                        array.set_value(&tail, value)?;
                        self.0.insert(key, array);
                    }
                    Err(_) => {
                        let mut table = Value::table();

                        table.set_value(&tail, value)?;
                        self.0.insert(key, table);
                    }
                }

                Ok(())
            }
        }
    }

    pub fn set_value_default(&mut self, key: &str, value: Value) -> Result<(), Error> {
        if self.get_value(key).is_err() {
            self.set_value(key, value)?;
        }

        Ok(())
    }

    pub fn merge(&mut self, table: &Self) -> Result<(), Error> {
        for (key, val) in &table.0 {
            match self.0.get_mut(key) {
                Some(item) => {
                    item.merge(&val)?;
                }
                None => {
                    self.0.insert(key.to_owned(), val.clone());
                }
            }
        }

        Ok(())
    }

    pub fn merge_default(&mut self, table: &Self) -> Result<(), Error> {
        for (key, val) in &table.0 {
            match self.0.get_mut(key) {
                Some(item) => {
                    item.merge_default(&val)?;
                }
                None => {
                    self.0.insert(key.to_owned(), val.clone());
                }
            }
        }

        Ok(())
    }

    pub fn merge_value(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Entry(_) => Err(Error::custom("cannot merge entry with table")),
            Value::Array(_) => Err(Error::custom("cannot merge array with table")),
            Value::Table(table) => self.merge(table),
        }
    }

    pub fn merge_value_default(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Entry(_) => Err(Error::custom("cannot merge entry with table")),
            Value::Array(_) => Err(Error::custom("cannot merge array with table")),
            Value::Table(table) => self.merge_default(table),
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(formatter)
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (key, value) in &self.0 {
            map.serialize_entry(&key, &value)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct TableVisitor;

        impl<'de> Visitor<'de> for TableVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid table")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut map = HashMap::new();

                while let Some(key) = visitor.next_key()? {
                    map.insert(key, visitor.next_value()?);
                }

                Ok(Table(map))
            }
        }

        deserializer.deserialize_any(TableVisitor)
    }
}

impl IntoIterator for Table {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a, String, Value>;

    fn into_iter(self) -> Iter<'a, String, Value> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Table {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = IterMut<'a, String, Value>;

    fn into_iter(self) -> IterMut<'a, String, Value> {
        self.0.iter_mut()
    }
}

impl From<HashMap<String, Value>> for Table {
    fn from(map: HashMap<String, Value>) -> Self {
        Self(map)
    }
}
