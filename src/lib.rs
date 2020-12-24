#![feature(const_fn)]
#![feature(const_mut_refs)]

pub mod country;
pub mod macros;
pub mod price;
pub mod primitives;
pub mod primitive_value;
pub mod units;

pub mod prelude {
    pub use crate::{
        primitive_value::PrimitiveValue,
        primitives::{
            Percent,
            F64
        },
        price::Price,
        units::UnitLike
    };
}

#[doc(hidden)]
/// used by declarative macros in this crate
pub mod export {
    pub use core as rust_core;
    pub use std as rust_std;

    pub use serde;
    pub use derive_more;
    pub use num_traits;
    pub use num_derive;

    pub use crate::primitive_value::{
        PrimitiveValue,
        visitor::PrimitiveValueVisitor
    };
    pub use crate::primitives::F64;
}
