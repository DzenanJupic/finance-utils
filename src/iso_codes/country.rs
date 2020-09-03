use serde::{Deserialize, Serialize};
use derive_more::Display;


macro_rules! make_country {
    (pub enum $enum_:ident { $($alpha2:literal, $alpha3:ident);* }) => {
        /// for explanation of the individual codes, see: [ISO_3166](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
        #[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
        pub enum $enum_ {
            $(
                #[serde(alias = $alpha2)]
                $alpha3,
            )*
        }

        impl $enum_ {
            pub const fn as_alpha2_code(&self) -> &'static str {
                match self {
                    $( Self::$alpha3 => $alpha2 ),*
                }
            }
            pub const fn as_alpha3_code(&self) -> &'static str {
                match self {
                    $( Self::$alpha3 => stringify!($alpha3) ),*
                }
            }
        }
    };
}

make_country!(pub enum Country {
    "DE", DEU;
    "CA", CAN;
    "JP", JPN;
    "CN", CHE;
    "US", USA
});

