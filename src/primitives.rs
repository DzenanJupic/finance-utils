use crate::{impl_primitive_value, impl_binary_op_refs};

impl_primitive_value!(pub struct RawPrice(f64));
impl_primitive_value!(pub struct Points(f64));

impl_primitive_value!(raw pub struct F64(pub(crate) f64));
impl_primitive_value!(impl Value for F64);
impl_primitive_value!(impl NumOps<Self> for F64);
impl_primitive_value!(impl NumOps<f64> for F64);
impl_primitive_value!(impl NumOps<Percent> for F64);

impl_primitive_value!(raw pub struct Percent(pub(crate) f64), one=0.01);
impl_primitive_value!(impl Value for Percent, one=0.01);
impl_primitive_value!(impl From<Percent> for F64);
impl_primitive_value!(impl From<F64> for Percent);
impl_binary_op_refs!(impl Neg for Percent as neg(0));
impl_primitive_value!(impl NumOps<f64> for Percent);
impl_primitive_value!(impl NumOps<F64> for Percent);
impl_primitive_value!(impl NumOps<Percent> for Percent);
