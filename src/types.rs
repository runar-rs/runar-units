//! This module contains the types for the different physical quantities.
//!
//! # Nomenclature
//! The types are named with a list of dimensions, where each dimension is assigned an exponent.
//! The dimensions are sorted alphabetically, different dimensions are separated by an underscore.
//! ## Base Dimensions
//! Dimensions are composed of exponents of the base dimensions. Base dimensions should be named
//! with a single latin letter, or written out greek letters.
//! - `I` - Electric current
//! - `J` - Luminous intensity
//! - `L` - Length
//! - `M` - Mass
//! - `N` - Amount of substance
//! - `T` - Time
//! - `Theta` - Thermodynamic temperature
//!
//! ### Additional Dimensions
//! In addition to the SI base dimensions, the following additional dimensions are defined by this
//! crate:
//! - `Sr` - Steradian (solid angle). In the SI System, the steradian is a dimensionless
//! derived unit. However, this crate defines it as a separate dimension to allow for more precise
//! type checking and to avoid confusion with other dimensionless quantities, or confusion of
//! other quantities, that are given per steradian, such as luminous intensity or radiance.
//!
//! ## Exponents
//! - `1` - Positive exponent
//! - `n1` - Negative exponent

pub mod i1;
pub mod i1_tn1;
pub mod j1;
pub mod l1;
pub mod l1_m1;
pub mod l1_m1_tn1;
pub mod l1_m1_tn2;
pub mod l1_t1;
pub mod l1_tn1;
pub mod l2;
pub mod l3;
pub mod m1;
pub mod n1;
pub mod t1;
pub mod theta1;
pub mod tn1;
