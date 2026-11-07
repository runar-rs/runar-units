//! Mass
use core::fmt::Display;
use core::marker::PhantomData;

use crate::types::l1::{L1, L1Unit};
use crate::types::l1_m1::L1_M1;
use crate::{Dimension, QuantityNum, Unit};

pub type Mass<V, U> = M1<V, U>;

#[derive(Debug, Clone, Copy)]
pub struct M1<V, U>
where
    V: QuantityNum,
    U: M1Unit,
{
    pub(crate) value: V,
    pub(crate) unit: PhantomData<U>,
}

impl<V: QuantityNum + Display, U: M1Unit> Display for M1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: M1Unit> Dimension for M1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

// Ops

impl_dimension_ops!(M1, M1Unit);

impl_dimension_mul!(M1, M1Unit, L1,  L1Unit => L1_M1);

/// Marker traits for units of mass M1.
pub trait M1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Kilogram;
impl Unit for Kilogram {
    const ABBREVIATION: &'static str = "kg";
}
impl M1Unit for Kilogram {}

impl Kilogram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Kilogram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Gram;
impl Unit for Gram {
    const ABBREVIATION: &'static str = "g";
    const SCALE_TO_BASE: f64 = 1e-3;
}
impl M1Unit for Gram {}

impl Gram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Gram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Milligram;
impl Unit for Milligram {
    const ABBREVIATION: &'static str = "mg";
    const SCALE_TO_BASE: f64 = 1e-6;
}
impl M1Unit for Milligram {}

impl Milligram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Milligram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Microgram;
impl Unit for Microgram {
    const ABBREVIATION: &'static str = "µg";
    const SCALE_TO_BASE: f64 = 1e-9;
}
impl M1Unit for Microgram {}

impl Microgram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Microgram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nanogram;
impl Unit for Nanogram {
    const ABBREVIATION: &'static str = "ng";
    const SCALE_TO_BASE: f64 = 1e-12;
}
impl M1Unit for Nanogram {}

impl Nanogram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Nanogram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Picogram;
impl Unit for Picogram {
    const ABBREVIATION: &'static str = "pg";
    const SCALE_TO_BASE: f64 = 1e-15;
}
impl M1Unit for Picogram {}

impl Picogram {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> M1<V, Picogram> {
        M1 {
            value,
            unit: PhantomData,
        }
    }
}

impl_linear_unit_conversions!(
    M1;
    Kilogram, Gram, Milligram, Microgram, Nanogram, Picogram
);
