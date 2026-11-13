//! Amount of Substance

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type AmountOfSubstance<V, U> = N1<V, U>;

#[derive(Debug, Clone, Copy)]
pub struct N1<V, U>
where
    V: QuantityNum,
    U: N1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: N1Unit> core::fmt::Display for N1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: N1Unit> Dimension for N1<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(N1, N1Unit);

/// Marker trait for `N1` units.
pub trait N1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Mole;
impl Unit for Mole {
    const ABBREVIATION: &'static str = "mol";
}
impl N1Unit for Mole {}

impl Mole {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> N1<V, Mole> {
        N1 {
            unit: PhantomData,
            value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Millimole;
impl Unit for Millimole {
    const ABBREVIATION: &'static str = "mmol";
    const SCALE_TO_BASE: f64 = 1e-3;
}
impl N1Unit for Millimole {}

impl Millimole {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> N1<V, Millimole> {
        N1 {
            unit: PhantomData,
            value,
        }
    }
}

impl_linear_unit_conversions!(N1; Mole, Millimole);
