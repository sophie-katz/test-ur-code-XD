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
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
//! for a usage guide.
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
///     * If they are unequal, return `Some(false)`
/// * Otherwise, return `None`
fn is_float_eq_non_finite<FloatType: Float>(lhs: FloatType, rhs: FloatType) -> Option<bool> {
    if lhs.is_infinite() != rhs.is_infinite() {
        // One is infinite and the other is not: unequal
        Some(false)
    } else if lhs.is_infinite() {
        // Both are infinite: check for equality between positive and negative
        Some(lhs == rhs)
    } else if lhs.is_nan() != rhs.is_nan() {
        // One is NaN and the other is not: unequal
        Some(false)
    } else if lhs.is_nan() {
        // Both are NaN: equal
        Some(true)
    } else {
        None
    }
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
    #[allow(clippy::arithmetic_side_effects)]
    let diff = (lhs - rhs).abs();

    // Check for absolute tolerance first to handle cases close to zero
    if diff <= absolute_tolerance {
        return true;
    }

    // Check for relative tolerance
    #[allow(clippy::arithmetic_side_effects)]
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

    // Check for absolute tolerance first to handle cases close to zero
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

    // Check for absolute tolerance first to handle cases close to zero
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
    FloatType: Debug,
>(
    operator: &str,
    ulps_tolerance: &UlpsType,
    bit_width: usize,
    epsilon_near_zero: FloatType,
) -> String {
    format!(
        "lhs {} rhs (within {} {}-bit float ulp{} or {:?} near zero)",
        operator,
        ulps_tolerance,
        bit_width,
        if *ulps_tolerance == 1 { "" } else { "s" },
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
pub fn format_float_predicate_description_relative<FloatType: Debug>(
    operator: &str,
    relative_epsilon: FloatType,
    epsilon_near_zero: FloatType,
) -> String {
    format!(
        "lhs {operator} rhs (within {relative_epsilon:?} relative to magnitude or {epsilon_near_zero:?} near zero)"
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
    #[allow(clippy::arithmetic_side_effects)]
    panic_message_builder
        .with_argument("lhs", lhs_description, &lhs_value)
        .with_argument("rhs", rhs_description, &rhs_value)
        .with_argument("absolute difference", "--", &(lhs_value - rhs_value).abs())
        .with_argument(
            "absolute difference (ulps)",
            "--",
            &if lhs_value < rhs_value {
                rhs_value.ulps(&lhs_value)
            } else {
                lhs_value.ulps(&rhs_value)
            },
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
    #[allow(clippy::arithmetic_side_effects)]
    panic_message_builder
        .with_argument("lhs", lhs_description, &lhs_value)
        .with_argument("rhs", rhs_description, &rhs_value)
        .with_argument("absolute difference", "--", &(lhs_value - rhs_value).abs())
}

#[doc(hidden)]
#[must_use]
pub fn assert_f32_eq_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "==",
                &$ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_eq_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i32, f32>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "==",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_eq_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f32>(
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
#[must_use]
pub fn assert_f32_ne_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    !is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
pub fn assert_f32_ne_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f32` values are unequal.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "!=",
                &$ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_ne_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i32, f32>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f32>(
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
#[must_use]
pub fn assert_f32_le_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    lhs <= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "<=",
                &$ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_le_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i32, f32>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "<=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_le_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f32>(
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
#[must_use]
pub fn assert_f32_ge_impl_ulps(lhs: f32, rhs: f32, epsilon_near_zero: f32, ulps: i32) -> bool {
    lhs >= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                ">=",
                &$ulps,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_ge_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i32, f32>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                ">=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f32_ge_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f32>(
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
#[must_use]
pub fn assert_f64_eq_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "==",
                &$ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_eq_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i64, f64>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "==",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_eq_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f64>(
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
#[must_use]
pub fn assert_f64_ne_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    !is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
pub fn assert_f64_ne_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

/// Asserts that two `f64` values are unequal.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "!=",
                &$ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_ne_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i64, f64>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f64>(
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
#[must_use]
pub fn assert_f64_le_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs <= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                "<=",
                &$ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_le_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i64, f64>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                "<=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_le_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f64>(
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
#[must_use]
pub fn assert_f64_ge_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs >= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

#[doc(hidden)]
#[must_use]
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/float](https://sophie-katz.github.io/test-ur-code-XD/assertions/float/)
/// for a usage guide.
///
/// # Arguments
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
            $crate::assertions::float::format_float_predicate_description_ulps(
                ">=",
                &$ulps,
                64,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_ge_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_ulps::<i64, f64>(
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
            $crate::assertions::float::format_float_predicate_description_relative(
                ">=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::float::assert_f64_ge_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float::configure_float_panic_message_relative::<f64>(
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
    use super::*;
    use crate::assert_eq;

    #[test]
    fn is_float_eq_non_finite_f32_infinity_infinity() {
        assert_eq!(
            is_float_eq_non_finite(f32::INFINITY, f32::INFINITY),
            Some(true)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_infinity_neg_infinity() {
        assert_eq!(
            is_float_eq_non_finite(f32::INFINITY, -f32::INFINITY),
            Some(false)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_infinity_nan() {
        assert_eq!(is_float_eq_non_finite(f32::INFINITY, f32::NAN), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_infinity_five() {
        assert_eq!(is_float_eq_non_finite(f32::INFINITY, 5.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_infinity_zero() {
        assert_eq!(is_float_eq_non_finite(f32::INFINITY, 0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_infinity_neg_zero() {
        assert_eq!(is_float_eq_non_finite(f32::INFINITY, -0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_infinity() {
        assert_eq!(
            is_float_eq_non_finite(-f32::INFINITY, f32::INFINITY),
            Some(false)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_neg_infinity() {
        assert_eq!(
            is_float_eq_non_finite(-f32::INFINITY, -f32::INFINITY),
            Some(true)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_nan() {
        assert_eq!(
            is_float_eq_non_finite(-f32::INFINITY, f32::NAN),
            Some(false)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_five() {
        assert_eq!(is_float_eq_non_finite(-f32::INFINITY, 5.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_zero() {
        assert_eq!(is_float_eq_non_finite(-f32::INFINITY, 0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_infinity_neg_zero() {
        assert_eq!(is_float_eq_non_finite(-f32::INFINITY, -0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_infinity() {
        assert_eq!(is_float_eq_non_finite(f32::NAN, f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_neg_infinity() {
        assert_eq!(
            is_float_eq_non_finite(f32::NAN, -f32::INFINITY),
            Some(false)
        );
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_nan() {
        assert_eq!(is_float_eq_non_finite(f32::NAN, f32::NAN), Some(true));
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_five() {
        assert_eq!(is_float_eq_non_finite(f32::NAN, 5.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_zero() {
        assert_eq!(is_float_eq_non_finite(f32::NAN, 0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_nan_neg_zero() {
        assert_eq!(is_float_eq_non_finite(f32::NAN, -0.0), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_infinity() {
        assert_eq!(is_float_eq_non_finite(5.0, f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_neg_infinity() {
        assert_eq!(is_float_eq_non_finite(5.0, -f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_nan() {
        assert_eq!(is_float_eq_non_finite(5.0, f32::NAN), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_five() {
        assert_eq!(is_float_eq_non_finite(5.0, 5.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_zero() {
        assert_eq!(is_float_eq_non_finite(5.0, 0.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_five_neg_zero() {
        assert_eq!(is_float_eq_non_finite(5.0, -0.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_infinity() {
        assert_eq!(is_float_eq_non_finite(0.0, f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_neg_infinity() {
        assert_eq!(is_float_eq_non_finite(0.0, -f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_nan() {
        assert_eq!(is_float_eq_non_finite(0.0, f32::NAN), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_five() {
        assert_eq!(is_float_eq_non_finite(0.0, 5.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_zero() {
        assert_eq!(is_float_eq_non_finite(0.0, 0.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_zero_neg_zero() {
        assert_eq!(is_float_eq_non_finite(0.0, -0.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_infinity() {
        assert_eq!(is_float_eq_non_finite(-0.0, f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_neg_infinity() {
        assert_eq!(is_float_eq_non_finite(-0.0, -f32::INFINITY), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_nan() {
        assert_eq!(is_float_eq_non_finite(-0.0, f32::NAN), Some(false));
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_five() {
        assert_eq!(is_float_eq_non_finite(-0.0, 5.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_zero() {
        assert_eq!(is_float_eq_non_finite(-0.0, 0.0), None::<bool>);
    }

    #[test]
    fn is_float_eq_non_finite_f32_neg_zero_neg_zero() {
        assert_eq!(is_float_eq_non_finite(-0.0, -0.0), None::<bool>);
    }

    #[test]
    fn format_float_predicate_description_ulps_simple() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &0, 32, 0.0),
            "lhs == rhs (within 0 32-bit float ulps or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_operator() {
        assert_eq!(
            format_float_predicate_description_ulps("!=", &0, 32, 0.0),
            "lhs != rhs (within 0 32-bit float ulps or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_ulps_tolerance_1() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &1, 32, 0.0),
            "lhs == rhs (within 1 32-bit float ulp or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_ulps_tolerance_2() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &2, 32, 0.0),
            "lhs == rhs (within 2 32-bit float ulps or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_bit_width() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &0, 64, 0.0),
            "lhs == rhs (within 0 64-bit float ulps or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_epsilon_near_zero_e_neg_30() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &0, 32, 1e-30),
            "lhs == rhs (within 0 32-bit float ulps or 1e-30 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_ulps_epsilon_near_zero_1() {
        assert_eq!(
            format_float_predicate_description_ulps("==", &0, 32, 1.0),
            "lhs == rhs (within 0 32-bit float ulps or 1.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_simple() {
        assert_eq!(
            format_float_predicate_description_relative("==", 0.0, 0.0),
            "lhs == rhs (within 0.0 relative to magnitude or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_operator() {
        assert_eq!(
            format_float_predicate_description_relative("!=", 0.0, 0.0),
            "lhs != rhs (within 0.0 relative to magnitude or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_relative_epsilon_e_neg_30() {
        assert_eq!(
            format_float_predicate_description_relative("==", 1e-30, 0.0),
            "lhs == rhs (within 1e-30 relative to magnitude or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_relative_epsilon_1() {
        assert_eq!(
            format_float_predicate_description_relative("==", 1.0, 0.0),
            "lhs == rhs (within 1.0 relative to magnitude or 0.0 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_epsilon_near_zero_e_neg_30() {
        assert_eq!(
            format_float_predicate_description_relative("==", 0.0, 1e-30),
            "lhs == rhs (within 0.0 relative to magnitude or 1e-30 near zero)"
        );
    }

    #[test]
    fn format_float_predicate_description_relative_epsilon_near_zero_1() {
        assert_eq!(
            format_float_predicate_description_relative("==", 0.0, 1.0),
            "lhs == rhs (within 0.0 relative to magnitude or 1.0 near zero)"
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps() {
        assert_f32_eq!(1.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_1_ulps() {
        assert_f32_eq!(1.0, 1.000_000_1, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps() {
        assert_f32_eq!(1.0, 1.000_000_2, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_2_ulps() {
        assert_f32_eq!(1.0, 1.000_000_2, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps() {
        assert_f32_eq!(1.0, 1.000_000_3, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact() {
        assert_f32_eq!(1.0, 1.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact() {
        assert_f32_eq!(
            1.0,
            1.000_000_1,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon() {
        assert_f32_eq!(
            0.5,
            0.999_999_94,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon() {
        assert_f32_eq!(0.5, 1.0, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon() {
        assert_f32_eq!(
            0.5,
            1.000_000_1,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_000.0,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_1_ulps_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_060.0,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_100.0,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_2_ulps_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_100.0,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_200.0,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_000.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact_large() {
        assert_f32_eq!(
            1_000_000_000.0,
            1_000_000_060.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon_large() {
        assert_f32_eq!(
            500_000_000.0,
            999_999_940.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon_large() {
        assert_f32_eq!(
            500_000_000.0,
            1_000_000_000.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon_large() {
        assert_f32_eq!(
            500_000_000.0,
            1_000_000_060.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_near_zero() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_1_ulps_near_zero() {
        assert_f32_eq!(0.0, 1e-45, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps_near_zero() {
        assert_f32_eq!(0.0, 3e-45, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_2_ulps_near_zero() {
        assert_f32_eq!(0.0, 3e-45, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps_near_zero() {
        assert_f32_eq!(0.0, 4e-45, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact_near_zero() {
        assert_f32_eq!(0.0, 0.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact_near_zero() {
        assert_f32_eq!(0.0, 1e-45, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon_near_zero() {
        assert_f32_eq!(0.0, 1e-45, relative_epsilon = 2.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon_near_zero() {
        assert_f32_eq!(0.0, 1e-45, relative_epsilon = 1.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon_near_zero() {
        assert_f32_eq!(0.0, 1e-45, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_absolute_epsilon_1_ulp_below() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_absolute_epsilon_exact() {
        assert_f32_eq!(0.0, 1e-45, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_0_ulps_absolute_epsilon_1_ulp_above() {
        assert_f32_eq!(0.0, 3e-45, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    fn assert_f32_eq_passing_absolute_epsilon_looser() {
        assert_f32_eq!(0.0, 3e-45, ulps = 1, epsilon_near_zero = 3e-45);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_negative() {
        assert_f32_eq!(-1.0, -1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_1_ulps_negative() {
        assert_f32_eq!(-1.0, -1.000_000_1, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps_negative() {
        assert_f32_eq!(-1.0, -1.000_000_2, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_2_ulps_negative() {
        assert_f32_eq!(-1.0, -1.000_000_2, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps_negative() {
        assert_f32_eq!(-1.0, -1.000_000_3, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact_negative() {
        assert_f32_eq!(-1.0, -1.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact_negative() {
        assert_f32_eq!(
            -1.0,
            -1.000_000_1,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon_negative() {
        assert_f32_eq!(
            -0.5,
            -0.999_999_94,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon_negative() {
        assert_f32_eq!(-0.5, -1.0, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon_negative() {
        assert_f32_eq!(
            -0.5,
            -1.000_000_1,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_000.0,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_1_ulps_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_060.0,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_100.0,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_2_ulps_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_100.0,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_200.0,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_000.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact_large_negative() {
        assert_f32_eq!(
            -1_000_000_000.0,
            -1_000_000_060.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon_large_negative() {
        assert_f32_eq!(
            -500_000_000.0,
            -999_999_940.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon_large_negative() {
        assert_f32_eq!(
            -500_000_000.0,
            -1_000_000_000.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon_large_negative() {
        assert_f32_eq!(
            -500_000_000.0,
            -1_000_000_060.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_near_zero_negative() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_1_ulps_near_zero_negative() {
        assert_f32_eq!(0.0, -1e-45, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_2_ulps_near_zero_negative() {
        assert_f32_eq!(0.0, -3e-45, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_exact_near_zero_negative() {
        assert_f32_eq!(0.0, 0.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_exact_near_zero_negative() {
        assert_f32_eq!(0.0, -1e-45, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_1_ulp_below_epsilon_near_zero_negative() {
        assert_f32_eq!(0.0, -1e-45, relative_epsilon = 2.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_at_epsilon_near_zero_negative() {
        assert_f32_eq!(0.0, -1e-45, relative_epsilon = 1.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_1_ulp_above_epsilon_near_zero_negative() {
        assert_f32_eq!(0.0, -1e-45, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_absolute_epsilon_1_ulp_below_negative() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_0_ulps_absolute_epsilon_exact_negative() {
        assert_f32_eq!(0.0, -1e-45, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_0_ulps_absolute_epsilon_1_ulp_above_negative() {
        assert_f32_eq!(0.0, -3e-45, ulps = 0, epsilon_near_zero = 1e-45);
    }

    #[test]
    fn assert_f32_eq_passing_absolute_epsilon_looser_negative() {
        assert_f32_eq!(0.0, -3e-45, ulps = 1, epsilon_near_zero = 3e-45);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps() {
        assert_f64_eq!(1.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_1_ulps() {
        assert_f64_eq!(
            1.0,
            1.000_000_000_000_000_2,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps() {
        assert_f64_eq!(
            1.0,
            1.000_000_000_000_000_4,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_2_ulps() {
        assert_f64_eq!(
            1.0,
            1.000_000_000_000_000_4,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps() {
        assert_f64_eq!(
            1.0,
            1.000_000_000_000_000_7,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact() {
        assert_f64_eq!(1.0, 1.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact() {
        assert_f64_eq!(
            1.0,
            1.000_000_000_000_000_2,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon() {
        assert_f64_eq!(
            0.5,
            0.999_999_999_999_999_9,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon() {
        assert_f64_eq!(0.5, 1.0, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon() {
        assert_f64_eq!(
            0.5,
            1.000_000_000_000_000_2,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.0,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_1_ulps_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.000_000_1,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.000_000_2,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_2_ulps_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.000_000_2,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.000_000_4,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact_large() {
        assert_f64_eq!(
            1_000_000_000.0,
            1_000_000_000.000_000_1,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon_large() {
        assert_f64_eq!(
            500_000_000.0,
            999_999_999.999_999_9,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon_large() {
        assert_f64_eq!(
            500_000_000.0,
            1_000_000_000.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon_large() {
        assert_f64_eq!(
            500_000_000.0,
            1_000_000_000.000_000_1,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_near_zero() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_1_ulps_near_zero() {
        assert_f64_eq!(0.0, 5e-324, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps_near_zero() {
        assert_f64_eq!(0.0, 1e-323, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_2_ulps_near_zero() {
        assert_f64_eq!(0.0, 1e-323, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps_near_zero() {
        assert_f64_eq!(0.0, 1.5e-323, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact_near_zero() {
        assert_f64_eq!(0.0, 0.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact_near_zero() {
        assert_f64_eq!(0.0, 5e-324, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon_near_zero() {
        assert_f64_eq!(0.0, 5e-324, relative_epsilon = 2.0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon_near_zero() {
        assert_f64_eq!(0.0, 5e-324, relative_epsilon = 1.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon_near_zero() {
        assert_f64_eq!(0.0, 5e-324, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_absolute_epsilon_1_ulp_below() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_absolute_epsilon_exact() {
        assert_f64_eq!(0.0, 5e-324, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_0_ulps_absolute_epsilon_1_ulp_above() {
        assert_f64_eq!(0.0, 1e-323, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    fn assert_f64_eq_passing_absolute_epsilon_looser() {
        assert_f64_eq!(0.0, 1e-323, ulps = 1, epsilon_near_zero = 1e-323);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_negative() {
        assert_f64_eq!(-1.0, -1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_1_ulps_negative() {
        assert_f64_eq!(
            -1.0,
            -1.000_000_000_000_000_2,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps_negative() {
        assert_f64_eq!(
            -1.0,
            -1.000_000_000_000_000_4,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_2_ulps_negative() {
        assert_f64_eq!(
            -1.0,
            -1.000_000_000_000_000_4,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps_negative() {
        assert_f64_eq!(
            -1.0,
            -1.000_000_000_000_000_7,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact_negative() {
        assert_f64_eq!(-1.0, -1.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact_negative() {
        assert_f64_eq!(
            -1.0,
            -1.000_000_000_000_000_2,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon_negative() {
        assert_f64_eq!(
            -0.5,
            -0.999_999_999_999_999_9,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon_negative() {
        assert_f64_eq!(-0.5, -1.0, relative_epsilon = 0.5, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon_negative() {
        assert_f64_eq!(
            -0.5,
            -1.000_000_000_000_000_2,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.0,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_1_ulps_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.000_000_1,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.000_000_2,
            ulps = 1,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_2_ulps_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.000_000_2,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.000_000_4,
            ulps = 2,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact_large_negative() {
        assert_f64_eq!(
            -1_000_000_000.0,
            -1_000_000_000.000_000_1,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon_large_negative() {
        assert_f64_eq!(
            -500_000_000.0,
            -999_999_999.999_999_9,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon_large_negative() {
        assert_f64_eq!(
            -500_000_000.0,
            -1_000_000_000.0,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon_large_negative() {
        assert_f64_eq!(
            -500_000_000.0,
            -1_000_000_000.000_000_1,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_near_zero_negative() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_1_ulps_near_zero_negative() {
        assert_f64_eq!(0.0, -5e-324, ulps = 1, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_2_ulps_near_zero_negative() {
        assert_f64_eq!(0.0, -1e-323, ulps = 2, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_relative_exact_near_zero_negative() {
        assert_f64_eq!(0.0, 0.0, relative_epsilon = 0.0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_exact_near_zero_negative() {
        assert_f64_eq!(
            0.0,
            -5e-324,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_1_ulp_below_epsilon_near_zero_negative() {
        assert_f64_eq!(
            0.0,
            -5e-324,
            relative_epsilon = 2.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_at_epsilon_near_zero_negative() {
        assert_f64_eq!(
            0.0,
            -5e-324,
            relative_epsilon = 1.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_1_ulp_above_epsilon_near_zero_negative() {
        assert_f64_eq!(
            0.0,
            -5e-324,
            relative_epsilon = 0.5,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_absolute_epsilon_1_ulp_below_negative() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    fn assert_f64_eq_passing_ulps_0_ulps_absolute_epsilon_exact_negative() {
        assert_f64_eq!(0.0, -5e-324, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_0_ulps_absolute_epsilon_1_ulp_above_negative() {
        assert_f64_eq!(0.0, -1e-323, ulps = 0, epsilon_near_zero = 5e-324);
    }

    #[test]
    fn assert_f64_eq_passing_absolute_epsilon_looser_negative() {
        assert_f64_eq!(0.0, -1e-323, ulps = 1, epsilon_near_zero = 1e-323);
    }

    #[test]
    fn assert_f32_eq_passing_simple() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_simple() {
        assert_f32_eq!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_ne_passing_simple() {
        assert_f32_ne!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_ne_failing_simple() {
        assert_f32_ne!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_le_passing_simple_lt() {
        assert_f32_le!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_le_passing_simple_eq() {
        assert_f32_le!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_le_failing_simple() {
        assert_f32_le!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_ge_passing_simple_gt() {
        assert_f32_ge!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_ge_passing_simple_eq() {
        assert_f32_ge!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_ge_failing_simple() {
        assert_f32_ge!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_simple_negate() {
        assert_f32_eq!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_simple_negate() {
        assert_f32_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f32_ne_passing_simple_negate() {
        assert_f32_ne!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_ne_failing_simple_negate() {
        assert_f32_ne!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_le_failing_simple_lt_negate() {
        assert_f32_le!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_le_failing_simple_eq_negate() {
        assert_f32_le!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f32_le_passing_simple_negate() {
        assert_f32_le!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_ge_failing_simple_gt_negate() {
        assert_f32_ge!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_ge_failing_simple_eq_negate() {
        assert_f32_ge!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f32_ge_passing_simple_negate() {
        assert_f32_ge!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f64_eq_passing_simple() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_simple() {
        assert_f64_eq!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_ne_passing_simple() {
        assert_f64_ne!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_ne_failing_simple() {
        assert_f64_ne!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_le_passing_simple_lt() {
        assert_f64_le!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_le_passing_simple_eq() {
        assert_f64_le!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_le_failing_simple() {
        assert_f64_le!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_ge_passing_simple_gt() {
        assert_f64_ge!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_ge_passing_simple_eq() {
        assert_f64_ge!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_ge_failing_simple() {
        assert_f64_ge!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_simple_negate() {
        assert_f64_eq!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_simple_negate() {
        assert_f64_eq!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f64_ne_passing_simple_negate() {
        assert_f64_ne!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_ne_failing_simple_negate() {
        assert_f64_ne!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_le_failing_simple_lt_negate() {
        assert_f64_le!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_le_failing_simple_eq_negate() {
        assert_f64_le!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f64_le_passing_simple_negate() {
        assert_f64_le!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_ge_failing_simple_gt_negate() {
        assert_f64_ge!(1.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_ge_failing_simple_eq_negate() {
        assert_f64_ge!(0.0, 0.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f64_ge_passing_simple_negate() {
        assert_f64_ge!(0.0, 1.0, ulps = 0, epsilon_near_zero = 0.0, negate = true);
    }

    #[test]
    fn assert_f32_eq_passing_ulps_infinity_infinity() {
        assert_f32_eq!(
            f32::INFINITY,
            f32::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_infinity_infinity() {
        assert_f32_eq!(
            f32::INFINITY,
            f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_infinity_neg_infinity() {
        assert_f32_eq!(
            f32::INFINITY,
            -f32::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_infinity_neg_infinity() {
        assert_f32_eq!(
            f32::INFINITY,
            -f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_infinity_nan() {
        assert_f32_eq!(f32::INFINITY, f32::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_infinity_nan() {
        assert_f32_eq!(
            f32::INFINITY,
            f32::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_infinity_five() {
        assert_f32_eq!(f32::INFINITY, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_infinity_five() {
        assert_f32_eq!(
            f32::INFINITY,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_neg_infinity_infinity() {
        assert_f32_eq!(
            -f32::INFINITY,
            f32::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_neg_infinity_infinity() {
        assert_f32_eq!(
            -f32::INFINITY,
            f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_neg_infinity_neg_infinity() {
        assert_f32_eq!(
            -f32::INFINITY,
            -f32::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_relative_neg_infinity_neg_infinity() {
        assert_f32_eq!(
            -f32::INFINITY,
            -f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_neg_infinity_nan() {
        assert_f32_eq!(-f32::INFINITY, f32::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_neg_infinity_nan() {
        assert_f32_eq!(
            -f32::INFINITY,
            f32::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_neg_infinity_five() {
        assert_f32_eq!(-f32::INFINITY, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_neg_infinity_five() {
        assert_f32_eq!(
            -f32::INFINITY,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_nan_infinity() {
        assert_f32_eq!(
            -f32::INFINITY,
            f32::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_nan_infinity() {
        assert_f32_eq!(
            f32::NAN,
            f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_nan_neg_infinity() {
        assert_f32_eq!(f32::NAN, -f32::INFINITY, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_nan_neg_infinity() {
        assert_f32_eq!(
            f32::NAN,
            -f32::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f32_eq_passing_ulps_nan_nan() {
        assert_f32_eq!(f32::NAN, f32::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f32_eq_passing_relative_nan_nan() {
        assert_f32_eq!(
            f32::NAN,
            f32::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_ulps_nan_five() {
        assert_f32_eq!(f32::NAN, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f32_eq_failing_relative_nan_five() {
        assert_f32_eq!(
            f32::NAN,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_infinity_infinity() {
        assert_f64_eq!(
            f64::INFINITY,
            f64::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_infinity_infinity() {
        assert_f64_eq!(
            f64::INFINITY,
            f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_infinity_neg_infinity() {
        assert_f64_eq!(
            f64::INFINITY,
            -f64::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_infinity_neg_infinity() {
        assert_f64_eq!(
            f64::INFINITY,
            -f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_infinity_nan() {
        assert_f64_eq!(f64::INFINITY, f64::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_infinity_nan() {
        assert_f64_eq!(
            f64::INFINITY,
            f64::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_infinity_five() {
        assert_f64_eq!(f64::INFINITY, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_infinity_five() {
        assert_f64_eq!(
            f64::INFINITY,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_neg_infinity_infinity() {
        assert_f64_eq!(
            -f64::INFINITY,
            f64::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_neg_infinity_infinity() {
        assert_f64_eq!(
            -f64::INFINITY,
            f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_neg_infinity_neg_infinity() {
        assert_f64_eq!(
            -f64::INFINITY,
            -f64::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_relative_neg_infinity_neg_infinity() {
        assert_f64_eq!(
            -f64::INFINITY,
            -f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_neg_infinity_nan() {
        assert_f64_eq!(-f64::INFINITY, f64::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_neg_infinity_nan() {
        assert_f64_eq!(
            -f64::INFINITY,
            f64::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_neg_infinity_five() {
        assert_f64_eq!(-f64::INFINITY, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_neg_infinity_five() {
        assert_f64_eq!(
            -f64::INFINITY,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_nan_infinity() {
        assert_f64_eq!(
            -f64::INFINITY,
            f64::INFINITY,
            ulps = 0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_nan_infinity() {
        assert_f64_eq!(
            f64::NAN,
            f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_nan_neg_infinity() {
        assert_f64_eq!(f64::NAN, -f64::INFINITY, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_nan_neg_infinity() {
        assert_f64_eq!(
            f64::NAN,
            -f64::INFINITY,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    fn assert_f64_eq_passing_ulps_nan_nan() {
        assert_f64_eq!(f64::NAN, f64::NAN, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    fn assert_f64_eq_passing_relative_nan_nan() {
        assert_f64_eq!(
            f64::NAN,
            f64::NAN,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_ulps_nan_five() {
        assert_f64_eq!(f64::NAN, 5.0, ulps = 0, epsilon_near_zero = 0.0);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn assert_f64_eq_failing_relative_nan_five() {
        assert_f64_eq!(
            f64::NAN,
            5.0,
            relative_epsilon = 0.0,
            epsilon_near_zero = 0.0
        );
    }
}
