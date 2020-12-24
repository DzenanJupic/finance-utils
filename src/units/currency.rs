use serde::{Deserialize, Serialize};
use derive_more::Display;
use crate::units::UnitLike;

/// for explanation of the individual codes, see: [ISO_4217](https://en.wikipedia.org/wiki/ISO_4217)
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    EUR,
    USD,
    GBP,
    CHF,
    CAD,
    AUD,
    JPY,
    RUB,
    SAR,
    #[serde(rename = "XXX")]
    NotACurrency,
}

impl UnitLike for Currency {}

impl Default for Currency {
    fn default() -> Self {
        Self::NotACurrency
    }
}
