#![cfg_attr(not(test), no_std)]

pub trait QuantityNum: Sized {}

pub trait LinearScale: QuantityNum {
    fn from_f64(value: f64) -> Self;
}

impl QuantityNum for f32 {}

impl LinearScale for f32 {
    fn from_f64(value: f64) -> Self {
        value as f32
    }
}

impl QuantityNum for f64 {}

impl LinearScale for f64 {
    fn from_f64(value: f64) -> Self {
        value
    }
}

pub trait Dimension {
    type ValueType: QuantityNum;

    /// Returns a reference to the underlying value of the dimension.
    #[must_use]
    fn get(&self) -> &Self::ValueType;
}

/// Marker Traits for Units
pub trait Unit {
    const ABBREVIATION: &'static str;
    /// Multiplicative factor that converts this unit to the dimension's base unit.
    const SCALE_TO_BASE: f64 = 1.0;
    /// Optional LaTeX abbreviation for the unit. If not provided, the `ABBREVIATION` will be used.
    const LATEX_ABBREVIATION: &'static str = Self::ABBREVIATION;
}

/// Implements multiplication between two dimensions with an explicit output unit.
///
/// The generated implementation accepts any units implementing the supplied
/// marker traits and returns `$output<V, $output_unit>`. The numeric result is
/// scaled using:
///
/// ```text
/// lhs_value * rhs_value
///     * lhs_scale_to_base
///     * rhs_scale_to_base
///     / output_scale_to_base
/// ```
///
/// # Parameters
///
/// - `$lhs`: path to the left-hand quantity type, without its unit parameter.
/// - `$rhs`: path to the right-hand quantity type, without its unit parameter.
/// - `$output`: path to the result quantity type, without its unit parameter.
/// - `$output_unit`: concrete unit used by the result.
/// - `$lhs_bound`: marker trait implemented by left-hand units.
/// - `$rhs_bound`: marker trait implemented by right-hand units.
///
///
///
/// This implements `L1<f64, Unit> * f64`, returning `L1<f64, Meter>`.
/// The scalar is dimensionless, so its scale is `1.0`. A scalar-left
/// implementation is intentionally not generated because Rust's orphan rules
/// do not allow implementing `Mul<Quantity>` for primitive scalar types.
///
/// Example:
///
/// ```text
/// impl_dimension_mul_with_unit!(
///     crate::types::l1::L1,
///     crate::types::l1::L1Unit,
///     crate::types::t1::T1,
///     crate::types::t1::T1Unit
///         => crate::types::l1_t1::L1_T1,
///     crate::types::l1_t1::MeterSecond
/// );
/// ```
#[macro_export]
macro_rules! impl_dimension_mul_with_unit {
    (scalar, (), $($rhs:ident)::+, $rhs_bound:path => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UR> core::ops::Mul<$($rhs)::+<V, UR>> for V
        where
            V: crate::LinearScale + core::ops::Mul<Output = V>,
            UR: $rhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn mul(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self * rhs.value)
                        * V::from_f64(UR::SCALE_TO_BASE / <$output_unit as crate::Unit>::SCALE_TO_BASE,),
                }
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, scalar, () => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UL> core::ops::Mul<V> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn mul(self, rhs: V) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self.value * rhs) * V::from_f64(UL::SCALE_TO_BASE / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, $($rhs:ident)::+, $rhs_bound:path => scalar, ()) => {
        impl<V, UL, UR> core::ops::Mul<$($rhs)::+<V, UR>> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = V;

            fn mul(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                (self.value * rhs.value)
                    * V::from_f64(UL::SCALE_TO_BASE * UR::SCALE_TO_BASE)
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, $($rhs:ident)::+, $rhs_bound:path => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UL, UR> core::ops::Mul<$($rhs)::+<V, UR>> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn mul(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self.value * rhs.value)
                        * V::from_f64(
                            UL::SCALE_TO_BASE
                                * UR::SCALE_TO_BASE
                                / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
}

/// Implements division between two dimensions with an explicit output unit.
///
/// The generated implementation accepts any units implementing the supplied
/// marker traits and returns `$output<V, $output_unit>`. The numeric result is
/// scaled using:
///
/// ```text
/// lhs_value / rhs_value
///     * lhs_scale_to_base
///     / rhs_scale_to_base
///     / output_scale_to_base
/// ```
///
/// The value type must support division, multiplication, and conversion from
/// an `f64` scale through [`LinearScale`].
///
/// # Parameters
///
/// The parameters have the same meaning as
/// [`impl_dimension_mul_with_unit`]. The generated operation is:
///
/// ```text
/// Lhs<V, LhsUnit> / Rhs<V, RhsUnit>
///     -> Output<V, OutputUnit>
/// ```
#[macro_export]
macro_rules! impl_dimension_div_with_unit {
    // Leave lhs_bound in the macro to avoid confusion with the scalar case, but it is not used.
    (scalar, (), $($rhs:ident)::+, $rhs_bound:path => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UR> core::ops::Div<$($rhs)::+<V, UR>> for V
        where
            V: crate::LinearScale
                + core::ops::Div<Output = V>
                + core::ops::Mul<Output = V>,
            UR: $rhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn div(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self / rhs.value)
                        * V::from_f64(UR::SCALE_TO_BASE / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, scalar, () => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UL> core::ops::Div<V> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Div<Output = V>
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn div(self, rhs: V) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self.value / rhs)
                        * V::from_f64(
                            UL::SCALE_TO_BASE / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, $($rhs:ident)::+, $rhs_bound:path => scalar, ()) => {
        impl<V, UL, UR> core::ops::Div<$($rhs)::+<V, UR>> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Div<Output = V>
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = V;

            fn div(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                (self.value / rhs.value)
                            * V::from_f64(
                                UL::SCALE_TO_BASE
                                    / UR::SCALE_TO_BASE
                            ),
            }
        }
    };
    ($($lhs:ident)::+, $lhs_bound:path, $($rhs:ident)::+, $rhs_bound:path => $($output:ident)::+, $output_unit:ty) => {
        impl<V, UL, UR> core::ops::Div<$($rhs)::+<V, UR>> for $($lhs)::+<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Div<Output = V>
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = $($output)::+<V, $output_unit>;

            fn div(self, rhs: $($rhs)::+<V, UR>) -> Self::Output {
                $($output)::+ {
                    unit: core::marker::PhantomData,
                    value: (self.value / rhs.value)
                        * V::from_f64(
                            UL::SCALE_TO_BASE
                                / UR::SCALE_TO_BASE
                                / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
}

/// Implements arithmetic common to every unit of a dimension.
///
/// The generated implementations provide:
///
/// - `Add<Self>` and `AddAssign<Self>`
/// - `Sub<Self>` and `SubAssign<Self>`
/// - `Mul<V>` and `MulAssign<V>`
/// - `Div<V>` and `DivAssign<V>`
/// - `Div<Self>`
///
/// `$quantity` is a path to the quantity type and `$unit_bound` is its unit
/// marker trait. Operations preserve the quantity's unit and only operate on the
/// underlying numeric value.
///
/// # Parameters
///
/// - `$quantity`: quantity type receiving the implementations.
/// - `$unit_bound`: marker trait restricting its unit parameter.
#[macro_export]
macro_rules! impl_dimension_ops {
    ($($quantity:ident)::+, $unit_bound:path) => {
        impl<V: crate::QuantityNum + core::ops::Add<Output = V>, U: $unit_bound>
            core::ops::Add for $($quantity)::+<V, U>
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value + rhs.value,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::AddAssign, U: $unit_bound>
            core::ops::AddAssign for $($quantity)::+<V, U>
        {
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Sub<Output = V>, U: $unit_bound>
            core::ops::Sub for $($quantity)::+<V, U>
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value - rhs.value,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::SubAssign, U: $unit_bound>
            core::ops::SubAssign for $($quantity)::+<V, U>
        {
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Mul<Output = V>, U: $unit_bound>
            core::ops::Mul<V> for $($quantity)::+<V, U>
        {
            type Output = Self;

            fn mul(self, rhs: V) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value * rhs,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::MulAssign, U: $unit_bound>
            core::ops::MulAssign<V> for $($quantity)::+<V, U>
        {
            fn mul_assign(&mut self, rhs: V) {
                self.value *= rhs;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Div<Output = V>, U: $unit_bound>
            core::ops::Div<V> for $($quantity)::+<V, U>
        {
            type Output = Self;

            fn div(self, rhs: V) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value / rhs,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::DivAssign, U: $unit_bound>
            core::ops::DivAssign<V> for $($quantity)::+<V, U>
        {
            fn div_assign(&mut self, rhs: V) {
                self.value /= rhs;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Div<Output = V>, U: $unit_bound>
            core::ops::Div<Self> for $($quantity)::+<V, U>
        {
            type Output = V;

            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }
    };
}

/// Implements one directed linear conversion between two units.
///
/// `$factor` is multiplied into the source value. The list-based
/// [`impl_linear_unit_conversions`] macro normally supplies this factor from
/// the units' [`Unit::SCALE_TO_BASE`] values.
///
/// # Parameters
///
/// - `$quantity`: quantity type, such as `L1`.
/// - `$from`: source unit marker type.
/// - `$to`: destination unit marker type.
/// - `$factor`: multiplier converting the source value to the destination
///   unit.
macro_rules! impl_linear_unit_conversion_pair {
    (
        $quantity:ident,
        $from:ty => $to:ty,
        $factor:expr
    ) => {
        impl<V> From<$quantity<V, $from>> for $quantity<V, $to>
        where
            V: crate::LinearScale + core::ops::Mul<Output = V>,
        {
            fn from(value: $quantity<V, $from>) -> Self {
                $quantity {
                    unit: core::marker::PhantomData,
                    value: value.value * V::from_f64($factor),
                }
            }
        }
    };
}

/// Implements linear conversions for every distinct pair in a unit list.
///
/// Each pair is generated in both directions. For a source unit `From` and
/// destination unit `To`, the factor is:
///
/// ```text
/// From::SCALE_TO_BASE / To::SCALE_TO_BASE
/// ```
///
/// # Example
///
/// ```text
/// impl_linear_unit_conversions!(
///     Length;
///     Kilometer, Meter, Centimeter
/// );
/// ```
///
/// The invocation generates conversions between all three units, including
/// both directions for every pair. The quantity type must have the form
/// `$quantity<V, Unit>`, and every listed type must implement [`Unit`].
///
/// # Parameters
///
/// - `$quantity`: quantity type whose unit is the second type parameter.
/// - Unit list: two or more unit marker types implementing [`Unit`].
///
/// For `N` listed units, this generates `N * (N - 1)` directed conversions.
macro_rules! impl_linear_unit_conversions {
    ($quantity:ident; $head:ty $(, $tail:ty)+ $(,)?) => {
        $(
            impl_linear_unit_conversion_pair!(
                $quantity,
                $head => $tail,
                <$head as crate::Unit>::SCALE_TO_BASE
                    / <$tail as crate::Unit>::SCALE_TO_BASE
            );
            impl_linear_unit_conversion_pair!(
                $quantity,
                $tail => $head,
                <$tail as crate::Unit>::SCALE_TO_BASE
                    / <$head as crate::Unit>::SCALE_TO_BASE
            );
        )*

        impl_linear_unit_conversions!(@rest $quantity; $($tail),*);
    };
    (@rest $quantity:ident; $head:ty $(, $tail:ty)+ $(,)?) => {
        impl_linear_unit_conversions!($quantity; $head, $($tail),*);
    };
    (@rest $quantity:ident; $single:ty) => {};
}

pub mod consts;
pub mod types;

#[cfg(test)]
mod test_utils {
    use core::marker::PhantomData;

    use crate::{QuantityNum, Unit};

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

    struct ScaledUnit;

    impl Unit for ScaledUnit {
        const ABBREVIATION: &'static str = "su";
        const SCALE_TO_BASE: f64 = 100.0;
    }

    struct BaseUnit;

    impl Unit for BaseUnit {
        const ABBREVIATION: &'static str = "bu";
    }

    trait TestUnit: Unit {}

    impl TestUnit for ScaledUnit {}
    impl TestUnit for BaseUnit {}

    struct TestQuantity<V, U>
    where
        V: QuantityNum,
        U: TestUnit,
    {
        unit: PhantomData<U>,
        value: V,
    }

    struct ScalarQuantity<V, U>
    where
        V: QuantityNum,
        U: TestUnit,
    {
        unit: PhantomData<U>,
        value: V,
    }

    impl_dimension_mul_with_unit!(
        TestQuantity, TestUnit,
        TestQuantity, TestUnit => TestQuantity, BaseUnit
    );

    impl_dimension_mul_with_unit!(
        TestQuantity, TestUnit,
        scalar, () => TestQuantity,
        BaseUnit
    );

    impl_dimension_mul_with_unit!(
        ScalarQuantity, TestUnit,
        TestQuantity, TestUnit => scalar, ()
    );

    #[test]
    fn dimension_multiplication_applies_both_input_scales() {
        let value = TestQuantity {
            unit: PhantomData::<ScaledUnit>,
            value: 2.0_f64,
        } * TestQuantity {
            unit: PhantomData::<ScaledUnit>,
            value: 3.0_f64,
        };

        assert_close(value.value, 60_000.0, f64::EPSILON);
        assert!(core::mem::size_of_val(&value.unit) == 0);
    }

    #[test]
    fn scalar_multiplication_applies_the_input_scale() {
        let value = TestQuantity {
            unit: PhantomData::<ScaledUnit>,
            value: 2.0_f64,
        } * 3.0;

        assert_close(value.value, 600.0, f64::EPSILON);
    }

    #[test]
    fn dimension_multiplication_can_return_a_scalar() {
        let value = ScalarQuantity {
            unit: PhantomData::<ScaledUnit>,
            value: 2.0_f64,
        } * TestQuantity {
            unit: PhantomData::<BaseUnit>,
            value: 3.0_f64,
        };

        assert_close(value, 600.0, f64::EPSILON);
    }
}
