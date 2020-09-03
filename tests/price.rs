#![allow(non_snake_case)]

use core::ops::*;

use lazy_static::lazy_static;
use rand::{random, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};

use pecunia::iso_codes::units::currency::Currency;
use pecunia::price::Price;
use pecunia::primitives::*;
use pecunia::test_ops;

lazy_static! {
    static ref TEST_ROUNDS: usize = std::env::var("TEST_ROUNDS").unwrap_or("1".to_string()).parse().unwrap();
}

/// generates a random f64 with up to 15 decimals
fn random_f64() -> f64 {
    (random::<u16>() as f64)
        .mul(random::<f64>())
        .mul(10.0f64.powi(10))
        .round()
        .div(10.0f64.powi(10))
}

fn random_currency() -> Currency {
    use rand::{thread_rng};
    let mut tr = thread_rng();
    let i = tr.gen_range(0, 10);

    match i {
        0 => Currency::NotACurrency,
        1 => Currency::USD,
        2 => Currency::EUR,
        3 => Currency::AUD,
        4 => Currency::CAD,
        5 => Currency::CHF,
        6 => Currency::GBP,
        7 => Currency::JPY,
        8 => Currency::RUB,
        9 => Currency::SAR,
        _ => unreachable!()
    }
}

#[test]
fn construct_Price() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let c = random_currency();

        let n = Price::new(p, c);
        let r = Price::from_raw(RawPrice::new(p), c);
        assert_eq!(n, r);
    }
}

#[test]
fn change_currency() {
    for _ in 0..*TEST_ROUNDS {
        let c = random_currency();

        let prev = Price::new(random_f64(), c);
        assert_eq!(prev.currency(), c);
        let c = random_currency();
        let aft = prev.with_currency(c);
        assert_eq!(aft.currency(), c);
    }
}

#[test]
fn get_RawPrice() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let c = random_currency();

        let price = Price::new(p, c);
        let raw = price.raw_price();
        assert_eq!(raw.as_f64(), p);
        assert_eq!(price, Price::from_raw(raw, c));
    }
}

#[test]
fn get_mut_RawPrice() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let mut price = Price::new(p, random_currency());
        assert_eq!(price.raw_price(), RawPrice::new(p));

        let p = random_f64();
        *price.mut_raw_price() = RawPrice::new(p);
        assert_eq!(price.raw_price(), RawPrice::new(p));
    }
}

#[test]
fn currency() {
    for _ in 0..*TEST_ROUNDS {
        let c = random_currency();
        let price = Price::new(random_f64(), c);
        assert_eq!(price.currency(), c);
    }
}

#[test]
fn as_f64() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();

        let price = Price::new(p, random_currency());
        assert_eq!(price.as_f64(), p);
    }
}

#[test]
fn as_mut_f64() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let mut price = Price::new(p, random_currency());
        assert_eq!(price.as_f64(), p);

        let p = random_f64();
        *price.as_mut_f64() = p;
        assert_eq!(price.as_f64(), p);
    }
}

#[test]
fn as_F64() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let price = Price::new(p, random_currency());
        assert_eq!(price.as_F64(), F64::new(p));
    }
}

#[test]
fn deserialize_Price() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let c = random_currency();
        let sc = serde_json::to_string(&c).unwrap();

        let price_currency = Price::deserialize_price_currency_map(
            &mut Deserializer::from_str(&format!(r#"{{"price": {}, "currency": {}}}"#, p, sc))
        ).unwrap();
        let unit_value = Price::deserialize_unit_value_map(
            &mut Deserializer::from_str(&format!(r#"{{"value": {}, "unit": {}}}"#, p, sc))
        ).unwrap();
        let seq = Price::deserialize(
            &mut Deserializer::from_str(&format!(r#"[{}, {}]"#, p, sc))
        ).unwrap();

        assert_eq!(price_currency, unit_value);
        assert_eq!(price_currency, seq);
        assert_eq!(price_currency, Price::new(p, c));
    }
}

#[test]
fn serialize_Price() {
    for _ in 0..*TEST_ROUNDS {
        let p = random_f64();
        let sp = serde_json::to_string(&p).unwrap();
        let c = random_currency();
        let sc = serde_json::to_string(&c).unwrap();
        let price = Price::new(p, c);

        let mut vec = Vec::new();
        {
            let mut serializer = Serializer::new(&mut vec);
            price.serialize(&mut serializer).unwrap();
            assert_eq!(String::from_utf8_lossy(&vec), format!(r#"[{},{}]"#, sp, sc));
        }
        vec.clear();
        {
            let mut serializer = Serializer::new(&mut vec);
            price.serialize_price_currency_map(&mut serializer).unwrap();
            assert_eq!(String::from_utf8_lossy(&vec), format!(r#"{{"price":{},"currency":{}}}"#, sp, sc));
        }
        vec.clear();
        {
            let mut serializer = Serializer::new(&mut vec);
            price.serialize_unit_value_map(&mut serializer).unwrap();
            assert_eq!(String::from_utf8_lossy(&vec), format!(r#"{{"value":{},"unit":{}}}"#, sp, sc));
        }
    }
}

test_ops!(Price, (|price: f64| Price::new(price, Currency::NotACurrency)), RawPrice, RawPrice::new);
test_ops!(Price, (|price: f64| Price::new(price, Currency::NotACurrency)), F64, F64::new);
test_ops!(Price, (|price: f64| Price::new(price, Currency::NotACurrency)), f64, (|v: f64| v));
test_ops!(Price, (|price: f64| Price::new(price, Currency::NotACurrency)), Percent);
