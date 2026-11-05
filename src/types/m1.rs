use core::fmt::Display;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use core::marker::PhantomData;

use crate::types::l1::{L1, Meter};
use crate::types::l1_m1::{L1_M1, MeterKilogram};
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

/// Mul with same `ValueType` and same Kilogram and Meter.
impl<V: QuantityNum + Mul<Output = V>> Mul<L1<V, Meter>> for M1<V, Kilogram> {
    type Output = L1_M1<V, MeterKilogram>;

    fn mul(self, rhs: L1<V, Meter>) -> Self::Output {
        L1_M1 {
            value: self.value * rhs.value,
            unit: PhantomData,
        }
    }
}

/// Marker traits for units of mass M1.
pub trait M1Unit: Unit {}

/// Unit for Kilogram (kg)
/// 
/// # Implemented Conversions
/// - [`M1<f32, Kilogram>`] to [`M1<f32, Gram>`]
/// - [`M1<f32, Kilogram>`] to [`M1<f32, Milligram>`]
/// - [`M1<f32, Kilogram>`] to [`M1<f32, Microgram>`]
/// - [`M1<f64, Kilogram>`] to [`M1<f64, Gram>`]
/// - [`M1<f64, Kilogram>`] to [`M1<f64, Milligram>`]
/// - [`M1<f64, Kilogram>`] to [`M1<f64, Microgram>`]
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

/// Unit for Gram (g)
/// 
/// # Implemented Conversions
/// - [`M1<f32, Gram>`] to [`M1<f32, Kilogram>`]
/// - [`M1<f32, Gram>`] to [`M1<f32, Milligram>`]
/// - [`M1<f32, Gram>`] to [`M1<f32, Microgram>`]
/// - [`M1<f32, Gram>`] to [`M1<f32, Nanogram>`]
/// - [`M1<f32, Gram>`] to [`M1<f32, Picogram>`]
/// - [`M1<f64, Gram>`] to [`M1<f64, Kilogram>`]
/// - [`M1<f64, Gram>`] to [`M1<f64, Milligram>`]
/// - [`M1<f64, Gram>`] to [`M1<f64, Microgram>`]
/// - [`M1<f64, Gram>`] to [`M1<f64, Nanogram>`]
/// - [`M1<f64, Gram>`] to [`M1<f64, Picogram>`]
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

/// Unit for Milligram (mg)
/// 
/// # Implemented Conversions
/// - [`M1<f32, Milligram>`] to [`M1<f32, Kilogram>`]
/// - [`M1<f32, Milligram>`] to [`M1<f32, Gram>`]
/// - [`M1<f32, Milligram>`] to [`M1<f32, Microgram>`]
/// - [`M1<f32, Milligram>`] to [`M1<f32, Nanogram>`]
/// - [`M1<f32, Milligram>`] to [`M1<f32, Picogram>`]
/// - [`M1<f64, Milligram>`] to [`M1<f64, Kilogram>`]
/// - [`M1<f64, Milligram>`] to [`M1<f64, Gram>`]
/// - [`M1<f64, Milligram>`] to [`M1<f64, Microgram>`]
/// - [`M1<f64, Milligram>`] to [`M1<f64, Nanogram>`]
/// - [`M1<f64, Milligram>`] to [`M1<f64, Picogram>`]
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

#[derive(Debug, Clone, Copy)]
pub struct Microgram;
impl Unit for Microgram {
    const ABBREVIATION: &'static str = "µg";
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

/// Unit for Nanogram (ng)
/// 
/// # Implemented Conversions
/// - [`M1<f32, Nanogram>`] to [`M1<f32, Kilogram>`]
/// - [`M1<f32, Nanogram>`] to [`M1<f32, Gram>`]
/// - [`M1<f32, Nanogram>`] to [`M1<f32, Milligram>`]
/// - [`M1<f32, Nanogram>`] to [`M1<f32, Microgram>`]
/// - [`M1<f32, Nanogram>`] to [`M1<f32, Picogram>`]
/// - [`M1<f64, Nanogram>`] to [`M1<f64, Kilogram>`]
/// - [`M1<f64, Nanogram>`] to [`M1<f64, Gram>`]
/// - [`M1<f64, Nanogram>`] to [`M1<f64, Milligram>`]
/// - [`M1<f64, Nanogram>`] to [`M1<f64, Microgram>`]
/// - [`M1<f64, Nanogram>`] to [`M1<f64, Picogram>`]
#[derive(Debug, Clone, Copy)]
pub struct Nanogram;
impl Unit for Nanogram {
    const ABBREVIATION: &'static str = "ng";
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

impl From<M1<f32, Kilogram>> for M1<f32, Gram> {
    fn from(val: M1<f32, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Kilogram>> for M1<f32, Milligram> {
    fn from(val: M1<f32, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Kilogram>> for M1<f32, Microgram> {
    fn from(val: M1<f32, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Kilogram>> for M1<f64, Gram> {
    fn from(val: M1<f64, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Kilogram>> for M1<f64, Milligram> {
    fn from(val: M1<f64, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Kilogram>> for M1<f64, Microgram> {
    fn from(val: M1<f64, Kilogram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Gram>> for M1<f32, Kilogram> {
    fn from(val: M1<f32, Gram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Gram>> for M1<f32, Milligram> {
    fn from(val: M1<f32, Gram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Gram>> for M1<f32, Microgram> {
    fn from(val: M1<f32, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Gram>> for M1<f32, Nanogram> {
    fn from(val: M1<f32, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Gram>> for M1<f32, Picogram> {
    fn from(val: M1<f32, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Gram>> for M1<f64, Kilogram> {
    fn from(val: M1<f64, Gram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Gram>> for M1<f64, Milligram> {
    fn from(val: M1<f64, Gram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Gram>> for M1<f64, Microgram> {
    fn from(val: M1<f64, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Gram>> for M1<f64, Nanogram> {
    fn from(val: M1<f64, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Gram>> for M1<f64, Picogram> {
    fn from(val: M1<f64, Gram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Milligram>> for M1<f32, Kilogram> {
    fn from(val: M1<f32, Milligram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Milligram>> for M1<f32, Gram> {
    fn from(val: M1<f32, Milligram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Milligram>> for M1<f32, Microgram> {
    fn from(val: M1<f32, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Milligram>> for M1<f32, Nanogram> {
    fn from(val: M1<f32, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Milligram>> for M1<f32, Picogram> {
    fn from(val: M1<f32, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Milligram>> for M1<f64, Kilogram> {
    fn from(val: M1<f64, Milligram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Milligram>> for M1<f64, Gram> {
    fn from(val: M1<f64, Milligram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Milligram>> for M1<f64, Microgram> {
    fn from(val: M1<f64, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Milligram>> for M1<f64, Nanogram> {
    fn from(val: M1<f64, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Milligram>> for M1<f64, Picogram> {
    fn from(val: M1<f64, Milligram>) -> Self {
        M1 {
            value: val.value * 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Microgram>> for M1<f32, Kilogram> {
    fn from(val: M1<f32, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Microgram>> for M1<f32, Gram> {
    fn from(val: M1<f32, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Microgram>> for M1<f32, Milligram> {
    fn from(val: M1<f32, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Microgram>> for M1<f32, Nanogram> {
    fn from(val: M1<f32, Microgram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Microgram>> for M1<f32, Picogram> {
    fn from(val: M1<f32, Microgram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Microgram>> for M1<f64, Kilogram> {
    fn from(val: M1<f64, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Microgram>> for M1<f64, Gram> {
    fn from(val: M1<f64, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Microgram>> for M1<f64, Milligram> {
    fn from(val: M1<f64, Microgram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Microgram>> for M1<f64, Nanogram> {
    fn from(val: M1<f64, Microgram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Microgram>> for M1<f64, Picogram> {
    fn from(val: M1<f64, Microgram>) -> Self {
        M1 {
            value: val.value * 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Nanogram>> for M1<f32, Kilogram> {
    fn from(val: M1<f32, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Nanogram>> for M1<f32, Gram> {
    fn from(val: M1<f32, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Nanogram>> for M1<f32, Milligram> {
    fn from(val: M1<f32, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Nanogram>> for M1<f32, Microgram> {
    fn from(val: M1<f32, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f32, Nanogram>> for M1<f32, Picogram> {
    fn from(val: M1<f32, Nanogram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Nanogram>> for M1<f64, Kilogram> {
    fn from(val: M1<f64, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Nanogram>> for M1<f64, Gram> {
    fn from(val: M1<f64, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Nanogram>> for M1<f64, Milligram> {
    fn from(val: M1<f64, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Nanogram>> for M1<f64, Microgram> {
    fn from(val: M1<f64, Nanogram>) -> Self {
        M1 {
            value: val.value / 1_000.0,
            unit: PhantomData,
        }
    }
}

impl From<M1<f64, Nanogram>> for M1<f64, Picogram> {
    fn from(val: M1<f64, Nanogram>) -> Self {
        M1 {
            value: val.value * 1_000.0,
            unit: PhantomData,
        }
    }
}