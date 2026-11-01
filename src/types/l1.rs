use core::fmt::Display;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use core::marker::PhantomData;

use crate::types::l1_t1::{L1_T1, MeterSecond};
use crate::types::l1_tn1::{L1_TN1, MeterPerSecond};
use crate::types::t1::{Second, T1};
use crate::{Dimension, QuantityNum, Unit};

pub type Length<V, U> = L1<V, U>;
#[derive(Debug, Clone, Copy)]
pub struct L1<V, U>
where V: QuantityNum, U: L1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: L1Unit> core::fmt::Display for L1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}


impl<V: QuantityNum, U: L1Unit> Dimension for L1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

/// Add with same `ValueType` and same Unit
impl<V: QuantityNum + Add<Output = V>, U: L1Unit> Add<L1<V, U>> for L1<V, U> {
    type Output = Self;

    fn add(self, rhs: L1<V, U>) -> Self::Output {
        L1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// `AddAssign` with same `ValueType` and same Unit
impl<V: QuantityNum, U: L1Unit> AddAssign<L1<V, U>> for L1<V, U> {
    fn add_assign(&mut self, rhs: L1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same `ValueType` and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: L1Unit> Sub<L1<V,U>> for L1<V, U> {
    type Output = Self;

    fn sub(self, rhs: L1<V,U>) -> Self::Output {
        L1 {
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}

/// `SubAssign` with same `ValueType` and same Unit
impl<V: QuantityNum, U: L1Unit> SubAssign<L1<V, U>> for L1<V,U> {
    fn sub_assign(&mut self, rhs: L1<V, U>) {
        self.value -= rhs.value;
    }
}

/// Mul with same `ValueType` and Second and Meter
impl<V: QuantityNum + Mul<Output = V>> Mul<T1<V, Second>> for L1<V, Meter> {
    type Output = L1_T1<V, MeterSecond>;

    fn mul(self, rhs: T1<V, Second>) -> Self::Output {
        L1_T1 {
            unit: PhantomData,
            value: self.value * rhs.value,
        }
    }
}

/// Div with same `ValueType` and Meter divided by Second
impl<V: QuantityNum + Div<Output = V>> Div<T1<V, Second>> for L1<V, Meter> {
    type Output = L1_TN1<V, MeterPerSecond>;

    fn div(self, rhs: T1<V, Second>) -> Self::Output {
        L1_TN1 {
            unit: PhantomData,
            value: self.value / rhs.value,
        }
    }
}
/// `PartialEq` with same `ValueType` and same Unit
impl<V: QuantityNum + PartialEq, U: L1Unit> PartialEq for L1<V, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// `PartialOrd` with same `ValueType` and same Unit
impl<V: QuantityNum + PartialOrd, U: L1Unit> PartialOrd for L1<V, U> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U: L1Unit> L1<f64, U> {
    #[must_use]
    pub fn new(value: f64) -> Self {
        L1 {
            unit: PhantomData,
            value
        }
    }

    #[must_use]
    pub fn abs(self) -> Self {
        L1 {
            unit: PhantomData,
            value: self.value.abs()
        }
    }

}

pub trait L1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Kilometer;
impl Unit for Kilometer {
    const ABBREVIATION: &'static str = "km";
}
impl L1Unit for Kilometer {}

#[derive(Debug, Clone, Copy)]
pub struct Meter;
impl Unit for Meter {
    const ABBREVIATION: &'static str = "m";
}
impl L1Unit for Meter {}

#[derive(Debug, Clone, Copy)]
pub struct Millimeter;
impl Unit for Millimeter {
    const ABBREVIATION: &'static str = "mm";
}
impl L1Unit for Millimeter {}

#[derive(Debug, Clone, Copy)]
pub struct Micrometer;
impl Unit for Micrometer {
    const ABBREVIATION: &'static str = "µm";
}
impl L1Unit for Micrometer {}

#[derive(Debug, Clone, Copy)]
pub struct Nanometer;
impl Unit for Nanometer {
    const ABBREVIATION: &'static str = "nm";
}
impl L1Unit for Nanometer {}

#[cfg(test)]
mod tests {
    use crate::{test_utils::assert_close, types::l1_tn1::MeterPerSecond};

    use super::*;

    #[test]
    fn test_l1_display() {
        let length = L1::<f64, Meter>::new(42.0);
        assert_eq!(format!("{}", length), "42 m");
    }

    #[test]
    fn test_l1_display_precision() {
        let length = L1::<f64, Meter>::new(42.123456789);
        assert_eq!(format!("{:.1}", length), "42.1 m");
        assert_eq!(format!("{:.2}", length), "42.12 m");
        assert_eq!(format!("{:.3}", length), "42.123 m");
    }

    #[test]
    fn test_l1_add() {
        let a = L1::<f64, Meter>::new(5.0);
        let b = L1::<f64, Meter>::new(3.0);
        let c = a + b;
        assert_eq!(c.value, 8.0);
    }

    #[test]
    fn test_l1_add_associativity() {
        let a = L1::<f64, Meter>::new(5.0);
        let b = L1::<f64, Meter>::new(3.0);
        let c = L1::<f64, Meter>::new(2.0);

        let result1 = (a + b) + c;
        let result2 = a + (b + c);

        assert_close(result1.value, result2.value, 1e-10);
    }

    #[test]
    fn test_l1_add_commutativity() {
        let a = L1::<f64, Meter>::new(5.0);
        let b = L1::<f64, Meter>::new(3.0);

        let result1 = a + b;
        let result2 = b + a;

        assert_close(result1.value, result2.value, 1e-10);
    }



    #[test]
    fn test_l1_sub() {
        let a = L1::<f64, Meter>::new(5.0);
        let b = L1::<f64, Meter>::new(3.0);
        let c = a - b;
        assert_close(c.value, 2.0, 1e-10);
    }

    #[test]
    fn test_l1_mul_t1() {
        let length = L1::<f64, Meter>::new(10.0);
        let time = T1::<f64, Second>::new(2.0);

        // Force the type of result to be L1_T1<f64, MeterSecond>, as this is the expected output
        // type of the multiplication
        let result: L1_T1<f64, MeterSecond> = length * time;
        assert_close(result.value, 20.0, 1e-10);
    }

    #[test]
    fn test_l1_div_t1() {
        let length = L1::<f64, Meter>::new(10.0);
        let time = T1::<f64, Second>::new(2.0);

        // Force the type of result to be L1_TN1<f64, MeterSecond>, as this is the expected output
        // type of the division
        let result: L1_TN1<f64, MeterPerSecond> = length / time;
        assert_close(result.value, 5.0, 1e-10);
    }
}