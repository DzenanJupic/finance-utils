use derive_more::*;
use serde::{Deserialize, Serialize, Deserializer, Serializer};

use crate::{impl_binary_op_refs, impl_binary_ops_refs, impl_assign_ops_refs};
use crate::iso_codes::units::currency::Currency;
use crate::primitives::{F64, Percent, RawPrice};
use crate::price::visitor::{RawPriceCurrencyVisitor, UnitValueVisitor};
use chrono::{TimeZone, DateTime, NaiveDate};
use serde::ser::SerializeMap;

pub mod visitor;

#[derive(
Clone, Copy, Debug, Serialize, Deserialize, PartialEq,
Display, AsRef, AsMut, From, Into
)]
#[display(fmt = "{} {}", _1, _0)]
pub struct Price(
    RawPrice,
    #[as_mut(ignore)]
    Currency,
);

#[derive(
Clone, Debug, PartialEq,
Display, AsRef, From, Into
)]
#[display(fmt = "{} at {}", _0, _1)]
pub struct TimeBoundedPrice<Tz: TimeZone>(
    Price,
    DateTime<Tz>
);

#[derive(
Clone, Copy, Debug, PartialEq,
Display, AsRef, From, Into
)]
#[display(fmt = "{} on {}", _0, _1)]
pub struct DateBoundedPrice(
    Price,
    NaiveDate
);

impl Price {
    pub const fn new(price: f64, currency: Currency) -> Self {
        Self(RawPrice::new(price), currency)
    }

    pub const fn from_raw(price: RawPrice, currency: Currency) -> Self {
        Self(price, currency)
    }

    pub const fn into_raw_price(self) -> RawPrice {
        self.0
    }

    pub const fn into_currency(self) -> Currency {
        self.1
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        self.1 = currency;
        self
    }

    pub const fn raw_price(&self) -> RawPrice {
        self.0
    }

    pub const fn mut_raw_price(&mut self) -> &mut RawPrice {
        &mut self.0
    }

    pub const fn currency(&self) -> Currency {
        self.1
    }

    pub const fn as_f64(&self) -> f64 {
        self.0.as_f64()
    }

    pub const fn as_mut_f64(&mut self) -> &mut f64 {
        self.0.as_mut_f64()
    }

    #[allow(non_snake_case)]
    pub const fn as_F64(&self) -> F64 {
        self.0.as_F64()
    }

    pub fn deserialize_price_currency_map<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_map(RawPriceCurrencyVisitor)
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
        map.serialize_entry(price_key, &self.0)?;
        map.serialize_entry(currency_key, &self.1)?;
        map.end()
    }
}

impl<Tz: TimeZone> TimeBoundedPrice<Tz> {
    pub const fn new(price: f64, currency: Currency, date_time: DateTime<Tz>) -> Self {
        Self(
            Price::new(price, currency),
            date_time
        )
    }

    pub const fn from_raw(price: RawPrice, currency: Currency, date_time: DateTime<Tz>) -> Self {
        Self(
            Price::from_raw(price, currency),
            date_time
        )
    }

    pub fn into_price(self) -> Price {
        self.0
    }

    pub fn into_date_time(self) -> DateTime<Tz> {
        self.1
    }

    pub const fn with_price(mut self, price: Price) -> Self {
        self.0 = price;
        self
    }

    pub const fn with_raw_price(mut self, price: RawPrice) -> Self {
        (self.0).0 = price;
        self
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        (self.0).1 = currency;
        self
    }

    pub fn with_date_time(mut self, date_time: DateTime<Tz>) -> Self {
        self.1 = date_time;
        self
    }

    pub const fn price(&self) -> &Price {
        &self.0
    }

    pub const fn date_time(&self) -> &DateTime<Tz> {
        &self.1
    }
}

impl DateBoundedPrice {
    pub const fn new(price: f64, currency: Currency, date: NaiveDate) -> Self {
        Self(
            Price::new(price, currency),
            date
        )
    }

    pub const fn from_raw(price: RawPrice, currency: Currency, date: NaiveDate) -> Self {
        Self(
            Price::from_raw(price, currency),
            date
        )
    }

    pub const fn into_price(self) -> Price {
        self.0
    }

    pub const fn into_date(self) -> NaiveDate {
        self.1
    }

    pub const fn with_price(mut self, price: Price) -> Self {
        self.0 = price;
        self
    }

    pub const fn with_raw_price(mut self, price: RawPrice) -> Self {
        (self.0).0 = price;
        self
    }

    pub const fn with_currency(mut self, currency: Currency) -> Self {
        (self.0).1 = currency;
        self
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.1 = date;
        self
    }

    pub const fn price(&self) -> &Price {
        &self.0
    }

    pub const fn date(&self) -> &NaiveDate {
        &self.1
    }
}

impl_binary_op_refs!(impl Neg for Price as neg(0));

impl_binary_ops_refs!(impl BinaryOps<RawPrice> for Price as op(0,));
impl_assign_ops_refs!(impl AssignOps<RawPrice> for Price as assign_op(0,));

impl_binary_ops_refs!(impl BinaryOps<F64> for Price as op(0,));
impl_assign_ops_refs!(impl AssignOps<F64> for Price as assign_op(0,));

impl_binary_ops_refs!(impl BinaryOps<Percent> for Price as op(0,));
impl_assign_ops_refs!(impl AssignOps<Percent> for Price as assign_op(0,));

impl_binary_ops_refs!(impl BinaryOps<f64> for Price as op(0,));
impl_assign_ops_refs!(impl AssignOps<f64> for Price as assign_op(0,));
