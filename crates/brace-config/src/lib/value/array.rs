use std::fmt::{self, Debug};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use super::{ser::ValueSerializer, Error, Value};

#[derive(Clone, PartialEq)]
pub struct Array(pub(crate) Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get<'de, T>(&'de self, key: &str) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.is_empty() {
            Err(Error::custom("empty key"))
        } else {
            match keys[0].parse::<usize>() {
                Ok(key) => {
                    if keys.len() == 1 {
                        match self.0.get(key) {
                            Some(value) => value.val(),
                            None => Err(Error::custom(format!("missing value for '{}'", key))),
                        }
                    } else {
                        match self.0.get(key) {
                            Some(value) => value.get(keys[1]),
                            None => Err(Error::custom(format!("missing value for '{}'", key))),
                        }
                    }
                }
                Err(err) => Err(Error::custom(err)),
            }
        }
    }

    pub fn get_value(&self, key: &str) -> Result<&Value, Error> {
        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.is_empty() {
            Err(Error::custom("empty key"))
        } else {
            match keys[0].parse::<usize>() {
                Ok(key) => {
                    if keys.len() == 1 {
                        match self.0.get(key) {
                            Some(value) => Ok(value),
                            None => Err(Error::custom(format!("missing value for '{}'", key))),
                        }
                    } else {
                        match self.0.get(key) {
                            Some(value) => value.get_value(keys[1]),
                            None => Err(Error::custom(format!("missing value for '{}'", key))),
                        }
                    }
                }
                Err(err) => Err(Error::custom(err)),
            }
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let keys: Vec<&str> = key.splitn(3, '.').collect();

        if keys.is_empty() {
            Err(Error::custom("empty key"))
        } else {
            match keys[0].parse::<usize>() {
                Ok(key) => {
                    if key > self.0.len() {
                        Err(Error::custom("cannot insert item after index"))
                    } else if keys.len() == 1 {
                        self.0.insert(
                            key,
                            value.serialize(ValueSerializer).map_err(Error::custom)?,
                        );

                        Ok(())
                    } else {
                        let peek = keys[1].to_string();
                        let mut tail = keys[1].to_string();

                        if keys.len() == 3 {
                            tail.push('.');
                            tail.push_str(keys[2]);
                        }

                        match self.0.get_mut(key) {
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
                }
                Err(err) => Err(Error::custom(err)),
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
            Err(Error::custom("empty key"))
        } else {
            match keys[0].parse::<usize>() {
                Ok(key) => {
                    if key > self.0.len() {
                        Err(Error::custom("cannot insert item after index"))
                    } else if keys.len() == 1 {
                        self.0.insert(key, value);

                        Ok(())
                    } else {
                        let peek = keys[1].to_string();
                        let mut tail = keys[1].to_string();

                        if keys.len() == 3 {
                            tail.push_str(keys[2]);
                        }

                        match self.0.get_mut(key) {
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
                }
                Err(err) => Err(Error::custom(err)),
            }
        }
    }

    pub fn set_value_default(&mut self, key: &str, value: Value) -> Result<(), Error> {
        if self.get_value(key).is_err() {
            self.set_value(key, value)?;
        }

        Ok(())
    }

    pub fn merge(&mut self, array: &Self) -> Result<(), Error> {
        for (idx, val) in array.0.iter().enumerate() {
            match self.0.get_mut(idx) {
                Some(item) => {
                    item.merge(&val)?;
                }
                None => {
                    self.0.push(val.clone());
                }
            }
        }

        Ok(())
    }

    pub fn merge_default(&mut self, array: &Self) -> Result<(), Error> {
        for (idx, val) in array.0.iter().enumerate() {
            match self.0.get_mut(idx) {
                Some(item) => {
                    item.merge_default(&val)?;
                }
                None => {
                    self.0.push(val.clone());
                }
            }
        }

        Ok(())
    }

    pub fn merge_value(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Entry(_) => Err(Error::custom("cannot merge entry with array")),
            Value::Table(_) => Err(Error::custom("cannot merge table with array")),
            Value::Array(array) => self.merge(array),
        }
    }

    pub fn merge_value_default(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Entry(_) => Err(Error::custom("cannot merge entry with array")),
            Value::Table(_) => Err(Error::custom("cannot merge table with array")),
            Value::Array(array) => self.merge_default(array),
        }
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Array {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(formatter)
    }
}

impl Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for element in &self.0 {
            seq.serialize_element(&element)?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct ArrayVisitor;

        impl<'de> Visitor<'de> for ArrayVisitor {
            type Value = Array;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid array")
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

                Ok(Array(vec))
            }
        }

        deserializer.deserialize_any(ArrayVisitor)
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = Iter<'a, Value>;

    fn into_iter(self) -> Iter<'a, Value> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = IterMut<'a, Value>;

    fn into_iter(self) -> IterMut<'a, Value> {
        self.0.iter_mut()
    }
}

impl From<Vec<Value>> for Array {
    fn from(vec: Vec<Value>) -> Self {
        Self(vec)
    }
}
