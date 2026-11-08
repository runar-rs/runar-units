use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

#[allow(non_camel_case_types)]
pub struct L1_T1<V: QuantityNum, U: L1_T1Unit> {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum, U: L1_T1Unit> Dimension for L1_T1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1_T1, L1_T1Unit);

#[allow(non_camel_case_types)]
pub trait L1_T1Unit: Unit {}

pub struct MeterSecond;
impl Unit for MeterSecond {
    const ABBREVIATION: &'static str = "ms";
}
impl L1_T1Unit for MeterSecond {}
