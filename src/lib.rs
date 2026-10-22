#![no_std]

use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub mod consts;
pub mod types;

pub trait QuantityNum: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + Sized {}

impl QuantityNum for f32 {}
impl QuantityNum for f64 {}

pub trait Dimension {
    type ValueType: QuantityNum;

    fn get_raw(&self) -> &Self::ValueType;
}

pub trait DimensionWithUnit<U: Unit>: Dimension {
    fn get(&self) -> Self::ValueType;
}

/// Marker Traits for Units
pub trait Unit {}