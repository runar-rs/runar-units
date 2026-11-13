//! Electric potential
//! 

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type ElectricPotential<V, U> = IN1_L2_M1_TN3<V, U>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct IN1_L2_M1_TN3<V, U>
where
    V: QuantityNum,
    U: IN1_L2_M1_TN3Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: IN1_L2_M1_TN3Unit> core::fmt::Display for IN1_L2_M1_TN3<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: IN1_L2_M1_TN3Unit> Dimension for IN1_L2_M1_TN3<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(IN1_L2_M1_TN3, IN1_L2_M1_TN3Unit);


#[allow(non_camel_case_types)]
pub trait IN1_L2_M1_TN3Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Volt;
impl Unit for Volt {
    const ABBREVIATION: &'static str = "V";
}
impl IN1_L2_M1_TN3Unit for Volt {}

#[derive(Debug, Clone, Copy)]
pub struct Millivolt;
impl Unit for Millivolt {
    const ABBREVIATION: &'static str = "mV";
    const SCALE_TO_BASE: f64 = 1e-3;
}
impl IN1_L2_M1_TN3Unit for Millivolt {}

impl_linear_unit_conversions!(
    IN1_L2_M1_TN3;
    Volt, Millivolt
);