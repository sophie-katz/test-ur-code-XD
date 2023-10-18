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

//! Assertions that operate on strings.
//!
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
//! for a usage guide.

#[cfg(feature = "regex")]
use regex::Regex;

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
pub fn assert_str_eq_impl(lhs: impl AsRef<str>, rhs: impl AsRef<str>) -> bool {
    lhs.as_ref().eq(rhs.as_ref())
}

/// Asserts that one string is equal to another and prints a diff if they are not.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
/// for a usage guide.
///
/// # Arguments
///
/// * `lhs` - The left-hand side string.
/// * `rhs` - The right-hand side string.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_str_eq;
/// #
/// assert_str_eq!("hello, world", "hello, world");
///
/// assert_str_eq!("hello, world", "hello! world", negate = true);
/// ```
#[cfg(feature = "string-diff")]
#[macro_export]
macro_rules! assert_str_eq {
    ($lhs:expr, $rhs:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "lhs == rhs",
            $crate::assertions::string::assert_str_eq_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &::std::convert::AsRef::<str>::as_ref(&$lhs))
                    .with_argument("rhs", stringify!($rhs), &::std::convert::AsRef::<str>::as_ref(&$rhs))
                    .with_argument_formatted("diff", "--",
                        $crate::utilities::diff::format_diff(
                            &$lhs,
                            &$rhs,
                            $crate::utilities::panic_message_builder::DEBUGGED_VALUE_PREFIX.len()
                        )
                    )
            }
            $(, $keys = $values)*
        )
    };
}

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
pub fn assert_str_contains_impl(value: impl AsRef<str>, substring: impl AsRef<str>) -> bool {
    value.as_ref().contains(substring.as_ref())
}

/// Asserts that a string contains a substring.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The string to check.
/// * `substring` - The substring for which to check.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_str_contains;
/// #
/// assert_str_contains!("hello, world", "hello");
///
/// assert_str_contains!("hello, world", "asdf", negate = true);
/// ```
#[macro_export]
macro_rules! assert_str_contains {
    ($value:expr, $substring:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value contains substring",
            $crate::assertions::string::assert_str_contains_impl(&$value, &$substring),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &::std::convert::AsRef::<str>::as_ref(&$value))
                    .with_argument("substring", stringify!($substring), &::std::convert::AsRef::<str>::as_ref(&$substring))
            }
            $(, $keys = $values)*
        )
    };
}

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
pub fn assert_str_starts_with_impl(value: impl AsRef<str>, prefix: impl AsRef<str>) -> bool {
    value.as_ref().starts_with(prefix.as_ref())
}

/// Asserts that a string starts with a prefix.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The string to check.
/// * `prefix` - The prefix for which to check.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_str_starts_with;
/// #
/// assert_str_starts_with!("hello, world", "hello");
///
/// assert_str_starts_with!("hello, world", "world", negate = true);
/// ```
#[macro_export]
macro_rules! assert_str_starts_with {
    ($value:expr, $prefix:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value starts with prefix",
            $crate::assertions::string::assert_str_starts_with_impl(&$value, &$prefix),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &::std::convert::AsRef::<str>::as_ref(&$value))
                    .with_argument("prefix", stringify!($prefix), &::std::convert::AsRef::<str>::as_ref(&$prefix))
            }
            $(, $keys = $values)*
        )
    };
}

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
pub fn assert_str_ends_with_impl(value: impl AsRef<str>, suffix: impl AsRef<str>) -> bool {
    value.as_ref().ends_with(suffix.as_ref())
}

/// Asserts that a string ends with a suffix.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The string to check.
/// * `suffix` - The suffix for which to check.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_str_ends_with;
/// #
/// assert_str_ends_with!("hello, world", "world");
///
/// assert_str_ends_with!("hello, world", "hello", negate = true);
/// ```
#[macro_export]
macro_rules! assert_str_ends_with {
    ($value:expr, $suffix:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value ends with suffix",
            $crate::assertions::string::assert_str_ends_with_impl(&$value, &$suffix),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &::std::convert::AsRef::<str>::as_ref(&$value))
                    .with_argument("suffix", stringify!($suffix), &::std::convert::AsRef::<str>::as_ref(&$suffix))
            }
            $(, $keys = $values)*
        )
    };
}

// Assertion implementations need to be public for the macros to use them, but should not appear in
// documentation.
#[doc(hidden)]
#[cfg(feature = "regex")]
pub fn assert_str_matches_impl(value: impl AsRef<str>, pattern: impl AsRef<str>) -> bool {
    use std::panic::Location;

    use crate::utilities::panic_message_builder::PanicMessageBuilder;

    let pattern = match Regex::new(pattern.as_ref()) {
        Ok(pattern_value) => pattern_value,
        Err(error) => PanicMessageBuilder::new("error while parsing regex", Location::caller())
            .with_argument("error", "--", &error.to_string())
            .panic(),
    };

    pattern.is_match(value.as_ref())
}

/// Asserts that a string matches a regular expression.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/string](https://sophie-katz.github.io/test-ur-code-XD/assertions/string/)
/// for a usage guide.
///
/// # Arguments
///
/// * `value` - The string to check.
/// * `pattern` - The pattern for which to check.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_str_matches;
/// #
/// assert_str_matches!("hello, world", "[a-z]+");
///
/// assert_str_matches!("hello, world", "^[a-z]+$", negate = true);
/// ```
#[cfg(feature = "regex")]
#[macro_export]
macro_rules! assert_str_matches {
    ($value:expr, $pattern:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value matches pattern",
            $crate::assertions::string::assert_str_matches_impl(&$value, &$pattern),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &::std::convert::AsRef::<str>::as_ref(&$value))
                    .with_argument("pattern", stringify!($pattern), &::std::convert::AsRef::<str>::as_ref(&$pattern))
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "string-diff")]
    #[test]
    fn assert_str_eq_passing() {
        assert_str_eq!("hello, world", "hello, world");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn assert_str_eq_passing_empty() {
        assert_str_eq!("", "");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_empty_some() {
        assert_str_eq!("", "asdf");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_some_empty() {
        assert_str_eq!("asdf", "");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_totally_different() {
        assert_str_eq!("hello, world", "asdf");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_slightly_different() {
        assert_str_eq!("hello, world", "hello! world");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_long() {
        assert_str_eq!("a".repeat(100), "b".repeat(100));
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_str_eq_failing_multiline() {
        assert_str_eq!("asdf\nasdf", "asdf\nfdsa");
    }

    #[test]
    fn assert_str_contains_passing() {
        assert_str_contains!("hello, world", "hello");
    }

    #[test]
    fn assert_str_contains_passing_empty() {
        assert_str_contains!("hello, world", "");
    }

    #[test]
    fn assert_str_contains_passing_empty_both_empty() {
        assert_str_contains!("", "");
    }

    #[test]
    #[should_panic(expected = "value contains substring")]
    fn assert_str_contains_failing() {
        assert_str_contains!("hello, world", "asdf");
    }

    #[test]
    #[should_panic(expected = "value contains substring")]
    fn assert_str_contains_failing_empty() {
        assert_str_contains!("", "asdf");
    }

    #[test]
    fn assert_str_starts_with_passing() {
        assert_str_starts_with!("hello, world", "hello");
    }

    #[test]
    fn assert_str_starts_with_passing_empty() {
        assert_str_starts_with!("hello, world", "");
    }

    #[test]
    fn assert_str_starts_with_passing_both_empty() {
        assert_str_starts_with!("", "");
    }

    #[test]
    #[should_panic(expected = "value starts with prefix")]
    fn assert_str_starts_with_failing() {
        assert_str_starts_with!("hello, world", "world");
    }

    #[test]
    #[should_panic(expected = "value starts with prefix")]
    fn assert_str_starts_with_failing_empty() {
        assert_str_starts_with!("", "hello");
    }

    #[test]
    fn assert_str_ends_with_passing() {
        assert_str_ends_with!("hello, world", "world");
    }

    #[test]
    fn assert_str_ends_with_passing_empty() {
        assert_str_ends_with!("hello, world", "");
    }

    #[test]
    fn assert_str_ends_with_passing_both_empty() {
        assert_str_ends_with!("", "");
    }

    #[test]
    #[should_panic(expected = "value ends with suffix")]
    fn assert_str_ends_with_failing() {
        assert_str_ends_with!("hello, world", "hello");
    }

    #[test]
    #[should_panic(expected = "value ends with suffix")]
    fn assert_str_ends_with_failing_empty() {
        assert_str_ends_with!("", "hello");
    }

    #[cfg(feature = "regex")]
    #[test]
    fn assert_str_matches_passing() {
        assert_str_matches!("hello, world", "[a-z, ]+");
    }

    #[cfg(feature = "regex")]
    #[test]
    #[should_panic(expected = "value matches pattern")]
    fn assert_str_matches_failing_partial() {
        assert_str_matches!("hello, world", "[A-Z]+");
    }

    #[cfg(feature = "regex")]
    #[test]
    #[should_panic(expected = "value matches pattern")]
    fn assert_str_matches_failing_whole() {
        assert_str_matches!("hello, world", "^[a-z]+$");
    }

    #[cfg(feature = "regex")]
    #[test]
    #[should_panic(expected = "error while parsing regex")]
    fn assert_str_matches_failing_bad_regex() {
        assert_str_matches!("hello, world", "[a-z, ");
    }
}
