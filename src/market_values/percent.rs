use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Percent(pub(crate) f64);

impl_market_value!(impl Percent, one=0.01);
impl_market_value!(Percent, percent);

impl Percent {
    pub const fn as_decimal(&self) -> f64 {
        self.0 * 100.0
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.2}%", self.as_decimal())
    }
}
