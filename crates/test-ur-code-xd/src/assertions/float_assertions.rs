// Copyright (c) 2023 Sophie Katz
//
// This file is part of test ur code XD.
//
// test ur code XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test ur code XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test ur code XD. If
// not, see <https://www.gnu.org/licenses/>.

//! Assertions that operate on floats.
//!
//! The assertions in this module are based off of
//! <a href="https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/">this excellent article</a>.

use float_cmp::{approx_eq, Ulps};
use num_traits::Float;
use std::fmt::{Debug, Display};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

/// Checks if two numbers are non-finite and equal
///
/// # Arguments
///
/// * `lhs` - The left-hand side of the comparison.
/// * `rhs` - The right-hand side of the comparison.
///
/// # Returns
///
/// * If both numbers are non-finite (either infinite or NaN):
///     * If they are equal, return `Some(true)`
///     * If they are inequal, return `Some(false)`
/// * Otherwise, return `None`
fn is_float_eq_non_finite<FloatType: Float>(lhs: FloatType, rhs: FloatType) -> Option<bool> {
    if lhs.is_infinite() != rhs.is_infinite() {
        // One is infinite and the other is not: inequal
        return Some(false);
    } else if lhs.is_infinite() {
        assert!(rhs.is_infinite());
        // Both are infinite: check for equality between positive and negative
        return Some(lhs == rhs);
    }

    if lhs.is_nan() != rhs.is_nan() {
        // One is NaN and the other is not: inequal
        return Some(false);
    } else if lhs.is_nan() {
        assert!(rhs.is_nan());
        // Both are NaN: equal
        return Some(true);
    }

    assert!(lhs.is_finite());
    assert!(rhs.is_finite());

    None
}

/// Checks if two numbers are equal using a relative epsilon tolerance
///
/// If the two numbers are both finite, a relative epsilon tolerance will be used to compare them.
/// If either are not, [`is_float_eq_non_finite`] will be used instead.
///
/// # Arguments
///
/// * `lhs` - The left-hand side of the comparison.
/// * `rhs` - The right-hand side of the comparison.
/// * `absolute_tolerance` - The absolute tolerance to use when `lhs` and `rhs` are very close to
///                          each other.
/// * `relative_tolerance` - The epsilon to use for tolerance relative to the magnitude of the
///                          largest operand.
fn is_float_eq_relative<FloatType: Float>(
    lhs: FloatType,
    rhs: FloatType,
    absolute_tolerance: FloatType,
    relative_tolerance: FloatType,
) -> bool {
    // Check for non-finite cases
    if let Some(equal) = is_float_eq_non_finite(lhs, rhs) {
        return equal;
    }

    // Calculate absolute difference
    let diff = (lhs - rhs).abs();

    // Check for absolute tolerence first to handle cases close to zero
    if diff <= absolute_tolerance {
        return true;
    }

    // Check for relative tolerance
    if diff <= lhs.abs().max(rhs.abs()) * relative_tolerance {
        return true;
    }

    false
}

/// Checks if two numbers are equal using an ULPs tolerance
///
/// If the two numbers are both finite, an ULPs tolerance will be used to compare them.
/// If either are not, [`is_float_eq_non_finite`] will be used instead.
///
/// # Arguments
///
/// * `lhs` - The left-hand side of the comparison.
/// * `rhs` - The right-hand side of the comparison.
/// * `absolute_tolerance` - The absolute tolerance to use when `lhs` and `rhs` are very close to
///                          each other.
/// * `ulps_tolerance` - The number of ULPs to use for tolerance.
fn is_float_eq_ulps_f32(lhs: f32, rhs: f32, absolute_tolerance: f32, ulps_tolerance: i32) -> bool {
    // Check for non-finite cases
    if let Some(equal) = is_float_eq_non_finite(lhs, rhs) {
        return equal;
    }

    // Calculate absolute difference
    let diff = (lhs - rhs).abs();

    // Check for absolute tolerence first to handle cases close to zero
    if diff <= absolute_tolerance {
        return true;
    }

    // Check for differing signs
    if lhs.is_sign_negative() != rhs.is_sign_negative() {
        return false;
    }

    // Check for ULPS tolerance
    if approx_eq!(f32, lhs, rhs, ulps = ulps_tolerance) {
        return true;
    }

    false
}

/// Checks if two numbers are equal using an ULPs tolerance
///
/// If the two numbers are both finite, an ULPs tolerance will be used to compare them.
/// If either are not, [`is_float_eq_non_finite`] will be used instead.
///
/// # Arguments
///
/// * `lhs` - The left-hand side of the comparison.
/// * `rhs` - The right-hand side of the comparison.
/// * `absolute_tolerance` - The absolute tolerance to use when `lhs` and `rhs` are very close to
///                          each other.
/// * `ulps_tolerance` - The number of ULPs to use for tolerance.
fn is_float_eq_ulps_f64(lhs: f64, rhs: f64, absolute_tolerance: f64, ulps_tolerance: i64) -> bool {
    // Check for non-finite cases
    if let Some(equal) = is_float_eq_non_finite(lhs, rhs) {
        return equal;
    }

    // Calculate absolute difference
    let diff = (lhs - rhs).abs();

    // Check for absolute tolerence first to handle cases close to zero
    if diff <= absolute_tolerance {
        return true;
    }

    // Check for differing signs
    if lhs.is_sign_negative() != rhs.is_sign_negative() {
        return false;
    }

    // Check for ULPS tolerance
    if approx_eq!(f64, lhs, rhs, ulps = ulps_tolerance) {
        return true;
    }

    false
}

/// Formats a predicate description message for a float assertion using an ULPs tolerance
///
/// # Arguments
///
/// * `operator` - The comparison operator (for example `==` or `<`)
/// * `ulps_tolerance` - The ULPs tolerance
/// * `bit_width` - The bit width of the float (either 32 or 64)
/// * `epsilon_near_zero` - The epsilon to use when comparing values near zero
#[doc(hidden)]
pub fn format_float_predicate_description_ulps<
    UlpsType: Display + PartialEq<i32>,
    FloatType: Display,
>(
    operator: &str,
    ulps_tolerance: UlpsType,
    bit_width: usize,
    epsilon_near_zero: FloatType,
) -> String {
    format!(
        "lhs {} rhs (within {} {}-bit float ulp{} or {} near equal)",
        operator,
        ulps_tolerance,
        bit_width,
        if ulps_tolerance == 1 { "" } else { "s" },
        epsilon_near_zero
    )
}

/// Formats a predicate description message for a float assertion using a relative epsilon tolerance
///
/// # Arguments
///
/// * `operator` - The comparison operator (for example `==` or `<`)
/// * `relative_epsilon` - The relative epsilon tolerance
/// * `epsilon_near_zero` - The epsilon to use when comparing values near zero
#[doc(hidden)]
pub fn format_float_predicate_description_relative<FloatType: Display>(
    operator: &str,
    relative_epsilon: FloatType,
    epsilon_near_zero: FloatType,
) -> String {
    format!(
        "lhs {} rhs (within {} relative to magnitude or {} near equal)",
        operator, relative_epsilon, epsilon_near_zero
    )
}

/// Configures a panic message builder for a float assertion using an ULPs tolerance
///
/// # Arguments
///
/// * `panic_message_builder` - The panic message builder to configure
/// * `lhs_description` - The description of the left-hand side of the comparison
/// * `lhs_value` - The left-hand side of the comparison
/// * `rhs_description` - The description of the right-hand side of the comparison
/// * `rhs_value` - The right-hand side of the comparison
#[doc(hidden)]
pub fn configure_float_panic_message_ulps<
    UlpsType: Debug,
    FloatType: Float + Debug + Ulps<U = UlpsType>,
>(
    panic_message_builder: PanicMessageBuilder,
    lhs_description: &str,
    lhs_value: FloatType,
    rhs_description: &str,
    rhs_value: FloatType,
) -> PanicMessageBuilder {
    panic_message_builder
        .with_argument("lhs", lhs_description, &lhs_value)
        .with_argument("rhs", rhs_description, &rhs_value)
        .with_argument("absolute difference", "--", &(lhs_value - rhs_value).abs())
        .with_argument(
            "absolute difference (ulps)",
            "--",
            &lhs_value.ulps(&rhs_value),
        )
}

/// Configures a panic message builder for a float assertion using a relative epsilon  tolerance
///
/// # Arguments
///
/// * `panic_message_builder` - The panic message builder to configure
/// * `lhs_description` - The description of the left-hand side of the comparison
/// * `lhs_value` - The left-hand side of the comparison
/// * `rhs_description` - The description of the right-hand side of the comparison
/// * `rhs_value` - The right-hand side of the comparison
#[doc(hidden)]
pub fn configure_float_panic_message_relative<FloatType: Float + Debug>(
    panic_message_builder: PanicMessageBuilder,
    lhs_description: &str,
    lhs_value: FloatType,
    rhs_description: &str,
    rhs_value: FloatType,
) -> PanicMessageBuilder {
    panic_message_builder
        .with_argument("lhs", lhs_description, &lhs_value)
        .with_argument("rhs", rhs_description, &rhs_value)
        .with_argument("absolute difference", "--", &(lhs_value - rhs_value).abs())
}

#[doc(hidden)]
pub fn assert_f32_eq_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f32_eq_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f32` values are equal.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f32_eq;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f32_eq!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f32_eq!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f32::EPSILON`, relative to magnitude
/// assert_f32_eq!(x, 3.0, relative_epsilon = f32::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f32_eq {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "==",
                $ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_eq_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "==",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_eq_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f32_ne_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    !is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f32_ne_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f32` values are inequal.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f32_ne;
/// #
/// # let x = 4.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f32_ne!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f32_ne!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f32::EPSILON`, relative to magnitude
/// assert_f32_ne!(x, 3.0, relative_epsilon = f32::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f32_ne {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "!=",
                $ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_ne_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f32_le_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    lhs <= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f32_le_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    lhs <= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that one `f32` value is less than or equal to the other.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f32_le;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f32_le!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f32_le!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f32::EPSILON`, relative to magnitude
/// assert_f32_le!(x, 3.0, relative_epsilon = f32::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f32_le {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "<=",
                $ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_le_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "<=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_le_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f32_ge_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    lhs >= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f32_ge_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    lhs >= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that one `f32` value is greater than or equal to the other.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f32_ge;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f32_ge!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f32_ge!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f32::EPSILON`, relative to magnitude
/// assert_f32_ge!(x, 3.0, relative_epsilon = f32::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f32_ge {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                ">=",
                $ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_ge_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                ">=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_ge_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f64_eq_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f64_eq_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f64` values are equal.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f64_eq;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f64_eq!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f64_eq!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f64::EPSILON`, relative to magnitude
/// assert_f64_eq!(x, 3.0, relative_epsilon = f64::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f64_eq {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "==",
                $ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_eq_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "==",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_eq_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f64_ne_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    !is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f64_ne_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f64` values are inequal.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f64_ne;
/// #
/// # let x = 4.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f64_ne!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f64_ne!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f64::EPSILON`, relative to magnitude
/// assert_f64_ne!(x, 3.0, relative_epsilon = f64::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f64_ne {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "!=",
                $ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_ne_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f64_le_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs <= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f64_le_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    lhs <= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that one `f64` value is less than or equal to the other.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f64_le;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f64_le!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f64_le!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f64::EPSILON`, relative to magnitude
/// assert_f64_le!(x, 3.0, relative_epsilon = f64::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f64_le {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "<=",
                $ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_le_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "<=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_le_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_f64_ge_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs >= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
pub fn assert_f64_ge_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    lhs >= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that one `f64` value is greater than or equal to the other.
///
/// # Arguents
///
/// * `lhs` - The left-hand side
/// * `rhs` - The right-hand side
/// * Can use one of:
///     * `ulps = <value>` - The number of ULPs to use for tolerance
///     * `relative_epsilon = <value>` - The epsilon to use for tolerance relative to the magnitude
/// * Optional keyword arguments for assertions
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_f64_ge;
/// #
/// # let x = 3.0;
/// #
/// // Compare `x` to 3.0 within 2 ULPs
/// assert_f64_ge!(x, 3.0, ulps = 2, epsilon_near_zero = 0.0);
///
/// // Compare `x` to 3.0 within 2 ULPs or within 1e-7 if they are very close
/// assert_f64_ge!(x, 3.0, ulps = 2, epsilon_near_zero = 1e-7);
///
/// // Compare `x` to 3.0 within `f64::EPSILON`, relative to magnitude
/// assert_f64_ge!(x, 3.0, relative_epsilon = f64::EPSILON, epsilon_near_zero = 0.0);
/// ```
#[macro_export]
macro_rules! assert_f64_ge {
    (
        $lhs:expr,
        $rhs:expr,
        ulps = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                ">=",
                $ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_ge_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_ulps(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };

    (
        $lhs:expr,
        $rhs:expr,
        relative_epsilon = $relative_epsilon:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                ">=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f64_ge_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message_relative(
                    panic_message_builder,
                    stringify!($lhs),
                    $lhs,
                    stringify!($rhs),
                    $rhs,
                )
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    // use crate::utilities::capture_output::capture_output;

    #[test]
    fn assert_f32_eq_passing() {
        // let captured_outputs = capture_output(|| {
        assert_f32_eq!(1.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
        assert_f32_eq!(
            0.15 + 0.15 + 0.15,
            0.1 + 0.1 + 0.25,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
        assert_f32_eq!(
            0.15 + 0.15 + 0.15,
            0.1 + 0.1 + 0.25,
            ulps = 0,
            epsilon_near_zero = f32::EPSILON
        );
        // })
        // .unwrap();

        // std::assert!(captured_outputs.stdout.is_empty());
        // std::assert!(captured_outputs.stderr.is_empty());
    }
}
