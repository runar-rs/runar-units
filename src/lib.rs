#![cfg_attr(not(test), no_std)]

use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub mod consts;
pub mod types;

pub trait QuantityNum: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + Sized {}

impl QuantityNum for f32 {}
impl QuantityNum for f64 {}

pub trait Dimension {
    type ValueType: QuantityNum;

    #[must_use]
    fn get_raw(&self) -> &Self::ValueType;
}

pub trait DimensionWithUnit<U: Unit>: Dimension {
    #[must_use]
    fn get(&self) -> Self::ValueType;
}

/// Marker Traits for Units
pub trait Unit {
    const ABBREVIATION: &'static str;
    /// Optional LaTeX abbreviation for the unit. If not provided, the `ABBREVIATION` will be used.
    const LATEX_ABBREVIATION: &'static str = Self::ABBREVIATION;
}