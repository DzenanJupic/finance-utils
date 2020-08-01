use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Points(f64);

impl_market_value!(impl Points, one=1.0);
impl_market_value!(Points);

impl fmt::Display for Points {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.2}%", self.0)
    }
}
