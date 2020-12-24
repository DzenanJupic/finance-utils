use serde::{Deserialize, Serialize, Serializer, Deserializer};
use self::currency::Currency;
use self::crypto_currency::CryptoCurrency;
use self::metal::Metal;
use serde::de::{Error, Unexpected};
use std::fmt::{self, Display};
use derive_more::Display;

pub mod crypto_currency;
pub mod currency;
pub mod metal;

pub trait UnitLike: Copy + Display + Default  {}

/// for explanation of the individual codes, see: [ISO_4217](https://en.wikipedia.org/wiki/ISO_4217)
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Unit {
    NotAUnit(NotAUnit),
    Currency(Currency),
    CryptoCurrency(CryptoCurrency),
    Metal(Metal),
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct NotAUnit;

impl UnitLike for Unit {}

impl Default for Unit {
    fn default() -> Self {
        Self::NotAUnit(NotAUnit)
    }
}

impl UnitLike for NotAUnit {}

impl Display for NotAUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NotAUnit")
    }
}

impl Serialize for NotAUnit {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str("XXX")
    }
}

impl<'de> Deserialize<'de> for NotAUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        match <&str>::deserialize(deserializer)? {
            "XXX" => Ok(Self),
            s => Err(D::Error::invalid_value(Unexpected::Str(s), &"XXX (represents NotAUnit)"))
        }
    }
}
