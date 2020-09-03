use serde::de::{Error, Unexpected, Visitor};
use serde::export::fmt;

pub struct PrimitiveValueVisitor;

impl<'de> Visitor<'de> for PrimitiveValueVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a implementor of PrimitiveValue")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error, {
        Ok(v as u8 as f64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error, {
        Ok(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error, {
        Ok(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error, {
        Ok(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where
        E: Error, {
        match v.to_digit(10) {
            Some(d) => Ok(d as f64),
            None => Err(E::invalid_value(Unexpected::Char(v), &self))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error, {
        match v.parse::<f64>() {
            Ok(f) => Ok(f),
            Err(_) => Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error, {
        if v.len() != core::mem::size_of::<f64>() {
            return Err(E::invalid_value(Unexpected::Bytes(v), &"expected exactly 8 bytes (64 bits)"));
        }
        let arr: [u8; 8] = [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]];
        Ok(f64::from_le_bytes(arr))
    }
}
