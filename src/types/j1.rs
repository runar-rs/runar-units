//! Luminous Intensity

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

#[derive(Debug, Clone, Copy)]
pub struct J1<V, U>
where V: QuantityNum, U: J1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: J1Unit> core::fmt::Display for J1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: J1Unit> Dimension for J1<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(J1, J1Unit);

/// Marker trait for `J1` units.
pub trait J1Unit: Unit {}

pub struct Candela;
impl Unit for Candela {
    const ABBREVIATION: &'static str = "cd";
}
impl J1Unit for Candela {}