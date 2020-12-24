use core::ops::*;

use num_traits::*;
use serde::{Deserializer, Serializer, Serialize};

use crate::primitives::*;
use visitor::PrimitiveValueVisitor;

pub mod visitor;

pub trait PrimitiveValue:
Num + NumCast + ToPrimitive + FromPrimitive + One + Zero + Neg<Output=Self> +
NumOps<Self, Self> + NumOps<f64, Self> + NumOps<F64, Self> + NumOps<Percent, Self> +
AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + RemAssign<Self> +
AddAssign<f64> + SubAssign<f64> + MulAssign<f64> + DivAssign<f64> + RemAssign<f64> +
AddAssign<Percent> + SubAssign<Percent> + MulAssign<Percent> + DivAssign<Percent> + RemAssign<Percent> +
AsRef<f64> + AsMut<f64> +
From<f64> + From<F64> + Into<f64> + Into<F64> +
Copy + core::fmt::Display + Default + PartialEq + PartialOrd {
    // todo: add a value type, to make this trait generic

    const ZERO: Self;
    const ONE: Self;
    const MINUS_ONE: Self;
    const HUNDRED: Self;
    const MINUS_HUNDRED: Self;
    const MIN: Self;
    const MAX: Self;

    fn new(value: f64) -> Self;
    fn as_f64(&self) -> f64;
    fn as_mut_f64(&mut self) -> &mut f64;

    #[allow(non_snake_case)]
    fn as_F64(&self) -> F64 { F64(self.as_f64()) }
    #[allow(non_snake_case)]
    fn from_F64(value: F64) -> Self { Self::new(value.0) }

    fn deserialize_str<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_str(PrimitiveValueVisitor)
            .map(Self::new)
    }
    fn deserialize_f64<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_f64(PrimitiveValueVisitor)
            .map(Self::new)
    }
    fn deserialize_i64<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_i64(PrimitiveValueVisitor)
            .map(Self::new)
    }
    fn deserialize_u64<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_u64(PrimitiveValueVisitor)
            .map(Self::new)
    }
    fn deserialize_any<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_any(PrimitiveValueVisitor)
            .map(Self::new)
    }

    fn serialize_f64_str<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&self.as_f64().to_string())
    }
    fn serialize_option_f64_str<S>(value: &Option<Self>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        match value {
            Some(value) => value.serialize_f64_str(serializer),
            None => Option::<f64>::None.serialize(serializer)
        }
    }
    fn serialize_i64_str<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&(self.as_f64() as i64).to_string())
    }
    fn serialize_u64_str<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&(self.as_f64() as u64).to_string())
    }
    fn serialize_f64<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_f64(self.as_f64())
    }
    fn serialize_i64<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_i64(self.as_f64() as i64)
    }
    fn serialize_u64<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_u64(self.as_f64() as u64)
    }
}
