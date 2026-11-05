use core::fmt::Display;
use core::marker::PhantomData;

use crate::types::l1_t1::{L1_T1, MeterSecond};
use crate::types::l1_tn1::L1_TN1;
use crate::types::t1::{Second, T1, T1Unit};
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

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1, L1Unit);

impl_dimension_mul!(L1, T1 => L1_T1, L1Unit, T1Unit);
impl_dimension_div!(L1, T1 => L1_TN1, L1Unit, T1Unit);
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
    #[deprecated(note = "Use the implementation of the unit instead, e.g., `Meter::new(value)`")]
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


/// Unit Kilometer
/// 
/// # Implemented Conversions
/// - [`L1<f32, Kilometer>`] to [`L1<f32, Meter>`]
/// - [`L1<f32, Kilometer>`] to [`L1<f32, Centimeter>`]
/// - [`L1<f32, Kilometer>`] to [`L1<f32, Millimeter>`]
/// - [`L1<f64, Kilometer>`] to [`L1<f64, Meter>`]
/// - [`L1<f64, Kilometer>`] to [`L1<f64, Centimeter>`]
/// - [`L1<f64, Kilometer>`] to [`L1<f64, Millimeter>`]
#[derive(Debug, Clone, Copy)]
pub struct Kilometer;
impl Unit for Kilometer {
    const ABBREVIATION: &'static str = "km";
    const SCALE_TO_BASE: f64 = 1e3;
}
impl L1Unit for Kilometer {}

impl Kilometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Kilometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Meter
/// 
/// # Implemented Conversions
/// - [`L1<f32, Meter>`] to [`L1<f32, Kilometer>`]
/// - [`L1<f32, Meter>`] to [`L1<f32, Centimeter>`]
/// - [`L1<f32, Meter>`] to [`L1<f32, Millimeter>`]
/// - [`L1<f64, Meter>`] to [`L1<f64, Kilometer>`]
/// - [`L1<f64, Meter>`] to [`L1<f64, Centimeter>`]
/// - [`L1<f64, Meter>`] to [`L1<f64, Millimeter>`]
#[derive(Debug, Clone, Copy)]
pub struct Meter;
impl Unit for Meter {
    const ABBREVIATION: &'static str = "m";
}
impl L1Unit for Meter {}

impl Meter {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Meter> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Centimeter
/// 
/// # Implemented Conversions
/// - [`L1<f32, Centimeter>`] to [`L1<f32, Meter>`]
/// - [`L1<f32, Centimeter>`] to [`L1<f32, Millimeter>`]
/// - [`L1<f64, Centimeter>`] to [`L1<f64, Meter>`]
/// - [`L1<f64, Centimeter>`] to [`L1<f64, Millimeter>`]
#[derive(Debug, Clone, Copy)]
pub struct Centimeter;
impl Unit for Centimeter {
    const ABBREVIATION: &'static str = "cm";
    const SCALE_TO_BASE: f64 = 1e-2;
}
impl L1Unit for Centimeter {}

impl Centimeter {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Centimeter> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Millimeter
/// 
/// # Implemented Conversions
/// - [`L1<f32, Millimeter>`] to [`L1<f32, Meter>`]
/// - [`L1<f64, Millimeter>`] to [`L1<f64, Meter>`]
#[derive(Debug, Clone, Copy)]
pub struct Millimeter;
impl Unit for Millimeter {
    const ABBREVIATION: &'static str = "mm";
    const SCALE_TO_BASE: f64 = 1e-3;
}
impl L1Unit for Millimeter {}

impl Millimeter {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Millimeter> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Micrometer;
impl Unit for Micrometer {
    const ABBREVIATION: &'static str = "µm";
    const SCALE_TO_BASE: f64 = 1e-6;
}
impl L1Unit for Micrometer {}

impl Micrometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Micrometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nanometer;
impl Unit for Nanometer {
    const ABBREVIATION: &'static str = "nm";
    const SCALE_TO_BASE: f64 = 1e-9;
}
impl L1Unit for Nanometer {}

impl Nanometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Nanometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Picometer;
impl Unit for Picometer {
    const ABBREVIATION: &'static str = "pm";
    const SCALE_TO_BASE: f64 = 1e-12;
}
impl L1Unit for Picometer {}

impl Picometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Picometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Femtometer;
impl Unit for Femtometer {
    const ABBREVIATION: &'static str = "fm";
    const SCALE_TO_BASE: f64 = 1e-15;
}
impl L1Unit for Femtometer {}

impl Femtometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Femtometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Attometer;
impl Unit for Attometer {
    const ABBREVIATION: &'static str = "am";
    const SCALE_TO_BASE: f64 = 1e-18;
}
impl L1Unit for Attometer {}

impl Attometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Attometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Zeptometer;
impl Unit for Zeptometer {
    const ABBREVIATION: &'static str = "zm";
    const SCALE_TO_BASE: f64 = 1e-21;
}
impl L1Unit for Zeptometer {}

impl Zeptometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Zeptometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Yoctometer;
impl Unit for Yoctometer {
    const ABBREVIATION: &'static str = "ym";
    const SCALE_TO_BASE: f64 = 1e-24;
}
impl L1Unit for Yoctometer {}

impl Yoctometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Yoctometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rontometer;
impl Unit for Rontometer {
    const ABBREVIATION: &'static str = "rm";
    const SCALE_TO_BASE: f64 = 1e-27;
}
impl L1Unit for Rontometer {}

impl Rontometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Rontometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quectometer;
impl Unit for Quectometer {
    const ABBREVIATION: &'static str = "qm";
    const SCALE_TO_BASE: f64 = 1e-30;
}
impl L1Unit for Quectometer {}

impl Quectometer {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L1<V, Quectometer> {
        L1 {
            unit: PhantomData,
            value
        }
    }
}

// Unit Conversions

impl_linear_unit_conversions!(
    L1;
    Kilometer, Meter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer,
    Femtometer, Attometer, Zeptometer, Yoctometer, Rontometer, Quectometer
);

#[cfg(test)]
mod tests {
    use crate::{test_utils::assert_close, types::l1_tn1::MeterPerSecond};

    use super::*;

    #[test]
    fn test_l1_display() {
        let length = Meter::new(42.0);
        assert_eq!(format!("{}", length), "42 m");
    }

    #[test]
    fn test_l1_display_precision() {
        let length = Meter::new(42.123456789);
        assert_eq!(format!("{:.1}", length), "42.1 m");
        assert_eq!(format!("{:.2}", length), "42.12 m");
        assert_eq!(format!("{:.3}", length), "42.123 m");
    }

    #[test]
    fn test_l1_add() {
        let a = Meter::new(5.0);
        let b = Meter::new(3.0);
        let c = a + b;
        assert_eq!(c.value, 8.0);
    }

    #[test]
    fn test_l1_add_associativity() {
        let a = Meter::new(5.0);
        let b = Meter::new(3.0);
        let c = Meter::new(2.0);

        let result1 = (a + b) + c;
        let result2 = a + (b + c);

        assert_close(result1.value, result2.value, 1e-10);
    }

    #[test]
    fn test_l1_add_commutativity() {
        let a = Meter::new(5.0);
        let b = Meter::new(3.0);

        let result1 = a + b;
        let result2 = b + a;

        assert_close(result1.value, result2.value, 1e-10);
    }



    #[test]
    fn test_l1_sub() {
        let a = Meter::new(5.0);
        let b = Meter::new(3.0);
        let c = a - b;
        assert_close(c.value, 2.0, 1e-10);
    }

    #[test]
    fn test_l1_conversion() {
        // Start at meter and go through all conversions and back to meter, checking that the final value is equal to the original value.
        let original = Meter::new(1.0);
        let km: L1<f64, Kilometer> = original.into();
        let cm: L1<f64, Centimeter> = km.into();
        let mm: L1<f64, Millimeter> = cm.into();
        let m: L1<f64, Meter> = mm.into();
        assert_close(m.value, original.value, 1e-10);
    }

    #[test]
    fn test_l1_mul_t1() {
        let length = Meter::new(10.0);
        let time = Second::new(2.0);

        // Force the type of result to be L1_T1<f64, MeterSecond>, as this is the expected output
        // type of the multiplication
        let result: L1_T1<f64, MeterSecond> = length * time;
        assert_close(result.value, 20.0, 1e-10);
    }

    #[test]
    fn test_l1_mul_t1_applies_operand_scales() {
        let length = Kilometer::new(1.0);
        let time = crate::types::t1::Hour::new(1.0);
        let result: L1_T1<f64, MeterSecond> = length * time;

        assert_close(result.value, 1000.0 * 3600.0, 1e-10);
    }

    #[test]
    fn test_l1_div_t1() {
        let length = Meter::new(10.0);
        let time = Second::new(2.0);

        // Force the type of result to be L1_TN1<f64, MeterSecond>, as this is the expected output
        // type of the division
        let result: L1_TN1<f64, MeterPerSecond> = length / time;
        assert_close(result.value, 5.0, 1e-10);
    }

    #[test]
    fn test_l1_div_t1_applies_operand_scales() {
        let length = Kilometer::new(1.0);
        let time = crate::types::t1::Hour::new(1.0);
        let result: L1_TN1<f64, MeterPerSecond> = length / time;

        assert_close(result.value, 1000.0 / 3600.0, 1e-10);
    }
}