#![cfg_attr(not(test), no_std)]

use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub mod consts;
pub mod types;

pub trait QuantityNum: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + Sized {}

impl QuantityNum for f32 {}
impl QuantityNum for f64 {}

pub trait Dimension {
    type ValueType: QuantityNum;

    /// Returns a reference to the underlying value of the dimension.
    #[must_use]
    fn get(&self) -> &Self::ValueType;
}

/// Marker Traits for Units
pub trait Unit {
    const ABBREVIATION: &'static str;
    /// Optional LaTeX abbreviation for the unit. If not provided, the `ABBREVIATION` will be used.
    const LATEX_ABBREVIATION: &'static str = Self::ABBREVIATION;
}

#[cfg(test)]
mod test_utils {
    /// Asserts that two floating point numbers are approximately equal, within a given epsilon.
    #[inline(always)]
    pub fn assert_close(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "Expected {} to be approximately equal to {} within epsilon {}",
            a,
            b,
            epsilon
        );
    }
}