use core::fmt::Display;
use core::marker::PhantomData;

use crate::types::l1::{L1, L1Unit};
use crate::types::m1::{M1, M1Unit};
use crate::{Dimension, QuantityNum, Unit};


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct L1_M1<V: QuantityNum, U: L1_M1Unit> {
    pub (crate) unit: PhantomData<U>,
    pub (crate) value: V
}

impl<V: QuantityNum + Display, U: L1_M1Unit> Display for L1_M1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L1_M1Unit> Dimension for L1_M1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

// Ops

impl_dimension_ops!(L1_M1, L1_M1Unit);

impl_dimension_div!(L1_M1, L1 => M1, L1_M1Unit, L1Unit);
impl_dimension_div!(L1_M1, M1 => L1, L1_M1Unit, M1Unit);

#[allow(non_camel_case_types)]
pub trait L1_M1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct MeterKilogram;
impl Unit for MeterKilogram {
    const ABBREVIATION: &'static str = "m·kg";
}
impl L1_M1Unit for MeterKilogram {}

impl MeterKilogram {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1<V, MeterKilogram> {
        L1_M1 {
            unit: PhantomData,
            value
        }
    }
}