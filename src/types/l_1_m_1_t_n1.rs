//! Impulse
use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};
use crate::types::*;

pub type Impulse<V, U> = L1_M1_TN1<V, U>;

#[allow(non_camel_case_types)]
pub struct L1_M1_TN1<V, U>
where
    V: QuantityNum,
    U: L1_M1_TN1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: L1_M1_TN1Unit> Display for L1_M1_TN1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L1_M1_TN1Unit> Dimension for L1_M1_TN1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1_M1_TN1, L1_M1_TN1Unit);
impl_dimension_mul_with_unit!(
    L1_M1_TN1, L1_M1_TN1Unit,
    t_n1::TN1, t_n1::TN1Unit
    => l_1_m_1_t_n2::L1_M1_TN2, l_1_m_1_t_n2::Newton);

/// Marker trait for units of `L1_M1_TN1`
#[allow(non_camel_case_types)]
pub trait L1_M1_TN1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct NewtonSecond;
impl Unit for NewtonSecond {
    const ABBREVIATION: &'static str = "N·s";
}
impl L1_M1_TN1Unit for NewtonSecond {}

impl NewtonSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN1<V, NewtonSecond> {
        L1_M1_TN1 {
            unit: PhantomData,
            value,
        }
    }
}

pub struct KilogramMeterPerSecond;
impl Unit for KilogramMeterPerSecond {
    const ABBREVIATION: &'static str = "kg·m/s";
    const LATEX_ABBREVIATION: &'static str = r"\frac{kg \cdot m}{s}";
}
impl L1_M1_TN1Unit for KilogramMeterPerSecond {}

impl KilogramMeterPerSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN1<V, KilogramMeterPerSecond> {
        L1_M1_TN1 {
            unit: PhantomData,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Impulse, KilogramMeterPerSecond, NewtonSecond};
    use crate::Dimension;

    #[test]
    fn converts_between_all_listed_units() {
        let impulse: Impulse<f64, NewtonSecond> = NewtonSecond::new(12.5);
        let converted: Impulse<f64, KilogramMeterPerSecond> = impulse.into();
        assert_eq!(converted.get(), &12.5);

        let restored: Impulse<f64, NewtonSecond> = converted.into();
        assert_eq!(restored.get(), &12.5);
    }
}

impl_linear_unit_conversions!(
    L1_M1_TN1;
    NewtonSecond,
    KilogramMeterPerSecond
);
