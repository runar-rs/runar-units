use core::fmt::Display;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type Mass<V, U> = M1<V, U>;

#[derive(Debug, Clone, Copy)]
pub struct M1<V, U>
where V: QuantityNum, U: M1Unit {
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

/// Add with same `ValueType` and same `Unit`.
impl<V: QuantityNum + Add<Output = V>, U: M1Unit> Add for M1<V, U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            unit: PhantomData,
        }
    }
}

/// AddAssign with same `ValueType` and same `Unit`.
impl<V: QuantityNum + AddAssign, U: M1Unit> AddAssign for M1<V, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

/// Subtract with same `ValueType` and same `Unit`.
impl<V: QuantityNum + Sub<Output = V>, U: M1Unit> Sub for M1<V, U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            unit: PhantomData,
        }
    }
}

/// SubAssign with same `ValueType` and same `Unit`.
impl<V: QuantityNum + SubAssign, U: M1Unit> SubAssign for M1<V, U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

/// Mul with same `ValueType` and scalar
impl<V: QuantityNum + Mul<Output = V>, U: M1Unit> Mul<V> for M1<V, U> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        Self {
            value: self.value * rhs,
            unit: PhantomData,
        }
    }
}

/// MulAssign with same `ValueType` and scalar
impl<V: QuantityNum + MulAssign, U: M1Unit> MulAssign<V> for M1<V, U> {
    fn mul_assign(&mut self, rhs: V) {
        self.value *= rhs;
    }
}

/// Div with same `ValueType` and scalar
impl<V: QuantityNum + Div<Output = V>, U: M1Unit> Div<V> for M1<V, U> {
    type Output = Self;

    fn div(self, rhs: V) -> Self::Output {
        Self {
            value: self.value / rhs,
            unit: PhantomData,
        }
    }
}

/// DivAssign with same `ValueType` and scalar
impl<V: QuantityNum + DivAssign, U: M1Unit> DivAssign<V> for M1<V, U> {
    fn div_assign(&mut self, rhs: V) {
        self.value /= rhs;
    }
}

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