#![feature(const_fn)]
#![feature(const_mut_refs)]

pub mod iso_codes;
pub mod macros;
pub mod primitive_value;
pub mod primitives;
pub mod price;

pub mod prelude {
    pub use crate::{
        primitive_value::PrimitiveValue,
        primitives::{
            Percent,
            F64
        },
        price::Price
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

    pub use crate::primitive_value::PrimitiveValue;
    pub use crate::primitives::F64;
}
