#[macro_export]
macro_rules! price_map_visitor {
    ($visitor:ident { raw_price: $raw_price:literal, currency: $currency:literal }) => {
        impl<'de> Visitor<'de> for $visitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Price")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error> where
                A: MapAccess<'de>, {
                let mut price = None;
                let mut currency = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        $raw_price => assign_to_none(&mut price, map.next_value()?, $raw_price)?,
                        $currency => assign_to_none(&mut currency, map.next_value()?, $currency)?,
                        f => return Err(A::Error::unknown_field(f, &[$raw_price, $currency]))
                    }
                }

                Ok(Price::from_raw(
                    price.ok_or(A::Error::missing_field($raw_price))?,
                    currency.ok_or(A::Error::missing_field($currency))?,
                ))
            }
        }
    };
}
