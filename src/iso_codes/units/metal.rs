use serde::{Serialize, Deserialize};
use crate::iso_codes::units::UnitLike;
use derive_more::Display;

/// for explanation of the individual codes, see: [ISO_4217](https://en.wikipedia.org/wiki/ISO_4217)
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metal {
    #[serde(rename = "XAU")]
    Gold,
    #[serde(rename = "XAG")]
    Silver,
    #[serde(rename = "XPD")]
    Palladium,
    #[serde(rename = "XPT")]
    Platinum,
    #[serde(rename = "XXX")]
    NotAMetal,
}

impl UnitLike for Metal {}

impl Default for Metal {
    fn default() -> Self {
        Self::NotAMetal
    }
}
