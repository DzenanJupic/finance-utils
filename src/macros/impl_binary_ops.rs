#[macro_export]
macro_rules! impl_binary_op {
    // matches for binary operations, where there's no Rhs and one or more field[s] to mutate
    // to match multiple fields, the fields need to be wrapped in parenthesis ( as op((field0, field1)) )
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident( ($($($lhs_field:tt).*),+) )
    ) => ($crate::impl_binary_op!(
        impl$(<$($impl_generics),*>)? $trait_<Output=Self> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method(lhs) {
                $(
                    lhs$(.$lhs_field)* = lhs$(.$lhs_field)*.$method();
                )+
                lhs
            }
    ););
    // matches for binary operations, where there's no Rhs and only one field to mutate
    // the output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*)
    ) => ($crate::impl_binary_op!(
        impl$(<$($impl_generics),*>)? $trait_<Output=Self> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method(lhs) {
                lhs$(.$lhs_field)* = lhs$(.$lhs_field)*.$method();
                lhs
            }
    ););
    // matches for binary operations, where there's a Rhs and one or more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as op((lhs0, rhs0), (lhs1, rhs1)) )
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($( ($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*) ),+)
    ) => ($crate::impl_binary_op!(
        impl$(<$($impl_generics),*>)? $trait_<Output=Self, $rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method(lhs, rhs) {
                $(
                    lhs$(.$lhs_field)* = lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*);
                )+
                lhs
            }
    ););
    // matches for binary operations, where there's a Rhs and only one field to mutate
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*)
    ) => ($crate::impl_binary_op!(
        impl$(<$($impl_generics),*>)? $trait_<Output=Self, $rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method(lhs, rhs) {
                lhs$(.$lhs_field)* = lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*);
                lhs
            }
    ););
    // matches for any binary operation, where there may be a Rhs
    // the operation code must be provided in form of an expression (or a block)
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident$(<$rhs:ty>)? for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident $(, $rhs_ident:ident)?) $code:expr
    ) => ($crate::impl_binary_op!(
        impl$(<$($impl_generics),*>)? $trait_<Output=Self $(, $rhs)?> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method($lhs_ident $(, $rhs_ident)?) $code
    ););
    // matches for any binary operation, where there may be a Rhs
    // the operation code must be provided in form of an expression (or a block)
    // the Output is free to choose
    // the Output must be provided as first trait argument (TRAIT<Output=OUTPUT>)
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<Output=$output:ty $(, $rhs:ty)?> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident $(, $rhs_ident:ident)?) $code:expr
    ) => {
        impl$(<$($impl_generics),*>)? ::core::ops::$trait_$(<$rhs>)? for $lhs
            $(where $($where_ty: $bound),+ )? {
            type Output = $output;
            #[allow(unused_mut)]
            fn $method(mut self $(, mut $rhs_ident: $rhs)?) -> Self::Output {
                let mut $lhs_ident = self;
                $code
            }
        }
    };
}

#[macro_export]
macro_rules! impl_binary_op_refs {
    // matches for binary operations, where there's no Rhs and one or more field[s] to mutate
    // to match multiple fields, the fields need to be wrapped in parenthesis ( as op((field0, field1)) )
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident( ($($($lhs_field:tt).*),+) )
    ) => ($crate::impl_binary_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<Output=$lhs> for $lhs
            where
                Self: Copy
                $($(,$where_ty: $bound)+)?
            as $method(lhs) {
                $(
                    lhs$(.$lhs_field)* = lhs$(.$lhs_field)*.$method();
                )+
                lhs
            }
    ););
    // matches for binary operations, where there's no Rhs and only one field to mutate
    // the output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*)
    ) => ($crate::impl_binary_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<Output=$lhs> for $lhs
            where
                Self: Copy
                $($(,$where_ty: $bound)+)?
            as $method(lhs) {
                let mut lhs: $lhs = lhs.clone();
                lhs$(.$lhs_field)* = lhs$(.$lhs_field)*.$method();
                lhs
            }
    ););
    // matches for binary operations, where there's a Rhs and one or more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as op((lhs0, rhs0), (lhs1, rhs1)) )
    // the Output must be Lhs, and is implicit
    // Self must be copy
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($( ($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*) ),+)
    ) => ($crate::impl_binary_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<Output=$lhs, $rhs> for $lhs
            where
                Self: Copy
                $($(,$where_ty: $bound)+)?
            as $method(lhs, rhs) {
                let mut lhs: $lhs = lhs.clone();
                $(
                    lhs$(.$lhs_field)* = lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*);
                )+
                lhs
            }
    ););
    // matches for binary operations, where there's a Rhs and only one field to mutate
    // the Output must be Lhs, and is implicit
    // Self must be copy
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*)
    ) => ($crate::impl_binary_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<Output=$lhs, $rhs> for $lhs
            where
                Self: Copy
                $($(,$where_ty: $bound)+ )?
            as $method(lhs, rhs) {
                let mut lhs: $lhs = lhs.clone();
                lhs$(.$lhs_field)* = lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*);
                lhs
            }
    ););
    // matches for any binary operation, where there may be a Rhs
    // the operation code must be provided in form of an expression (or a block)
    // the Output must be Lhs, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident$(<$rhs:ty>)? for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident $(, $rhs_ident:ident)?) $code:expr
    ) => ($crate::impl_binary_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<Output=$lhs $(, $rhs)?> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method($lhs_ident $(, $rhs_ident)?) $code
    ););
    // matches for binary operations, where there's no Rhs
    // the operation code must be provided in form of an expression (or a block)
    // the Output is free to choose
    // the Output must be provided as first trait argument (TRAIT<Output=OUTPUT>)
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<Output=$output:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident) $code:expr
    ) => {
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output> for $lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident) $code
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output> for &$lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident) $code
        );
    };
    // matches for binary operations, where there's a Rhs
    // the operation code must be provided in form of an expression (or a block)
    // the Output is free to choose
    // the Output must be provided as first trait argument (TRAIT<Output=OUTPUT>)
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<Output=$output:ty, $rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident , $rhs_ident:ident) $code:expr
    ) => {
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output, $rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident, $rhs_ident) $code
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output, &$rhs> for &$lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident, $rhs_ident) $code
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output, &$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident, $rhs_ident) $code
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? $trait_<Output=$output, $rhs> for &$lhs
                $(where $($where_ty: $bound),+ )?
                as $method($lhs_ident, $rhs_ident) $code
        );
    };
}

#[macro_export]
macro_rules! impl_binary_ops {
    // matches for binary operations, where there's a Rhs and one or more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as op((lhs0, rhs0), (lhs1, rhs1)) )
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? BinaryOps<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as op($( ($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*) ),+)
    ) => {
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Add<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as add($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Sub<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as sub($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Mul<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as mul($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Div<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as div($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Rem<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as rem($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
    };
    // matches for binary operations, where there's a Rhs and only one field to mutate
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? BinaryOps<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as op($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*)
    ) => {
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Add<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as add($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Mul<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as mul($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Div<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as div($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op!(
            impl$(<$($impl_generics),*>)? Rem<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as rem($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
    };
}

#[macro_export]
macro_rules! impl_binary_ops_refs {
    // matches for binary operations, where there's a Rhs and one or more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as op((lhs0, rhs0), (lhs1, rhs1)) )
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? BinaryOps<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as op($( ($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*) ),+)
    ) => {
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Add<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as add($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Sub<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as sub($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Mul<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as mul($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Div<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as div($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Rem<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as rem($( ($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*) ),+)
        );
    };
    // matches for binary operations, where there's a Rhs and only one field to mutate
    // the Output must be Self, and is implicit
    (
        impl$(<$($impl_generics:tt),*>)? BinaryOps<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as op($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*)
    ) => {
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Add<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as add($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Sub<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as sub($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Mul<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as mul($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Div<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as div($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
        $crate::impl_binary_op_refs!(
            impl$(<$($impl_generics),*>)? Rem<$rhs> for $lhs
                $(where $($where_ty: $bound),+ )?
                as rem($($lhs_field).*, $($rhs_field $(($($rhs_field_args),*))?).*)
        );
    };
}
