// Copyright (c) 2023 Sophie Katz
//
// This file is part of test-ur-code-XD.
//
// test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
// not, see <https://www.gnu.org/licenses/>.

// TODO: Add this to documentation
// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/

use float_cmp::{approx_eq, Ulps};
use num_traits::Float;
use std::fmt::{Debug, Display};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

fn is_float_eq_non_finite<FloatType: Float>(lhs: FloatType, rhs: FloatType) -> Option<bool> {
    if lhs.is_infinite() != rhs.is_infinite() {
        // One is infinite and the other is not: inequal
        return Some(false);
    } else if lhs.is_infinite() {
        assert!(rhs.is_infinite());
        // Both are infinite: equal
        return Some(true);
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

pub fn is_float_eq_relative<FloatType: Float>(
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

pub fn is_float_eq_ulps_f32(
    lhs: f32,
    rhs: f32,
    absolute_tolerance: f32,
    ulps_tolerance: i32,
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

pub fn is_float_eq_ulps_f64(
    lhs: f64,
    rhs: f64,
    absolute_tolerance: f64,
    ulps_tolerance: i64,
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
        "lhs {} rhs (within {} {}-bit float ulp{} or {} near zero)",
        operator,
        ulps_tolerance,
        bit_width,
        if ulps_tolerance == 1 { "" } else { "s" },
        epsilon_near_zero
    )
}

pub fn format_float_predicate_description_relative<FloatType: Display>(
    operator: &str,
    relative_epsilon: FloatType,
    epsilon_near_zero: FloatType,
) -> String {
    format!(
        "lhs {} rhs (within {} relative to magnitude or {} near zero)",
        operator, relative_epsilon, epsilon_near_zero
    )
}

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

pub fn configure_float_panic_message<FloatType: Float + Debug>(
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

pub fn assert_f32_eq_impl_ulps(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    ulps_tolerance: i32,
) -> bool {
    is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps_tolerance)
}

pub fn assert_f32_eq_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f32_eq {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "==",
                $ulps_tolerance,
                32,
                $epsilon_near_zero,
            ),
            $crate::assertions::float_assertions::assert_f32_eq_impl_ulps(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $ulps_tolerance
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f32_ne_impl_ulps(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    ulps_tolerance: i32,
) -> bool {
    !is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps_tolerance)
}

pub fn assert_f32_ne_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f32_ne {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "!=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::assert_f32_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f32_le_impl_ulps(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    ulps_tolerance: i32,
) -> bool {
    lhs <= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps_tolerance)
}

pub fn assert_f32_le_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    lhs <= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f32_le {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "<=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f32_ge_impl_ulps(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    ulps_tolerance: i32,
) -> bool {
    lhs >= rhs || is_float_eq_ulps_f32(lhs, rhs, epsilon_near_zero, ulps_tolerance)
}

pub fn assert_f32_ge_impl_relative(
    lhs: f32,
    rhs: f32,
    epsilon_near_zero: f32,
    relative_epsilon: f32,
) -> bool {
    lhs >= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f32_ge {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                ">=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f64_eq_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

pub fn assert_f64_eq_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f64_eq {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "==",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f64_ne_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    !is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

pub fn assert_f64_ne_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    !is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f64_ne {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "!=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_relative(
                "!=",
                $relative_epsilon,
                $epsilon_near_zero,
            ),
            $crate::assertions::assert_f64_ne_impl_relative(
                $lhs,
                $rhs,
                $epsilon_near_zero,
                $relative_epsilon
            ),
            |panic_message_builder| {
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f64_le_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs <= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

pub fn assert_f64_le_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    lhs <= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f64_le {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                "<=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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

pub fn assert_f64_ge_impl_ulps(lhs: f64, rhs: f64, epsilon_near_zero: f64, ulps: i64) -> bool {
    lhs >= rhs || is_float_eq_ulps_f64(lhs, rhs, epsilon_near_zero, ulps)
}

pub fn assert_f64_ge_impl_relative(
    lhs: f64,
    rhs: f64,
    epsilon_near_zero: f64,
    relative_epsilon: f64,
) -> bool {
    lhs >= rhs || is_float_eq_relative(lhs, rhs, epsilon_near_zero, relative_epsilon)
}

#[macro_export]
macro_rules! assert_f64_ge {
    (
        $lhs:expr,
        $rhs:expr,
        ulps_tolerance = $ulps_tolerance:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assert_custom!(
            $crate::assertions::float_assertions::format_float_predicate_description_ulps(
                ">=",
                $ulps_tolerance,
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
        relative_epsilon = $ulps:expr,
        epsilon_near_zero = $epsilon_near_zero:expr,
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
                $crate::assertions::float_assertions::configure_float_panic_message(
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
    use crate::utilities::capture_output::capture_output;

    #[test]
    fn assert_f32_eq_passing() {
        let captured_outputs = capture_output(|| {
            assert_f32_eq!(1.0, 1.0, ulps_tolerance = 0, epsilon_near_zero = 0.0);
            assert_f32_eq!(
                0.15 + 0.15 + 0.15,
                0.1 + 0.1 + 0.25,
                ulps_tolerance = 1,
                epsilon_near_zero = 0.0
            );
            assert_f32_eq!(
                0.15 + 0.15 + 0.15,
                0.1 + 0.1 + 0.25,
                ulps_tolerance = 0,
                epsilon_near_zero = f32::EPSILON
            );
        })
        .unwrap();

        std::assert!(captured_outputs.stdout.is_empty());
        std::assert!(captured_outputs.stderr.is_empty());
    }
}
