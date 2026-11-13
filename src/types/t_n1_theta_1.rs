//! Rate of temperature change

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, Unit, QuantityNum};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct TN1_Theta1<V, U>
where
    V: QuantityNum,
    U: TN1_Theta1Unit
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: TN1_Theta1Unit> Display for TN1_Theta1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: TN1_Theta1Unit> Dimension for TN1_Theta1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(TN1_Theta1, TN1_Theta1Unit);

#[allow(non_camel_case_types)]
pub trait TN1_Theta1Unit: Unit {

}

#[derive(Debug, Clone, Copy)]
pub struct KelvinPerSecond;
impl Unit for KelvinPerSecond {
    const ABBREVIATION: &'static str = "K/s";
    const LATEX_ABBREVIATION: &'static str = "Ks^{-1}";
}
impl TN1_Theta1Unit for KelvinPerSecond {}

pub struct KelvinPerMinute;
impl Unit for KelvinPerMinute {
    const ABBREVIATION: &'static str = "K/min";
    const SCALE_TO_BASE: f64 = 1.0/60.0;
}
impl TN1_Theta1Unit for KelvinPerMinute {}
impl_linear_unit_conversions!(
    TN1_Theta1;
    KelvinPerSecond,
    KelvinPerMinute,
);