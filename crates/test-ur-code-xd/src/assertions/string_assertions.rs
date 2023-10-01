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

use regex::Regex;

#[doc(hidden)]
pub fn assert_str_contains_impl(value: impl AsRef<str>, substring: impl AsRef<str>) -> bool {
    value.as_ref().contains(substring.as_ref())
}

/// Asserts that a string contains a substring.
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
            $crate::assertions::string_assertions::assert_str_contains_impl(&$value, &$substring),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
                    .with_argument("substring", stringify!($substring), &$substring)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_str_starts_with_impl(value: impl AsRef<str>, prefix: impl AsRef<str>) -> bool {
    value.as_ref().starts_with(prefix.as_ref())
}

/// Asserts that a string starts with a prefix.
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
            $crate::assertions::string_assertions::assert_str_starts_with_impl(&$value, &$prefix),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
                    .with_argument("prefix", stringify!($prefix), &$prefix)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_str_ends_with_impl(value: impl AsRef<str>, suffix: impl AsRef<str>) -> bool {
    value.as_ref().ends_with(suffix.as_ref())
}

/// Asserts that a string ends with a suffix.
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
            $crate::assertions::string_assertions::assert_str_ends_with_impl(&$value, &$suffix),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
                    .with_argument("suffix", stringify!($suffix), &$suffix)
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_str_matches_impl(value: impl AsRef<str>, pattern: impl AsRef<str>) -> bool {
    let pattern = Regex::new(pattern.as_ref()).unwrap();

    pattern.is_match(value.as_ref())
}

/// Asserts that a string matches a regular expression.
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
#[macro_export]
macro_rules! assert_str_matches {
    ($value:expr, $pattern:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value matches pattern",
            $crate::assertions::string_assertions::assert_str_matches_impl(&$value, &$pattern),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
                    .with_argument("pattern", stringify!($pattern), &$pattern)
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    // use crate::utilities::capture_output::capture_output;

    #[test]
    fn assert_str_contains_passing() {
        // let captured_outputs = capture_output(|| {
        assert_str_contains!("hello, world", "hello");
        // })
        // .unwrap();

        // std::assert!(captured_outputs.stdout.is_empty());
        // std::assert!(captured_outputs.stderr.is_empty());
    }
}
