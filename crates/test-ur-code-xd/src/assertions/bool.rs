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

//! Assertions that operate on booleans.
//!
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/boolean](https://sophie-katz.github.io/test-ur-code-XD/assertions/boolean/)
//! for a usage guide.

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
#[must_use]
pub fn assert_impl(value: bool) -> bool {
    value
}

/// Asserts that the boolean value is `true`.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/boolean](https://sophie-katz.github.io/test-ur-code-XD/assertions/boolean/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The `bool` value to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert;
/// #
/// assert!(true);
/// ```
#[macro_export]
macro_rules! assert {
    ($value:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value is true",
            $crate::assertions::bool::assert_impl($value),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
            }
            $(, $keys = $values)*
        )
    };
}

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
#[must_use]
pub fn assert_not_impl(value: bool) -> bool {
    !value
}

/// Asserts that the boolean value is `false`.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/boolean](https://sophie-katz.github.io/test-ur-code-XD/assertions/boolean/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The `bool` value to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_not;
/// #
/// assert_not!(false);
/// ```
#[macro_export]
macro_rules! assert_not {
    ($value:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value is false",
            $crate::assertions::bool::assert_not_impl($value),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_passing() {
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "value is true")]
    fn assert_failing() {
        assert!(false);
    }

    #[test]
    fn assert_passing_negate() {
        assert!(false, negate = true);
    }

    #[test]
    #[should_panic(expected = "value is true")]
    fn assert_failing_negate() {
        assert!(true, negate = true);
    }

    #[test]
    fn assert_not_passing() {
        assert_not!(false);
    }

    #[test]
    #[should_panic(expected = "value is false")]
    fn assert_not_failing() {
        assert_not!(true);
    }

    #[test]
    fn assert_not_passing_negate() {
        assert_not!(true, negate = true);
    }

    #[test]
    #[should_panic(expected = "value is false")]
    fn assert_not_failing_negate() {
        assert_not!(false, negate = true);
    }
}
