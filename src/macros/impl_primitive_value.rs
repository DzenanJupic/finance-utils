#[macro_export]
macro_rules! impl_primitive_value {
    ($vis:vis struct $new_type:ident($inner_vis:vis f64)) => {
        $crate::impl_primitive_value!(raw $vis struct $new_type($inner_vis f64));
        $crate::impl_primitive_value!(impl Value for $new_type);
        $crate::impl_primitive_value!(impl From<$new_type> for F64);
        $crate::impl_primitive_value!(impl From<F64> for $new_type);
        $crate::impl_primitive_value!(impl NumOps for $new_type);
    };
    (raw $vis:vis struct $new_type:ident($inner_vis:vis f64)) => ($crate::impl_primitive_value!(
        raw $vis struct $new_type($inner_vis f64), one=1.0
    ););
    (raw $vis:vis struct $new_type:ident($inner_vis:vis f64), one=$one:literal) => {
        #[derive(
            $crate::export::rust_core::clone::Clone, $crate::export::rust_core::marker::Copy,
            $crate::export::rust_core::fmt::Debug, $crate::export::rust_core::default::Default,
            $crate::export::rust_core::cmp::PartialEq, $crate::export::rust_core::cmp::PartialOrd,
            $crate::export::derive_more::Display, $crate::export::derive_more::AsRef,
            $crate::export::derive_more::AsMut, $crate::export::derive_more::From,
            $crate::export::derive_more::Into, $crate::export::num_derive::Num,
            $crate::export::num_derive::NumCast, $crate::export::num_derive::Float,
            $crate::export::num_derive::FromPrimitive, $crate::export::num_derive::ToPrimitive,
            $crate::export::num_derive::Zero, $crate::export::serde::Serialize,
            $crate::export::serde::Deserialize
        )]
        $vis struct $new_type(
            #[display(fmt = "{:.2}")]
            $inner_vis f64
        );

        #[allow(unused)]
        impl $new_type {
            const ZERO: Self = Self(0.0);
            const ONE: Self = Self($one);
            const MINUS_ONE: Self = Self(-$one);
            const HUNDRED: Self = Self(100.0 * $one);
            const MINUS_HUNDRED: Self = Self(-100.0 * $one);
            const MIN: Self = Self($crate::export::rust_core::f64::MIN);
            const MAX: Self = Self($crate::export::rust_core::f64::MAX);

            $vis const fn new(value: f64) -> Self { Self(value) }
            $vis const fn as_f64(&self) -> f64 { self.0 }
            $vis const fn as_mut_f64(&mut self) -> &mut f64 { &mut self.0 }
            #[allow(non_snake_case)]
            $vis const fn as_F64(&self) -> $crate::primitives::F64 { F64(self.0) }
        }

        impl $crate::export::num_traits::One for $new_type {
            fn one() -> Self { Self($one) }
        }
    };
    (impl Value for $new_type:ident) =>
        ($crate::impl_primitive_value!(impl Value for $new_type, one=1.0););
    (impl Value for $new_type:ident, one=$one:literal) => {
        impl $crate::export::PrimitiveValue for $new_type {
            const ZERO: Self = Self(0.0);
            const ONE: Self = Self($one);
            const MINUS_ONE: Self = Self(-$one);
            const HUNDRED: Self = Self(100.0 * $one);
            const MINUS_HUNDRED: Self = Self(-100.0 * $one);
            const MIN: Self = Self($crate::export::rust_core::f64::MIN);
            const MAX: Self = Self($crate::export::rust_core::f64::MAX);

            fn new(value: f64) -> Self { Self(value) }
            fn as_f64(&self) -> f64 { self.0 }
            fn as_mut_f64(&mut self) -> &mut f64 { &mut self.0 }
        }
    };
    (impl From<$new_type:ty> for F64) => {
        // it's usually not necessary to bring PrimitiveValue to scope,
        // since each PrimitiveValue implemented by this macro should
        // have a const fn as_f64() and a const fn new(value: f64)
        impl $crate::export::rust_core::convert::From<$new_type> for $crate::export::F64 {
            fn from(value: $new_type) -> $crate::export::F64 {
                $crate::export::F64::new(value.as_f64())
            }
        }
    };
    (impl From<F64> for $new_type:ty) => {
        // it's usually not necessary to bring PrimitiveValue to scope,
        // since each PrimitiveValue implemented by this macro should
        // have a const fn as_f64() and a const fn new(value: f64)
        impl $crate::export::rust_core::convert::From<$crate::export::F64> for $new_type {
            fn from(value: $crate::export::F64) -> Self {
                Self(value.as_f64())
            }
        }
    };
    (impl NumOps for $new_type:ident) => {
        $crate::impl_primitive_value!(impl NumOps<Self> for $new_type);
        $crate::impl_primitive_value!(impl NumOps<f64> for $new_type);
        $crate::impl_primitive_value!(impl NumOps<$crate::primitives::F64> for $new_type);
        $crate::impl_primitive_value!(impl NumOps<Percent> for $new_type);
    };
    (impl NumOps<Self> for $new_type:ident) => {
        $crate::impl_binary_op_refs!(impl Neg for $new_type as neg(0));
        $crate::impl_primitive_value!(impl NumOps<$new_type> for $new_type);
    };
    (impl NumOps<Percent> for $new_type:path) => {
        $crate::impl_binary_op_refs!(impl Add<$crate::primitives::Percent> for $new_type as add(n, p) $new_type(n.0 + n.0 * p.as_f64()));
        $crate::impl_binary_op_refs!(impl Sub<$crate::primitives::Percent> for $new_type as sub(n, p) $new_type(n.0 - n.0 * p.as_f64()));
        $crate::impl_binary_op_refs!(impl Mul<$crate::primitives::Percent> for $new_type as mul(0, as_f64()));
        $crate::impl_binary_op_refs!(impl Div<$crate::primitives::Percent> for $new_type as div(0, as_f64()));
        $crate::impl_binary_op_refs!(impl Rem<$crate::primitives::Percent> for $new_type as rem(0, as_f64()));

        $crate::impl_assign_op_refs!(impl AddAssign<$crate::primitives::Percent> for $new_type as add_assign(n, p) n.0 += n.0 * p.as_f64());
        $crate::impl_assign_op_refs!(impl SubAssign<$crate::primitives::Percent> for $new_type as sub_assign(n, p) n.0 -= n.0 * p.as_f64());
        $crate::impl_assign_op_refs!(impl MulAssign<$crate::primitives::Percent> for $new_type as mul_assign(0, as_f64()));
        $crate::impl_assign_op_refs!(impl DivAssign<$crate::primitives::Percent> for $new_type as div_assign(0, as_f64()));
        $crate::impl_assign_op_refs!(impl RemAssign<$crate::primitives::Percent> for $new_type as rem_assign(0, as_f64()));
    };
    (impl NumOps<f64> for $new_type:ident) => ($crate::impl_primitive_value!(
        impl NumOps<f64> for $new_type
        as op((0,))
    ););
    (impl NumOps<$rhs:ty> for $new_type:ident) => ($crate::impl_primitive_value!(
        impl NumOps<$rhs> for $new_type
            where $rhs: $crate::primitive_value::PrimitiveValue
            as op((0, as_f64()))
    ););
    (impl$(<$($impl_generics:tt),*>)? NumOps<$rhs:ty> for $new_type:ident $(where $($where_ty:ty : $bound:path),+)? as op($( ($($new_type_field:tt).*, $($rhs_field:tt $(($($rhs_field_args:tt),*))?).*) ),+)) => {
        $crate::impl_binary_ops_refs!(impl$(<$($impl_generics),*>)? BinaryOps<$rhs> for $new_type $(where $($where_ty: $bound),+)? as op($($($new_type_field).*, $($rhs_field$(($($rhs_field_args),*))?).*),+));
        $crate::impl_assign_ops_refs!(impl$(<$($impl_generics),*>)? AssignOps<$rhs> for $new_type $(where $($where_ty: $bound),+)? as assign_op($($($new_type_field).*, $($rhs_field$(($($rhs_field_args),*))?).*),+));
    };
}
