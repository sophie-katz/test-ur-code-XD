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
//!
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
//! for a usage guide.

#[doc(hidden)]
pub fn assert_eq_impl<LhsType: PartialEq<RhsType>, RhsType>(lhs: &LhsType, rhs: &RhsType) -> bool {
    lhs.eq(rhs)
}

/// Asserts that two values are equal to each other using the [`PartialEq`] trait.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_eq_impl(&$lhs, &$rhs),
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

/// Asserts that two values are unequal to each other using the [`PartialEq`] trait.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_ne_impl(&$lhs, &$rhs),
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_lt_impl(&$lhs, &$rhs),
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_le_impl(&$lhs, &$rhs),
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_gt_impl(&$lhs, &$rhs),
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
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic](https://sophie-katz.github.io/test-ur-code-XD/assertions/arithmetic/)
/// for a usage guide.
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
            $crate::assertions::arithmetic::assert_ge_impl(&$lhs, &$rhs),
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
    #[derive(Debug, PartialEq, PartialOrd)]
    struct NoDefaultTraitsI32 {
        value: i32,
    }

    #[test]
    fn assert_eq_passing_bool() {
        assert_eq!(true, true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_bool() {
        assert_eq!(true, false);
    }

    #[test]
    fn assert_eq_passing_bool_negate() {
        assert_eq!(false, true, negate = true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_bool_negate() {
        assert_eq!(false, false, negate = true);
    }

    #[test]
    fn assert_eq_passing_i32() {
        assert_eq!(0, 0);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_i32() {
        assert_eq!(3, 4);
    }

    #[test]
    fn assert_eq_passing_i32_negate() {
        assert_eq!(6, -3, negate = true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_i32_negate() {
        assert_eq!(3, 3, negate = true);
    }

    #[test]
    fn assert_eq_passing_f64() {
        assert_eq!(0.3, 0.3);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_f64() {
        assert_eq!(1.4, 1.5);
    }

    #[test]
    fn assert_eq_passing_f64_negate() {
        assert_eq!(6.0, -3.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_f64_negate() {
        assert_eq!(3.0, 3.0, negate = true);
    }

    #[test]
    fn assert_eq_passing_string() {
        assert_eq!("hi".to_owned(), "hi".to_owned());
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_string() {
        assert_eq!("hi".to_owned(), "bye".to_owned());
    }

    #[test]
    fn assert_eq_passing_string_negate() {
        assert_eq!("hii".to_owned(), "hi".to_owned(), negate = true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_string_negate() {
        assert_eq!(String::new(), String::new(), negate = true);
    }

    #[test]
    fn assert_eq_passing_vec() {
        assert_eq!(vec![1, 2], vec![1, 2]);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_vector() {
        assert_eq!(vec![1, 2], vec![3, 4]);
    }

    #[test]
    fn assert_eq_passing_vec_negate() {
        assert_eq!(vec![1, 2], vec![1, 2, 3], negate = true);
    }

    #[test]
    #[should_panic = "lhs == rhs"]
    fn assert_eq_failing_vector_negate() {
        assert_eq!(Vec::<i32>::new(), Vec::<i32>::new(), negate = true);
    }

    #[test]
    fn assert_ne_passing_bool() {
        assert_ne!(true, false);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_bool() {
        assert_ne!(true, true);
    }

    #[test]
    fn assert_ne_passing_bool_negate() {
        assert_ne!(false, false, negate = true);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_bool_negate() {
        assert_ne!(false, true, negate = true);
    }

    #[test]
    fn assert_ne_passing_i32() {
        assert_ne!(0, 1);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_i32() {
        assert_ne!(3, 3);
    }

    #[test]
    fn assert_ne_passing_i32_negate() {
        assert_ne!(6, 6, negate = true);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_i32_negate() {
        assert_ne!(3, -3, negate = true);
    }

    #[test]
    fn assert_ne_passing_f64() {
        assert_ne!(0.3, 0.4);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_f64() {
        assert_ne!(1.4, 1.4);
    }

    #[test]
    fn assert_ne_passing_f64_negate() {
        assert_ne!(6.0, 6.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_f64_negate() {
        assert_ne!(3.0, -3.0, negate = true);
    }

    #[test]
    fn assert_ne_passing_string() {
        assert_ne!("hi".to_owned(), "bye".to_owned());
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_string() {
        assert_ne!("hi".to_owned(), "hi".to_owned());
    }

    #[test]
    fn assert_ne_passing_string_negate() {
        assert_ne!("hii".to_owned(), "hii".to_owned(), negate = true);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_string_negate() {
        assert_ne!(String::new(), "a".to_owned(), negate = true);
    }

    #[test]
    fn assert_ne_passing_vec() {
        assert_ne!(vec![1, 2], vec![1, 3]);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_vec() {
        assert_ne!(vec![1, 2], vec![1, 2]);
    }

    #[test]
    fn assert_ne_passing_vec_negate() {
        assert_ne!(vec![1, 2], vec![1, 2], negate = true);
    }

    #[test]
    #[should_panic = "lhs != rhs"]
    fn assert_ne_failing_vec_negate() {
        assert_ne!(Vec::<i32>::new(), vec![1], negate = true);
    }

    #[test]
    fn assert_lt_passing_i32() {
        assert_lt!(0, 1);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_i32_eq() {
        assert_lt!(3, 3);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_i32_gt() {
        assert_lt!(-3, -4);
    }

    #[test]
    fn assert_lt_passing_i32_negate() {
        assert_lt!(6, 3, negate = true);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_i32_negate() {
        assert_lt!(3, 4, negate = true);
    }

    #[test]
    fn assert_lt_passing_f64() {
        assert_lt!(0.3, 0.31);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_f64_eq() {
        assert_lt!(1.4, 1.4);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_f64_gt() {
        assert_lt!(1.41, 1.4);
    }

    #[test]
    fn assert_lt_passing_f64_negate() {
        assert_lt!(6.0, -3.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_f64_negate() {
        assert_lt!(3.0, 4.0, negate = true);
    }

    #[test]
    fn assert_lt_passing_no_default_traits_i32() {
        assert_lt!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 1 }
        );
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_no_default_traits_i32_eq() {
        assert_lt!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_no_default_traits_i32_gt() {
        assert_lt!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    fn assert_lt_passing_no_default_traits_i32_negate() {
        assert_lt!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 0 },
            negate = true
        );
    }

    #[test]
    #[should_panic = "lhs < rhs"]
    fn assert_lt_failing_no_default_traits_i32_negate() {
        assert_lt!(
            NoDefaultTraitsI32 { value: 99 },
            NoDefaultTraitsI32 { value: 100 },
            negate = true
        );
    }

    #[test]
    fn assert_le_passing_i32_lt() {
        assert_le!(0, 1);
    }

    #[test]
    fn assert_le_passing_i32_eq() {
        assert_le!(1, 1);
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_i32() {
        assert_le!(-3, -4);
    }

    #[test]
    fn assert_le_passing_i32_negate() {
        assert_le!(6, 3, negate = true);
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_i32_negate() {
        assert_le!(3, 4, negate = true);
    }

    #[test]
    fn assert_le_passing_f64_lt() {
        assert_le!(0.3, 0.31);
    }

    #[test]
    fn assert_le_passing_f64_eq() {
        assert_le!(0.3, 0.3);
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_f64() {
        assert_le!(1.41, 1.4);
    }

    #[test]
    fn assert_le_passing_f64_negate() {
        assert_le!(6.0, -3.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_f64_negate() {
        assert_le!(3.0, 4.0, negate = true);
    }

    #[test]
    fn assert_le_passing_no_default_traits_i32_lt() {
        assert_le!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 1 }
        );
    }

    #[test]
    fn assert_le_passing_no_default_traits_i32_eq() {
        assert_le!(
            NoDefaultTraitsI32 { value: 1 },
            NoDefaultTraitsI32 { value: 1 }
        );
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_no_default_traits_i32() {
        assert_le!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    fn assert_le_passing_no_default_traits_i32_negate() {
        assert_le!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 0 },
            negate = true
        );
    }

    #[test]
    #[should_panic = "lhs <= rhs"]
    fn assert_le_failing_no_default_traits_i32_negate() {
        assert_le!(
            NoDefaultTraitsI32 { value: 99 },
            NoDefaultTraitsI32 { value: 100 },
            negate = true
        );
    }

    #[test]
    fn assert_gt_passing_i32() {
        assert_gt!(1, 0);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_i32_eq() {
        assert_gt!(3, 3);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_i32_lt() {
        assert_gt!(-4, -3);
    }

    #[test]
    fn assert_gt_passing_i32_negate() {
        assert_gt!(3, 6, negate = true);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_i32_negate() {
        assert_gt!(4, 3, negate = true);
    }

    #[test]
    fn assert_gt_passing_f64() {
        assert_gt!(0.31, 0.3);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_f64_eq() {
        assert_gt!(1.4, 1.4);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_f64_lt() {
        assert_gt!(1.4, 1.41);
    }

    #[test]
    fn assert_gt_passing_f64_negate() {
        assert_gt!(-3.0, 6.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_f64_negate() {
        assert_gt!(4.0, 3.0, negate = true);
    }

    #[test]
    fn assert_gt_passing_no_default_traits_i32() {
        assert_gt!(
            NoDefaultTraitsI32 { value: 1 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_no_default_traits_i32_eq() {
        assert_gt!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_no_default_traits_i32_lt() {
        assert_gt!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 100 }
        );
    }

    #[test]
    fn assert_gt_passing_no_default_traits_i32_negate() {
        assert_gt!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 100 },
            negate = true
        );
    }

    #[test]
    #[should_panic = "lhs > rhs"]
    fn assert_gt_failing_no_default_traits_i32_negate() {
        assert_gt!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 99 },
            negate = true
        );
    }

    #[test]
    fn assert_ge_passing_i32_gt() {
        assert_ge!(1, 0);
    }

    #[test]
    fn assert_ge_passing_i32_eq() {
        assert_ge!(1, 1);
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_i32() {
        assert_ge!(-4, -3);
    }

    #[test]
    fn assert_ge_passing_i32_negate() {
        assert_ge!(3, 6, negate = true);
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_i32_negate() {
        assert_ge!(4, 3, negate = true);
    }

    #[test]
    fn assert_ge_passing_f64_gt() {
        assert_ge!(0.31, 0.3);
    }

    #[test]
    fn assert_ge_passing_f64_eq() {
        assert_ge!(0.3, 0.3);
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_f64() {
        assert_ge!(1.4, 1.41);
    }

    #[test]
    fn assert_ge_passing_f64_negate() {
        assert_ge!(-3.0, 6.0, negate = true);
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_f64_negate() {
        assert_ge!(4.0, 3.0, negate = true);
    }

    #[test]
    fn assert_ge_passing_no_default_traits_i32_gt() {
        assert_ge!(
            NoDefaultTraitsI32 { value: 1 },
            NoDefaultTraitsI32 { value: 0 }
        );
    }

    #[test]
    fn assert_ge_passing_no_default_traits_i32_eq() {
        assert_ge!(
            NoDefaultTraitsI32 { value: 1 },
            NoDefaultTraitsI32 { value: 1 }
        );
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_no_default_traits_i32() {
        assert_ge!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 100 }
        );
    }

    #[test]
    fn assert_ge_passing_no_default_traits_i32_negate() {
        assert_ge!(
            NoDefaultTraitsI32 { value: 0 },
            NoDefaultTraitsI32 { value: 100 },
            negate = true
        );
    }

    #[test]
    #[should_panic = "lhs >= rhs"]
    fn assert_ge_failing_no_default_traits_i32_negate() {
        assert_ge!(
            NoDefaultTraitsI32 { value: 100 },
            NoDefaultTraitsI32 { value: 99 },
            negate = true
        );
    }
}
