#[macro_export]
macro_rules! serde_with {
    (
        Serializer for $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $serializer:ident with $def_lit:literal
        $($($metas:meta)+)?
    ) => {
        #[derive($crate::export::serde::Serialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $serializer<'serializer $($(, $remote_generics)*)?> {
            #[serde(with = $def_lit)]
            $vis remote: &'serializer $remote_type$(<$($remote_generics),*>)?
        }
        #[allow(unused)]
        $vis fn serialize<$($($($where_lifetimes,)*)?)? $($($remote_generics,)*)? S>(remote_type: &$remote_type$(<$($remote_generics),*>)?, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::export::serde::Serializer,
                $($($where_ty: $bound),+)? {
            $crate::export::serde::Serialize::serialize(&$serializer { remote: remote_type }, serializer)
        }
    };
    (
        Deserializer for $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $deserializer:ident with $def_lit:literal
        $($($metas:meta)+)?
    ) => {
        #[derive($crate::export::serde::Deserialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $deserializer<$($($remote_generics),*)?> {
            #[serde(with = $def_lit)]
            $vis remote: $remote_type$(<$($remote_generics),*>)?
        }
        #[allow(unused)]
        $vis fn deserialize<'de, $($($($where_lifetimes,)*)?)? $($($remote_generics,)*)? D>(deserializer: D) -> Result<$remote_type$(<$($remote_generics),*>)?, D::Error>
            where
                D: $crate::export::serde::Deserializer<'de>,
                $($($where_ty: $bound),+)? {
            $crate::export::serde::Deserialize::deserialize(deserializer).map(|ok: $deserializer$(<$($remote_generics),*>)?| ok.remote)
        }
    };
}

#[macro_export]
macro_rules! serde_option {
    (
        serialize $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $option_serializer:ident
        with $serializer:ident
        $($($metas:meta)+)?
    ) => {
        #[derive($crate::export::serde::Serialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $option_serializer<'serializer $($(, $remote_generics)*)?> {
            #[serde(serialize_with = "self::serialize")]
            $vis remote: &'serializer ::core::option::Option<$remote_type$(<$($remote_generics),*>)?>
        }
        #[allow(unused)]
        $vis fn serialize<$($($($where_lifetimes,)*)?)? $($($remote_generics,)*)? S>(remote_type_option: &Option<$remote_type$(<$($remote_generics),*>)?>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::export::serde::Serializer,
                $($($where_ty: $bound),+)? {
            match remote_type_option {
                Some(ref remote_type) => $crate::export::serde::Serialize::serialize(&$serializer { remote: remote_type }, serializer),
                None => $crate::export::serde::Serialize::serialize(&::core::option::Option::<$serializer$(<$($remote_generics),*>)?>::None, serializer)
            }
        }
    };
    (
        deserialize $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $option_deserializer:ident
        with $deserializer:ident
        $($($metas:meta)+)?
    ) => {
        #[derive($crate::export::serde::Deserialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $option_deserializer$(<$($remote_generics),*>)? {
            #[serde(deserialize_with = "self::deserialize")]
            $vis remote: ::core::option::Option<$remote_type$(<$($remote_generics),*>)?>
        }
        #[allow(unused)]
        $vis fn deserialize<'de, $($($($where_lifetimes,)*)?)? $($($remote_generics,)*)? D>(deserializer: D) -> Result<Option<$remote_type$(<$($remote_generics),*>)?>, D::Error>
            where
                D: $crate::export::serde::Deserializer<'de>,
                $($($where_ty: $bound),+)? {
            $crate::export::serde::Deserialize::deserialize(deserializer).map(|ok: $deserializer$(<$($remote_generics,)*>)?| Some(ok.remote))
        }
    };
}

#[macro_export]
macro_rules! serde_vec {
    (
        serialize $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $vec_serializer:ident
        with $serializer:ident
        $($($metas:meta)+)?
    ) => {
        #[derive($crate::export::serde::Serialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $vec_serializer<'serializer $($(, $remote_generics)*)?> {
            #[serde(serialize_with = "self::serialize")]
            $vis remote: &'serializer $crate::export::rust_std::vec::Vec<$remote_type$(<$($remote_generics),*>)?>
        }
        #[allow(unused)]
        $vis fn serialize<$($($where_lifetimes,)*)? $($($remote_generics,)*)? S>(remote_type_vec: &Vec<$remote_type$(<$($remote_generics),*>)?>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::export::serde::Serializer,
                $($($where_ty: $bound),+)? {
            let mut vec = $crate::export::serde::Serializer::serialize_seq(serializer, Some(remote_type_vec.len()))?;
            for remote_type in remote_type_vec {
                $crate::export::serde::ser::SerializeSeq::serialize_element(&mut vec, &$serializer { remote: remote_type })?;
            }
            $crate::export::serde::ser::SerializeSeq::end(vec)
        }
    };
    (
        deserialize $remote_type:ident$(<$($remote_generics:tt),*>)?
        $(where$(<$($where_lifetimes:lifetime),*>)? $($where_ty:path : $bound:path),+ )?
        as $vis:vis $vec_deserializer:ident
        with $deserializer:ident
        $($($metas:meta)+)?
        $(, max=$max:literal)?
    ) => {
        #[derive($crate::export::serde::Deserialize)]
        #[allow(unused)]
        #[serde(transparent)]
        $($(#[$metas])+)?
        $vis struct $vec_deserializer$(<$($remote_generics),*>)? {
            #[serde(deserialize_with = "self::deserialize")]
            $vis remote: $crate::export::rust_std::vec::Vec<$remote_type$(<$($remote_generics),*>)?>
        }
        #[allow(unused)]
        $vis fn deserialize<'de, $($($where_lifetimes,)*)? $($($remote_generics,)*)? D>(deserializer: D) -> Result<Vec<$remote_type$(<$($remote_generics,)*>)?>, D::Error>
            where
                D: $crate::export::serde::Deserializer<'de>,
                $($($where_ty: $bound),+)? {
            $crate::export::serde::Deserializer::deserialize_seq(deserializer, VecVisitor { _phantom: ::core::marker::PhantomData })
        }

        struct VecVisitor$(<$($remote_generics),*>)? {
            _phantom: ::core::marker::PhantomData<($($($remote_generics,)*)?)>
        }

        impl<'de $($(,$where_lifetimes)*)? $($(,$remote_generics)*)?> $crate::export::serde::de::Visitor<'de> for VecVisitor$(<$($remote_generics,)*>)?
            $(where $($where_ty: $bound),+)?{
            type Value = $crate::export::rust_std::vec::Vec<$remote_type$(<$($remote_generics),*>)?>;

            fn expecting(&self, formatter: &mut $crate::export::serde::export::fmt::Formatter) -> $crate::export::serde::export::fmt::Result {
                formatter.write_str(concat!("a Vec<", stringify!($remote_type<$($($remote_generics),*>)?),">"))
            }

            fn visit_seq<V: $crate::export::serde::de::SeqAccess<'de>>(self, mut visitor: V) -> Result<Self::Value, V::Error> {
                let mut values = match visitor.size_hint() {
                    Some(size) => Vec::with_capacity(size),
                    None => {
                        let first = match visitor.next_element::<$deserializer$(<$($remote_generics,)*>)?>()? {
                            Some(f) => f,
                            None => return Ok(Vec::new())
                        };
                        let mut values = Vec::with_capacity(0 $( + $max)?);
                        values.push(first.remote);
                        values
                    },
                };

                while let Some(type_deserializer) = visitor.next_element::<$deserializer$(<$($remote_generics,)*>)?>()? {
                    values.push(type_deserializer.remote)
                }

                Ok(values)
            }
        }
    };
}