//! Luminous flux

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};
use crate::types::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct J1_Sr1<V, U>
where
    V: QuantityNum,
    U: J1_Sr1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: J1_Sr1Unit> Display for J1_Sr1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: J1_Sr1Unit> Dimension for J1_Sr1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(J1_Sr1, J1_Sr1Unit);
impl_dimension_div_with_unit!(
    J1_Sr1, J1_Sr1Unit,
    sr_1::Sr1, sr_1::Sr1Unit
    => j_1::J1, j_1::Candela);

#[allow(non_camel_case_types)]
pub trait J1_Sr1Unit: Unit {}

pub struct Lumen;
impl Unit for Lumen {
    const ABBREVIATION: &'static str = "lm";
}
impl J1_Sr1Unit for Lumen {}

impl Lumen {
    pub fn new<V: QuantityNum>(value: V) -> J1_Sr1<V, Lumen> {
        J1_Sr1 {
            unit: PhantomData,
            value,
        }
    }
}