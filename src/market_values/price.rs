use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Price(f64);

impl_market_value!(impl Price, one=1.0);
impl_market_value!(Price);

impl fmt::Display for Price {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.2}%", self.0)
    }
}
