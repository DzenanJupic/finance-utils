use core::convert::*;
use std::ops::*;

use serde::export::fmt::Display;

use percent::Percent;

macro_rules! impl_market_value {
    (impl $struct_name:path, one=$one:literal) => {
        impl $struct_name {
                pub const fn new(value: f64) -> Self { Self(value) }
                pub const fn zero() -> Self { Self(0.0) }
                pub const fn one() -> Self { Self($one) }
                pub const fn minus_one() -> Self { Self(-$one) }
                pub const fn value(&self) -> f64 { self.0 }
                pub const fn max(&self, other: Self) -> Self { if self.0 > other.0 { *self } else { other } }
                pub const fn min(&self, other: Self) -> Self { if self.0 < other.0 { *self } else { other } }
        }
    };
    ($struct_name:path) => {
        impl_market_value!($struct_name, percent);
        // binary operators         lhs: $struct_name    rhs: $struct_name
        auto_ops::impl_op_ex!(+ |a: &$struct_name, b: &$struct_name| -> $struct_name { $struct_name(a.0 + b.0) });
        auto_ops::impl_op_ex!(- |a: &$struct_name, b: &$struct_name| -> $struct_name { $struct_name(a.0 - b.0) });
        auto_ops::impl_op_ex!(* |a: &$struct_name, b: &$struct_name| -> $struct_name { $struct_name(a.0 * b.0) });
        auto_ops::impl_op_ex!(/ |a: &$struct_name, b: &$struct_name| -> $struct_name { $struct_name(a.0 / b.0) });
        auto_ops::impl_op_ex!(% |a: &$struct_name, b: &$struct_name| -> $struct_name { $struct_name(a.0 % b.0) });
        // assignment operators     lhs: $struct_name    rhs: $struct_name
        auto_ops::impl_op_ex!(+= |a: &mut $struct_name, b: &$struct_name| { a.0 += b.0 });
        auto_ops::impl_op_ex!(-= |a: &mut $struct_name, b: &$struct_name| { a.0 -= b.0 });
        auto_ops::impl_op_ex!(*= |a: &mut $struct_name, b: &$struct_name| { a.0 *= b.0 });
        auto_ops::impl_op_ex!(/= |a: &mut $struct_name, b: &$struct_name| { a.0 /= b.0 });
        auto_ops::impl_op_ex!(%= |a: &mut $struct_name, b: &$struct_name| { a.0 %= b.0 });
    };
    ($struct_name:path, percent) => {
        impl crate::market_values::MarketValue for $struct_name {
            fn value(&self) -> f64 { self.0 }
        }
        impl ::core::convert::AsRef<f64> for $struct_name {
            fn as_ref(&self) -> &f64 { &self.0 }
        }
        impl ::core::convert::AsMut<f64> for $struct_name {
            fn as_mut(&mut self) -> &mut f64 { &mut self.0 }
        }
        impl ::std::ops::Deref for $struct_name {
            type Target = f64;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        impl ::core::convert::From<f64> for $struct_name {
            fn from(float: f64) -> Self { $struct_name(float) }
        }
        impl ::core::convert::From<&f64> for $struct_name {
            fn from(float: &f64) -> Self { $struct_name(*float) }
        }
        impl ::core::convert::Into<f64> for $struct_name {
            fn into(self) -> f64 { self.0 }
        }
        // binary operators         lhs: $struct_name    rhs: Percent
        auto_ops::impl_op_ex!(+ |a: &$struct_name, b: &crate::market_values::percent::Percent| -> $struct_name { $struct_name(a.0 + a.0 * b.0) });
        auto_ops::impl_op_ex!(- |a: &$struct_name, b: &crate::market_values::percent::Percent| -> $struct_name { $struct_name(a.0 - a.0 * b.0) });
        auto_ops::impl_op_ex!(* |a: &$struct_name, b: &crate::market_values::percent::Percent| -> $struct_name { $struct_name(a.0 * b.0) });
        auto_ops::impl_op_ex!(/ |a: &$struct_name, b: &crate::market_values::percent::Percent| -> $struct_name { $struct_name(a.0 / b.0) });
        auto_ops::impl_op_ex!(% |a: &$struct_name, b: &crate::market_values::percent::Percent| -> $struct_name { $struct_name(a.0 % b.0) });
        // assignment operators     lhs: $struct_name    rhs: crate::percent::Percent
        auto_ops::impl_op_ex!(+= |a: &mut $struct_name, b: &crate::market_values::percent::Percent| { a.0 += a.0 * b.0 });
        auto_ops::impl_op_ex!(-= |a: &mut $struct_name, b: &crate::market_values::percent::Percent| { a.0 -= a.0 * b.0 });
        auto_ops::impl_op_ex!(*= |a: &mut $struct_name, b: &crate::market_values::percent::Percent| { a.0 *= b.0 });
        auto_ops::impl_op_ex!(/= |a: &mut $struct_name, b: &crate::market_values::percent::Percent| { a.0 /= b.0 });
        auto_ops::impl_op_ex!(%= |a: &mut $struct_name, b: &crate::market_values::percent::Percent| { a.0 %= b.0 });
        // binary operators         lhs: $struct_name    rhs: f64
        auto_ops::impl_op_ex!(+ |a: &$struct_name, b: &f64| -> $struct_name { $struct_name(a.0 + b) });
        auto_ops::impl_op_ex!(- |a: &$struct_name, b: &f64| -> $struct_name { $struct_name(a.0 - b) });
        auto_ops::impl_op_ex!(* |a: &$struct_name, b: &f64| -> $struct_name { $struct_name(a.0 * b) });
        auto_ops::impl_op_ex!(/ |a: &$struct_name, b: &f64| -> $struct_name { $struct_name(a.0 / b) });
        auto_ops::impl_op_ex!(% |a: &$struct_name, b: &f64| -> $struct_name { $struct_name(a.0 % b) });
        // assignment operators     lhs: $struct_name    rhs: f64
        auto_ops::impl_op_ex!(+= |a: &mut $struct_name, b: &f64| { a.0 += b });
        auto_ops::impl_op_ex!(-= |a: &mut $struct_name, b: &f64| { a.0 -= b });
        auto_ops::impl_op_ex!(*= |a: &mut $struct_name, b: &f64| { a.0 *= b });
        auto_ops::impl_op_ex!(/= |a: &mut $struct_name, b: &f64| { a.0 /= b });
        auto_ops::impl_op_ex!(%= |a: &mut $struct_name, b: &f64| { a.0 %= b });
        // unary operators
        auto_ops::impl_op!(- |a: $struct_name| -> $struct_name { $struct_name(-a.0) });
        auto_ops::impl_op!(- |a: &$struct_name| -> $struct_name { $struct_name(-a.0) });
    };
}

pub mod percent;
pub mod points;
pub mod price;

pub trait MarketValue:
Add<Self, Output=Self> + AddAssign<Self> + Sub<Self, Output=Self> + SubAssign<Self> + Mul<Self, Output=Self> + MulAssign<Self> + Div<Self, Output=Self> + DivAssign<Self> +
Add<f64, Output=Self> + AddAssign<f64> + Sub<f64, Output=Self> + SubAssign<f64> + Mul<f64, Output=Self> + MulAssign<f64> + Div<f64, Output=Self> + DivAssign<f64> +
Add<Percent, Output=Self> + AddAssign<Percent> + Sub<Percent, Output=Self> + SubAssign<Percent> + Mul<Percent, Output=Self> + MulAssign<Percent> + Div<Percent, Output=Self> + DivAssign<Percent> +
AsRef<f64> + AsMut<f64> + Deref<Target=f64> + DerefMut<Target=f64> + From<f64> + Into<f64> +
Copy + Display + PartialEq + PartialOrd {
    fn value(&self) -> f64;
}
