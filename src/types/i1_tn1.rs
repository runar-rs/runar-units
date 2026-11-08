//! Electric Charge

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type ElectricCharge<V, U> = I1_TN1<V, U>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct I1_TN1<V, U>
where
    V: QuantityNum,
    U: I1_TN1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: I1_TN1Unit> core::fmt::Display for I1_TN1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: I1_TN1Unit> Dimension for I1_TN1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(I1_TN1, I1_TN1Unit);

#[allow(non_camel_case_types)]
pub trait I1_TN1Unit: Unit{

}

#[derive(Debug, Clone, Copy)]
pub struct Coulomb;
impl Unit for Coulomb {
    const ABBREVIATION: &'static str = "C";
}
impl I1_TN1Unit for Coulomb {}

impl Coulomb {
    pub fn new<V: QuantityNum>(value: V) -> I1_TN1<V, Coulomb> {
        I1_TN1 {
            unit: PhantomData,
            value,
        }
    }
}