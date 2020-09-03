use serde::de::{Error, MapAccess, Visitor};
use serde::export::{fmt};

use crate::price_map_visitor;
use super::Price;

pub struct RawPriceCurrencyVisitor;
pub struct UnitValueVisitor;

price_map_visitor!(RawPriceCurrencyVisitor { raw_price: "price", currency: "currency" });
price_map_visitor!(UnitValueVisitor { raw_price: "value", currency: "unit" });

fn assign_to_none<T, E>(assign_to: &mut Option<T>, value: T, field: &'static str) -> Result<(), E>
    where E: Error {
    match assign_to {
        Some(_) => Err(Error::duplicate_field(field)),
        None => {
            *assign_to = Some(value);
            Ok(())
        }
    }
}
