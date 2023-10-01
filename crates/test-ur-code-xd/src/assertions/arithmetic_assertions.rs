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

//! Assertions that use arithmetic comparisons.

#[doc(hidden)]
pub fn assert_eq_impl<LhsType: PartialEq<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.eq(rhs)
}

/// Asserts that two values are equal to each other using the [`PartialEq`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_eq;
/// #
/// # let x = 5;
/// #
/// assert_eq!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_eq {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs == rhs",
            $crate::assertions::arithmetic_assertions::assert_eq_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_ne_impl<LhsType: PartialEq<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    !lhs.eq(rhs)
}

/// Asserts that two values are inequal to each other using the [`PartialEq`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_ne;
/// #
/// # let x = 4;
/// #
/// assert_ne!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_ne {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs != rhs",
            $crate::assertions::arithmetic_assertions::assert_ne_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_lt_impl<LhsType: PartialOrd<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.lt(rhs)
}

/// Asserts that one value is less than the other using the [`PartialOrd`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_lt;
/// #
/// # let x = 4;
/// #
/// assert_lt!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_lt {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs < rhs",
            $crate::assertions::arithmetic_assertions::assert_lt_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_le_impl<LhsType: PartialOrd<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.le(rhs)
}

/// Asserts that one value is less than or equal to the other using the [`PartialOrd`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_le;
/// #
/// # let x = 5;
/// #
/// assert_le!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_le {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs <= rhs",
            $crate::assertions::arithmetic_assertions::assert_le_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_gt_impl<LhsType: PartialOrd<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.gt(rhs)
}

/// Asserts that one value is greater than the other using the [`PartialOrd`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_gt;
/// #
/// # let x = 6;
/// #
/// assert_gt!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_gt {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs > rhs",
            $crate::assertions::arithmetic_assertions::assert_gt_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_ge_impl<LhsType: PartialOrd<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.ge(rhs)
}

/// Asserts that one value is greater than or equal to the other using the [`PartialOrd`] trait.
///
/// # Arguments
///
/// * `lhs` - The value on the left-hand side.
/// * `rhs` - The value on the right-hand side.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_ge;
/// #
/// # let x = 5;
/// #
/// assert_ge!(x, 5);
/// ```
#[macro_export]
macro_rules! assert_ge {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs >= rhs",
            $crate::assertions::arithmetic_assertions::assert_ge_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    // use crate::utilities::capture_output::capture_output;

    #[test]
    fn assert_eq_passing() {
        // let captured_outputs = capture_output(|| {
        assert_eq!(true, true);
        assert_eq!(true, false, negate = true);
        assert_eq!(1, 1);
        assert_eq!(1, 2, negate = true);
        assert_eq!(3.2, 3.2);
        assert_eq!(3.2, 3.3, negate = true);
        assert_eq!("hi", "hi");
        assert_eq!("hi", "bye", negate = true);
        assert_eq!("hi".to_owned(), "hi".to_owned());
        assert_eq!("hi".to_owned(), "bye".to_owned(), negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_bool() {
        assert_eq!(true, false);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_bool_negated() {
        assert_eq!(true, true, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_i32() {
        assert_eq!(1, 2);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_i32_negated() {
        assert_eq!(1, 1, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_f64() {
        assert_eq!(3.2, 3.3);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_f64_negate() {
        assert_eq!(3.2, 3.2, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_str() {
        assert_eq!("hi", "bye");
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_str_negate() {
        assert_eq!("hi", "hi", negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_string() {
        assert_eq!("hi".to_owned(), "bye".to_owned());
    }

    #[test]
    #[should_panic]
    fn assert_eq_fail_string_negate() {
        assert_eq!("hi".to_owned(), "hi".to_owned(), negate = true);
    }

    #[test]
    fn assert_ne_passing() {
        // let captured_outputs = capture_output(|| {
        assert_ne!(true, false);
        assert_ne!(true, true, negate = true);
        assert_ne!(1, 2);
        assert_ne!(1, 1, negate = true);
        assert_ne!(3.2, 3.3);
        assert_ne!(3.2, 3.2, negate = true);
        assert_ne!("hi", "bye");
        assert_ne!("hi", "hi", negate = true);
        assert_ne!("hi".to_owned(), "bye".to_owned());
        assert_ne!("hi".to_owned(), "hi".to_owned(), negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_bool() {
        assert_ne!(true, true);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_bool_negated() {
        assert_ne!(true, false, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_i32() {
        assert_ne!(1, 1);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_i32_negated() {
        assert_ne!(1, 2, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_f64() {
        assert_ne!(3.2, 3.2);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_f64_negate() {
        assert_ne!(3.2, 3.3, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_str() {
        assert_ne!("hi", "hi");
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_str_negate() {
        assert_ne!("hi", "bye", negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_string() {
        assert_ne!("hi".to_owned(), "hi".to_owned());
    }

    #[test]
    #[should_panic]
    fn assert_ne_fail_string_negate() {
        assert_ne!("hi".to_owned(), "bye".to_owned(), negate = true);
    }

    #[test]
    fn assert_lt_passing() {
        // let captured_outputs = capture_output(|| {
        assert_lt!(1, 2);
        assert_lt!(1, 1, negate = true);
        assert_lt!(3.2, 3.3);
        assert_lt!(3.2, 3.2, negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_i32_equal() {
        assert_lt!(1, 1);
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_i32_greater_than() {
        assert_lt!(1, 0);
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_i32_negated() {
        assert_lt!(1, 2, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_f64_equal() {
        assert_lt!(3.2, 3.2);
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_f64_greater_than() {
        assert_lt!(3.3, 3.2);
    }

    #[test]
    #[should_panic]
    fn assert_lt_fail_f64_negated() {
        assert_lt!(3.2, 3.3, negate = true);
    }

    #[test]
    fn assert_le_passing() {
        // let captured_outputs = capture_output(|| {
        assert_le!(1, 2);
        assert_le!(1, 1);
        assert_le!(1, 0, negate = true);
        assert_le!(3.2, 3.3);
        assert_le!(3.2, 3.2);
        assert_le!(3.2, 3.1, negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_i32_greater_than() {
        assert_le!(1, 0);
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_i32_negated_less_than() {
        assert_le!(1, 2, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_i32_negated_equal_to() {
        assert_le!(1, 1, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_f64_greater_than() {
        assert_le!(3.3, 3.2);
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_f64_negated_less_than() {
        assert_le!(3.2, 3.3, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_le_fail_f64_negated_equal_to() {
        assert_le!(3.2, 3.2, negate = true);
    }

    #[test]
    fn assert_gt_passing() {
        // let captured_outputs = capture_output(|| {
        assert_gt!(2, 1);
        assert_gt!(1, 1, negate = true);
        assert_gt!(3.3, 3.2);
        assert_gt!(3.2, 3.2, negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_i32_equal() {
        assert_gt!(1, 1);
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_i32_less_than() {
        assert_gt!(1, 2);
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_i32_negated() {
        assert_gt!(1, 0, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_f64_equal() {
        assert_gt!(3.2, 3.2);
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_f64_less_than() {
        assert_gt!(3.2, 3.3);
    }

    #[test]
    #[should_panic]
    fn assert_gt_fail_f64_negated() {
        assert_gt!(3.3, 3.2, negate = true);
    }

    #[test]
    fn assert_ge_passing() {
        // let captured_outputs = capture_output(|| {
        assert_ge!(1, 0);
        assert_ge!(1, 1);
        assert_ge!(1, 2, negate = true);
        assert_ge!(3.3, 3.2);
        assert_ge!(3.2, 3.2);
        assert_ge!(3.2, 3.3, negate = true);
        // })
        // .unwrap();

        // std::assert!(
        //     captured_outputs.stdout.is_empty(),
        //     "STDOUT: <<<{}>>>",
        //     captured_outputs.stdout
        // );
        // std::assert!(
        //     captured_outputs.stderr.is_empty(),
        //     "STDERR: <<<{}>>>",
        //     captured_outputs.stdout
        // );
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_i32_less_than() {
        assert_ge!(1, 2);
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_i32_negated_greater_than() {
        assert_ge!(1, 0, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_i32_negated_equal_to() {
        assert_ge!(1, 1, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_f64_less_than() {
        assert_ge!(3.2, 3.3);
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_f64_negated_less_than() {
        assert_ge!(3.3, 3.2, negate = true);
    }

    #[test]
    #[should_panic]
    fn assert_ge_fail_f64_negated_equal_to() {
        assert_ge!(3.2, 3.2, negate = true);
    }
}
