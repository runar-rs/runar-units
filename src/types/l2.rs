//! Area
use core::fmt::Display;
use core::marker::PhantomData;

use crate::{
    Dimension, QuantityNum, Unit,
    types::{
        l1::{L1, L1Unit},
        l3::L3,
    },
};

pub type Area<V, U> = L2<V, U>;

#[derive(Debug, Clone, Copy)]
pub struct L2<V, U>
where
    V: QuantityNum,
    U: L2Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: L2Unit> core::fmt::Display for L2<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L2Unit> Dimension for L2<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L2, L2Unit);

impl_dimension_mul!(L2, L2Unit, L1, L1Unit => L3);
impl_dimension_div!(L2, L2Unit, L1, L1Unit => L1);

pub trait L2Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct SquareMeter;
impl Unit for SquareMeter {
    const ABBREVIATION: &'static str = "m²";
    const LATEX_ABBREVIATION: &'static str = "m^2";
}
impl L2Unit for SquareMeter {}

impl SquareMeter {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L2<V, SquareMeter> {
        L2 {
            unit: PhantomData,
            value,
        }
    }
}
