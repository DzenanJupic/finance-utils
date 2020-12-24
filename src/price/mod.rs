use derive_more::*;
use serde::{Deserialize, Serialize, Deserializer, Serializer};

use crate::{impl_binary_op_refs, impl_binary_ops_refs, impl_assign_ops_refs};
use crate::units::currency::Currency;
use crate::primitives::{F64, Percent, RawPrice};
use crate::price::visitor::{PriceCurrencyVisitor, UnitValueVisitor};
use chrono::{TimeZone, DateTime, NaiveDate};
use serde::ser::SerializeMap;

pub mod visitor;

#[derive(
Clone, Copy, Debug, Serialize, Deserialize, PartialEq,
Display, AsRef, AsMut, From, Into
)]
#[display(fmt = "{} {}", currency, raw_price)]
pub struct Price {
    raw_price: RawPrice,
    #[as_mut(ignore)]
    currency: Currency,
}

#[derive(
Clone, Debug,
Display, AsRef, From, Into
)]
#[display(fmt = "{} at {}", price, date_time)]
pub struct TimeBoundedPrice<Tz: TimeZone> {
    price: Price,
    date_time: DateTime<Tz>,
}

#[derive(
Clone, Copy, Debug, PartialEq,
Display, AsRef, From, Into
)]
#[display(fmt = "{} on {}", price, date)]
pub struct DateBoundedPrice {
    price: Price,
    date: NaiveDate,
}

impl Price {
    pub const fn new(price: f64, currency: Currency) -> Self {
        Self { raw_price: RawPrice::new(price), currency }
    }

    pub const fn from_raw(raw_price: RawPrice, currency: Currency) -> Self {
        Self { raw_price, currency }
    }

    pub const fn into_raw_price(self) -> RawPrice {
        self.raw_price
    }

    pub const fn into_currency(self) -> Currency {
        self.currency
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = currency;
        self
    }

    pub const fn raw_price(&self) -> RawPrice {
        self.raw_price
    }

    pub const fn mut_raw_price(&mut self) -> &mut RawPrice {
        &mut self.raw_price
    }

    pub const fn currency(&self) -> Currency {
        self.currency
    }

    pub const fn as_f64(&self) -> f64 {
        self.raw_price.as_f64()
    }

    pub const fn as_mut_f64(&mut self) -> &mut f64 {
        self.raw_price.as_mut_f64()
    }

    #[allow(non_snake_case)]
    pub const fn as_F64(&self) -> F64 {
        self.raw_price.as_F64()
    }

    pub fn deserialize_price_currency_map<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_map(PriceCurrencyVisitor)
    }
    pub fn deserialize_unit_value_map<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_map(UnitValueVisitor)
    }

    pub fn serialize_price_currency_map<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        self.serialize_map(serializer, "price", "currency")
    }
    pub fn serialize_unit_value_map<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        self.serialize_map(serializer, "value", "unit")
    }
    pub fn serialize_map<S>(&self, serializer: S, price_key: &str, currency_key: &str) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(price_key, &self.raw_price)?;
        map.serialize_entry(currency_key, &self.currency)?;
        map.end()
    }
}

impl<Tz: TimeZone> TimeBoundedPrice<Tz> {
    pub const fn new(price: f64, currency: Currency, date_time: DateTime<Tz>) -> Self {
        Self {
            price: Price::new(price, currency),
            date_time,
        }
    }

    pub const fn from_raw(price: RawPrice, currency: Currency, date_time: DateTime<Tz>) -> Self {
        Self {
            price: Price::from_raw(price, currency),
            date_time,
        }
    }

    pub fn into_price(self) -> Price {
        self.price
    }

    pub fn into_date_time(self) -> DateTime<Tz> {
        self.date_time
    }

    pub const fn with_price(mut self, price: Price) -> Self {
        self.price = price;
        self
    }

    pub const fn with_raw_price(mut self, raw_price: RawPrice) -> Self {
        self.price.raw_price = raw_price;
        self
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        self.price.currency = currency;
        self
    }

    pub fn with_date_time(mut self, date_time: DateTime<Tz>) -> Self {
        self.date_time = date_time;
        self
    }

    pub const fn price(&self) -> &Price {
        &self.price
    }

    pub const fn date_time(&self) -> &DateTime<Tz> {
        &self.date_time
    }
}

impl DateBoundedPrice {
    pub const fn new(price: f64, currency: Currency, date: NaiveDate) -> Self {
        Self {
            price: Price::new(price, currency),
            date,
        }
    }

    pub const fn from_raw(raw_price: RawPrice, currency: Currency, date: NaiveDate) -> Self {
        Self {
            price: Price::from_raw(raw_price, currency),
            date,
        }
    }

    pub const fn into_price(self) -> Price {
        self.price
    }

    pub const fn into_date(self) -> NaiveDate {
        self.date
    }

    pub const fn with_price(mut self, price: Price) -> Self {
        self.price = price;
        self
    }

    pub const fn with_raw_price(mut self, raw_price: RawPrice) -> Self {
        self.price.raw_price = raw_price;
        self
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        self.price.currency = currency;
        self
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = date;
        self
    }

    pub const fn price(&self) -> &Price {
        &self.price
    }

    pub const fn date(&self) -> &NaiveDate {
        &self.date
    }
}

impl<Tz> PartialEq for TimeBoundedPrice<Tz>
    where
        Tz: TimeZone,
        DateTime<Tz>: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.date_time == other.date_time
    }

    fn ne(&self, other: &Self) -> bool {
        self.price != other.price || self.date_time != other.date_time
    }
}

impl_binary_op_refs!(impl Neg for Price as neg(raw_price));

impl_binary_ops_refs!(impl BinaryOps<RawPrice> for Price as op(raw_price,));
impl_assign_ops_refs!(impl AssignOps<RawPrice> for Price as assign_op(raw_price,));

impl_binary_ops_refs!(impl BinaryOps<F64> for Price as op(raw_price,));
impl_assign_ops_refs!(impl AssignOps<F64> for Price as assign_op(raw_price,));

impl_binary_ops_refs!(impl BinaryOps<Percent> for Price as op(raw_price,));
impl_assign_ops_refs!(impl AssignOps<Percent> for Price as assign_op(raw_price,));

impl_binary_ops_refs!(impl BinaryOps<f64> for Price as op(raw_price,));
impl_assign_ops_refs!(impl AssignOps<f64> for Price as assign_op(raw_price,));
