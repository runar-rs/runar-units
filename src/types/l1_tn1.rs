use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type Velocity<V, U> = L1_TN1<V, U>;

#[allow(non_camel_case_types)]
pub struct L1_TN1<V: QuantityNum, U: Unit> {
    pub (crate) unit: PhantomData<U>,
    pub (crate) value: V
}

impl<V: QuantityNum, U: Unit> Dimension for L1_TN1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1_TN1, Unit);

pub struct MeterPerSecond;
impl Unit for MeterPerSecond {
    const ABBREVIATION: &'static str = "m/s";
    const LATEX_ABBREVIATION: &'static str = "\\frac{\\text{m}}{\\text{s}}";
}

impl MeterPerSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_TN1<V, MeterPerSecond> {
        L1_TN1 {
            unit: PhantomData,
            value
        }
    }
}

#[cfg(test)]
mod tests {
}
