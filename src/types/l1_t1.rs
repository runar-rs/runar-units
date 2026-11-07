use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

#[allow(non_camel_case_types)]
pub struct L1_T1<V: QuantityNum, U: Unit> {
    pub unit: PhantomData<U>,
    pub value: V,
}

impl<V: QuantityNum, U: Unit> Dimension for L1_T1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1_T1, Unit);
pub struct MeterSecond;
impl Unit for MeterSecond {
    const ABBREVIATION: &'static str = "ms";
}
