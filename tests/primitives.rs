#![allow(non_snake_case)]

use core::ops::*;

use concat_idents::concat_idents;
use lazy_static::lazy_static;
use num_traits::{One, Zero};
use serde_json::{Deserializer, Serializer};

use pecunia::primitive_value::PrimitiveValue;
use pecunia::primitives::*;
use pecunia::test_ops;

lazy_static! {
    static ref TEST_ROUNDS: usize = std::env::var("TEST_ROUNDS").unwrap_or("1".to_string()).parse().unwrap();
}

macro_rules! test_primitive {
    ($pascal_case:ident, $snake_case:ident) => {
        test_primitive!($pascal_case, $snake_case, one=1.0);
        test_ops!($pascal_case, $pascal_case::new, $pascal_case, $pascal_case::new);
    };
    ($pascal_case:ident, $snake_case:ident, one=$one:literal) => {
        test_ops!($pascal_case, $pascal_case::new);
        test_ops!($pascal_case, $pascal_case::new, Percent);
        test_ops!($pascal_case, $pascal_case::new, F64, F64::new);
        test_ops!($pascal_case, $pascal_case::new, f64, (|v: f64| v));

        concat_idents!(test_constants = $pascal_case, _, constants {
            #[test]
            fn test_constants() {for _ in 0..*TEST_ROUNDS {
                let zero = $pascal_case::ZERO;
                let zero_Value = <$pascal_case as PrimitiveValue>::ZERO;
                let zero_trait = $pascal_case::zero();
                assert_eq!(zero, zero_Value);
                assert_eq!(zero, zero_trait);
                assert_eq!(zero.as_f64(), 0.0);

                let one = $pascal_case::ONE;
                let one_Value = <$pascal_case as PrimitiveValue>::ONE;
                let one_trait = $pascal_case::one();
                assert_eq!(one, one_Value);
                assert_eq!(one, one_trait);
                assert_eq!(one.as_f64(), $one);

                let minus_one = $pascal_case::MINUS_ONE;
                let minus_one_Value = <$pascal_case as PrimitiveValue>::MINUS_ONE;
                assert_eq!(minus_one, minus_one_Value);
                assert_eq!(minus_one.as_f64(), -$one);

                let hundred = $pascal_case::HUNDRED;
                let hundred_Value = <$pascal_case as PrimitiveValue>::HUNDRED;
                assert_eq!(hundred, hundred_Value);
                assert_eq!(hundred.as_f64(), $one * 100.0);

                let minus_hundred = $pascal_case::MINUS_HUNDRED;
                let minus_hundred_Value = <$pascal_case as PrimitiveValue>::MINUS_HUNDRED;
                assert_eq!(minus_hundred, minus_hundred_Value);
                assert_eq!(minus_hundred.as_f64(), -$one * 100.0);

                let min = $pascal_case::MIN;
                let min_Value = <$pascal_case as PrimitiveValue>::MIN;
                assert_eq!(min, min_Value);
                assert_eq!(min.as_f64(), f64::MIN);

                let max = $pascal_case::MAX;
                let max_Value = <$pascal_case as PrimitiveValue>::MAX;
                assert_eq!(max, max_Value);
                assert_eq!(max.as_f64(), f64::MAX);
            }}
        });
        concat_idents!(test_construction = construct, _, $pascal_case {
            #[test]
            fn test_construction() {for _ in 0..*TEST_ROUNDS {
                let n = $pascal_case::new(10.0);
                assert_eq!(n.as_f64(), 10.0);
            }}
        });
        concat_idents!(test_get_value = get, _, $pascal_case, _, value {
            #[test]
            fn test_get_value() {for _ in 0..*TEST_ROUNDS {
                let $snake_case = $pascal_case::new(10.0);

                let as_f64 = $snake_case.as_f64();
                let as_F64 = $snake_case.as_F64().as_f64();
                let as_ref: f64 = *$snake_case.as_ref();

                let as_f64_Value = PrimitiveValue::as_f64(&$snake_case);
                let as_F64_Value = PrimitiveValue::as_F64(&$snake_case).as_f64();

                assert_eq!(as_f64, as_F64);
                assert_eq!(as_f64, as_ref);
                assert_eq!(as_ref, as_F64);

                assert_eq!(as_f64, as_f64_Value);
                assert_eq!(as_F64, as_F64_Value);
            }}
        });
        concat_idents!(test_mutate_value = mutate, _, $pascal_case, _, value {
            #[test]
            fn test_mutate_value() {for _ in 0..*TEST_ROUNDS {
                let mut $snake_case = $pascal_case::new(10.0);
                assert_eq!($snake_case.as_f64(), 10.0);

                *$snake_case.as_mut_f64() = 42.0;
                assert_eq!($snake_case.as_f64(), 42.0);

                *AsMut::<f64>::as_mut(&mut $snake_case) = 10.0;
                assert_eq!($snake_case.as_f64(), 10.0);
            }}
        });
        concat_idents!(test_deserialization = deserialize, _, $pascal_case {
            #[test]
            fn test_deserialization() {for _ in 0..*TEST_ROUNDS {
                let f64_str = $pascal_case::deserialize_str(&mut Deserializer::from_str(r#""10.0""#)).unwrap();
                let f64_str_any = $pascal_case::deserialize_any(&mut Deserializer::from_str(r#""10.0""#)).unwrap();
                let neg_f64_str = $pascal_case::deserialize_str(&mut Deserializer::from_str(r#""-10.0""#)).unwrap();
                let neg_f64_str_any = $pascal_case::deserialize_any(&mut Deserializer::from_str(r#""-10.0""#)).unwrap();
                let i64_str = $pascal_case::deserialize_str(&mut Deserializer::from_str(r#""-10""#)).unwrap();
                let i64_str_any = $pascal_case::deserialize_any(&mut Deserializer::from_str(r#""-10""#)).unwrap();
                let u64_str = $pascal_case::deserialize_str(&mut Deserializer::from_str(r#""10""#)).unwrap();
                let u64_str_any = $pascal_case::deserialize_any(&mut Deserializer::from_str(r#""10""#)).unwrap();

                let f64 = $pascal_case::deserialize_f64(&mut Deserializer::from_str("10.0")).unwrap();
                let f64_any = $pascal_case::deserialize_any(&mut Deserializer::from_str("10.0")).unwrap();
                let neg_f64 = $pascal_case::deserialize_f64(&mut Deserializer::from_str("-10.0")).unwrap();
                let neg_f64_any = $pascal_case::deserialize_any(&mut Deserializer::from_str("-10.0")).unwrap();
                let i64 = $pascal_case::deserialize_i64(&mut Deserializer::from_str("-10")).unwrap();
                let i64_any = $pascal_case::deserialize_any(&mut Deserializer::from_str("-10")).unwrap();
                let u64 = $pascal_case::deserialize_u64(&mut Deserializer::from_str("10")).unwrap();
                let u64_any = $pascal_case::deserialize_any(&mut Deserializer::from_str("10")).unwrap();

                assert_eq!(f64_str, $pascal_case::new(10.0));
                assert_eq!(f64_str_any, $pascal_case::new(10.0));
                assert_eq!(neg_f64_str, $pascal_case::new(-10.0));
                assert_eq!(neg_f64_str_any, $pascal_case::new(-10.0));
                assert_eq!(i64_str, $pascal_case::new(-10.0));
                assert_eq!(i64_str_any, $pascal_case::new(-10.0));
                assert_eq!(u64_str, $pascal_case::new(10.0));
                assert_eq!(u64_str_any, $pascal_case::new(10.0));

                assert_eq!(f64, $pascal_case::new(10.0));
                assert_eq!(f64_any, $pascal_case::new(10.0));
                assert_eq!(neg_f64, $pascal_case::new(-10.0));
                assert_eq!(neg_f64_any, $pascal_case::new(-10.0));
                assert_eq!(i64, $pascal_case::new(-10.0));
                assert_eq!(i64_any, $pascal_case::new(-10.0));
                assert_eq!(u64, $pascal_case::new(10.0));
                assert_eq!(u64_any, $pascal_case::new(10.0));
            }}
        });
        concat_idents!(test_serialization = serialize, _, $pascal_case {
            #[test]
            fn test_serialization() {for _ in 0..*TEST_ROUNDS {
                let $snake_case = $pascal_case::new(10.1);
                let neg_value = $pascal_case::new(-10.1);

                let mut vec = Vec::<u8>::new();

                {
                    let mut serializer = Serializer::new(&mut vec);
                    $snake_case.serialize_f64_str(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), r#""10.1""#);
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    neg_value.serialize_f64_str(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), r#""-10.1""#);
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    neg_value.serialize_i64_str(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), r#""-10""#);
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    $snake_case.serialize_u64_str(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), r#""10""#);
                }
                vec.clear();

                {
                    let mut serializer = Serializer::new(&mut vec);
                    $snake_case.serialize_f64(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), "10.1");
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    neg_value.serialize_f64(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), "-10.1");
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    neg_value.serialize_i64(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), "-10");
                }
                vec.clear();
                {
                    let mut serializer = Serializer::new(&mut vec);
                    $snake_case.serialize_u64(&mut serializer).unwrap();
                    assert_eq!(String::from_utf8_lossy(&vec), "10");
                }
            }}
        });
    };
}


test_primitive!(RawPrice, raw_price);
test_primitive!(Points, points);
test_primitive!(F64, f64, one=1.0);
test_primitive!(Percent, percent, one=0.01);
