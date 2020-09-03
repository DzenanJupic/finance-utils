#[macro_export]
macro_rules! impl_assign_op {
    // matches for any assignment operation, where there's one ore more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as op((lhs0, rhs0), (lhs1, rhs1)) )
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as $method:ident($(($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)),+)
    ) => ($crate::impl_assign_op!(
        impl$(<$($impl_generics),*>)? $trait_<$rhs> for $lhs
            $(where $($where_ty: $bound),+)?
            as $method(lhs, rhs) {
                $( lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*); )+
            }
    ););
    // matches for any assignment operation, where there's one field to mutate
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)
    ) => ($crate::impl_assign_op!(
        impl$(<$($impl_generics),*>)? $trait_<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )?
            as $method(lhs, rhs) lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*)
    ););
    // matches for any assignment operation
    // the operation code must be provided in form of an expression (or a block)
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+ )?
            as $method:ident($lhs_ident:ident, $rhs_ident:ident) $code:expr
    ) => {
        impl$(<$($impl_generics),*>)? ::core::ops::$trait_<$rhs> for $lhs
            $(where $($where_ty: $bound),+ )? {
            #[allow(unused_mut)]
            fn $method(&mut self, mut $rhs_ident: $rhs) {
                let ref mut $lhs_ident = *self;
                $code
            }
        }
    };
}

#[macro_export]
macro_rules! impl_assign_op_refs {
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as $method:ident($(($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)),+)
    ) => ($crate::impl_assign_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<$rhs> for $lhs
            $(where $($where_ty: $bound),+)?
            as $method(lhs, rhs) {
                $( lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*); )+
            }
    ););
    // matches for any assignment operation, where there's one field to mutate
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as $method:ident($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)
    ) => ($crate::impl_assign_op_refs!(
        impl$(<$($impl_generics),*>)? $trait_<$rhs> for $lhs
            $(where $( $where_ty: $bound),+)?
            as $method(lhs, rhs) lhs.$($lhs_field.)*$method(rhs$(.$rhs_field $(($($rhs_field_args),*))?).*)
    ););
    // matches for assignment any assignment operations
    // the operation code must be provided in form of an expression (or a block)
    (
        impl$(<$($impl_generics:tt),*>)? $trait_:ident<$rhs:ty> for $lhs:ty
            $(where $($where_ty:ty : $bound:path),+)?
            as $method:ident($lhs_ident:ident , $rhs_ident:ident) $code:expr
    ) => {
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? $trait_<$rhs> for $lhs
                $(where $($where_ty: $bound),+)?
                as $method($lhs_ident , $rhs_ident) $code
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? $trait_<&$rhs> for &mut $lhs
                $(where $($where_ty: $bound),+)?
                as $method($lhs_ident , $rhs_ident) $code
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? $trait_<&$rhs> for $lhs
                $(where $($where_ty: $bound),+)?
                as $method($lhs_ident , $rhs_ident) $code
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? $trait_<$rhs> for &mut $lhs
                $(where $($where_ty: $bound),+)?
                as $method($lhs_ident , $rhs_ident) $code
        );
    };
}

#[macro_export]
macro_rules! impl_assign_ops {
    // matches for any assignment operation, where there's one ore more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as assign_op((lhs0, rhs0), (lhs1, rhs1)) )
    (
        impl$(<$($impl_generics:tt),*>)? AssignOps<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as assign_op($(($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)),+)
    ) => {
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? AddAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as add_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? SubAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as sub_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? MulAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as mul_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? DivAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as div_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? RemAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as rem_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
    };
    // matches for any assignment operation, where there's one field to mutate
    (
        impl$(<$($impl_generics:tt),*>)? AssignOps<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as assign_op($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)
    ) => {
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? AddAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as add_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? SubAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as sub_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? MulAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as mul_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? DivAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as div_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op!(
            impl$(<$($impl_generics),*>)? RemAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as rem_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
    };
}

#[macro_export]
macro_rules! impl_assign_ops_refs {
    // matches for any assignment operation, where there's one ore more field[s] to mutate
    // to match multiple fields, the lhs-rhs-pairs need to be wrapped in parenthesis ( as assign_op((lhs0, rhs0), (lhs1, rhs1)) )
    (
        impl$(<$($impl_generics:tt),*>)? AssignOps<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as assign_op($(($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)),+)
    ) => {
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? AddAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as add_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? SubAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as sub_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? MulAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as mul_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? DivAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as div_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? RemAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as rem_assign($(($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)),+)
        );
    };
    // matches for any assignment operation, where there's one field to mutate
    (
        impl$(<$($impl_generics:tt),*>)? AssignOps<$rhs:ty> for $lhs:ty
            $(where $( $where_ty:ty : $bound:path),+ )?
            as assign_op($($lhs_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt).*))?).*)
    ) => {
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? AddAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as add_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? SubAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as sub_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? MulAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as mul_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? DivAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as div_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
        $crate::impl_assign_op_refs!(
            impl$(<$($impl_generics),*>)? RemAssign<$rhs> for $lhs
                $(where $( $where_ty: $bound),+ )?
                as rem_assign($($lhs_field).*, $($rhs_field $(($($rhs_field_args).*))?).*)
        );
    };
}
