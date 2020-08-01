#![feature(const_fn)]
#![feature(const_if_match)]

pub mod currency;
pub mod indicators;
pub mod market_values;

pub mod prelude {
    pub use crate::{
        currency::Currency,
        indicators,
        market_values::{
            percent::Percent,
            points::Points,
            price::Price,
        },
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
