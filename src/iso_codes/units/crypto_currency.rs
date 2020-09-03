use serde::{Serialize, Deserialize};
use crate::iso_codes::units::UnitLike;
use derive_more::Display;

/// for explanation of the individual codes, see: [ISO_4217](https://en.wikipedia.org/wiki/ISO_4217)
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum CryptoCurrency {
    BCH,
    BNB,
    #[serde(alias = "XBT")]
    BTC,
    DASH,
    EOS,
    ETH,
    LTC,
    VTC,
    XML,
    XMR,
    XPR,
    #[serde(rename = "XXX")]
    NotACryptoCurrency,
}

impl UnitLike for CryptoCurrency {}

impl Default for CryptoCurrency {
    fn default() -> Self {
        Self::NotACryptoCurrency
    }
}
