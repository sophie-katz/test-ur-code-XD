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

#[cfg(feature = "string-diff")]
use console::{style, Color};

#[cfg(feature = "regex")]
use regex::Regex;

#[cfg(feature = "string-diff")]
const DIFF_CONTEXT_SIZE: usize = 5;

#[doc(hidden)]
pub fn assert_str_eq_impl(lhs: impl AsRef<str>, rhs: impl AsRef<str>) -> bool {
    lhs.as_ref() == rhs.as_ref()
}

#[cfg(feature = "string-diff")]
fn truncate(text: &str) -> String {
    if text.len() > 5 + 2 * DIFF_CONTEXT_SIZE {
        format!(
            "{} ... {}",
            &text[..DIFF_CONTEXT_SIZE],
            &text[text.len() - DIFF_CONTEXT_SIZE..]
        )
    } else {
        text.to_owned()
    }
}

#[cfg(feature = "string-diff")]
fn format_diff_text_line(diffs: &[diff::Result<String>]) -> String {
    let mut result = String::new();

    for diff in diffs {
        match diff {
            diff::Result::Left(left) => {
                result.push_str(style(truncate(left)).fg(Color::Green).to_string().as_str());
            }
            diff::Result::Right(right) => {
                result.push_str(style(truncate(right)).fg(Color::Red).to_string().as_str());
            }
            diff::Result::Both(both, _) => {
                result.push_str(truncate(both).as_str());
            }
        }
    }
    result
}

#[cfg(feature = "string-diff")]
fn format_diff_diff_line(diffs: &[diff::Result<String>]) -> String {
    let mut result = String::new();

    for diff in diffs {
        match diff {
            diff::Result::Left(left) => {
                result.push_str(
                    style("<".repeat(left.len().min(5 + 2 * DIFF_CONTEXT_SIZE)))
                        .fg(Color::Green)
                        .to_string()
                        .as_str(),
                );
            }
            diff::Result::Right(right) => {
                result.push_str(
                    style(">".repeat(right.len().min(5 + 2 * DIFF_CONTEXT_SIZE)))
                        .fg(Color::Red)
                        .to_string()
                        .as_str(),
                );
            }
            diff::Result::Both(both, _) => {
                result.push_str(
                    " ".repeat(both.len().min(5 + 2 * DIFF_CONTEXT_SIZE))
                        .as_str(),
                );
            }
        }
    }

    result
}

#[cfg(feature = "string-diff")]
fn convert_char_diff_to_string_diff(diff: &diff::Result<char>) -> diff::Result<String> {
    match diff {
        diff::Result::Left(left) => diff::Result::Left(left.to_string()),
        diff::Result::Right(right) => diff::Result::Right(right.to_string()),
        diff::Result::Both(both, _) => diff::Result::Both(both.to_string(), both.to_string()),
    }
}

#[cfg(feature = "string-diff")]
fn are_diffs_same_variant<T, U>(lhs: &diff::Result<T>, rhs: &diff::Result<U>) -> bool {
    match (lhs, rhs) {
        (diff::Result::Left(_), diff::Result::Left(_)) => true,
        (diff::Result::Both(_, _), diff::Result::Both(_, _)) => true,
        (diff::Result::Right(_), diff::Result::Right(_)) => true,
        _ => false,
    }
}

#[cfg(feature = "string-diff")]
fn append_char_diff_to_string_diff(
    string_diff: &mut diff::Result<String>,
    char_diff: &diff::Result<char>,
) {
    match (string_diff, char_diff) {
        (diff::Result::Left(string_value), diff::Result::Left(char_value)) => {
            string_value.push(*char_value)
        }
        (
            diff::Result::Both(string_value_left, string_value_right),
            diff::Result::Both(char_value, _),
        ) => {
            string_value_left.push(*char_value);
            string_value_right.push(*char_value);
        }
        (diff::Result::Right(string_value), diff::Result::Right(char_value)) => {
            string_value.push(*char_value)
        }
        _ => panic!("string diff and char diff must be of the same variant"),
    }
}

#[cfg(feature = "string-diff")]
fn merge_char_diffs(diffs: &[diff::Result<char>]) -> Vec<diff::Result<String>> {
    let mut result: Vec<diff::Result<String>> = Vec::new();

    let mut current: Option<diff::Result<String>> = None;

    for diff in diffs {
        if current.is_none() {
            current = Some(convert_char_diff_to_string_diff(diff));
        } else if let Some(current_value) = &mut current {
            if are_diffs_same_variant(current_value, diff) {
                append_char_diff_to_string_diff(current_value, diff);
            } else {
                result.push(current_value.clone());
                current = Some(convert_char_diff_to_string_diff(diff));
            }
        }
    }

    if let Some(current_value) = current {
        result.push(current_value);
    }

    result
}

#[doc(hidden)]
#[cfg(feature = "string-diff")]
pub fn format_diff(lhs: &str, rhs: &str) -> String {
    let lhs_string = format!("{:?}", lhs);
    let rhs_string = format!("{:?}", rhs);

    let char_diffs = diff::chars(lhs_string.as_str(), rhs_string.as_str());
    let string_diffs = merge_char_diffs(&char_diffs);

    format!(
        "{}\n   {}",
        format_diff_text_line(&string_diffs),
        format_diff_diff_line(&string_diffs)
    )
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
            $crate::assertions::string_assertions::assert_str_eq_impl(&$lhs, &$rhs),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", stringify!($lhs), &$lhs)
                    .with_argument("rhs", stringify!($rhs), &$rhs)
                    .with_argument_formatted("diff", "--",
                        $crate::assertions::string_assertions::format_diff(&$lhs, &$rhs)
                    )
            }
            $(, $keys = $values)*
        )
    };
}

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
#[cfg(feature = "regex")]
pub fn assert_str_matches_impl(value: impl AsRef<str>, pattern: impl AsRef<str>) -> bool {
    let pattern = Regex::new(pattern.as_ref()).unwrap();

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
    #[cfg(feature = "string-diff")]
    use super::*;

    #[cfg(feature = "string-diff")]
    use crate::assert_eq;

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_all_both() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Both("hello".to_owned(), "hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_all_left() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Left("hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_all_right() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Right("hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_long_both() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Both(
            "a".repeat(DIFF_CONTEXT_SIZE + 100),
            "a".repeat(DIFF_CONTEXT_SIZE + 100),
        )]);

        assert_eq!(formatted, "aaaaa ... aaaaa");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_long_left() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Left("a".repeat(DIFF_CONTEXT_SIZE + 100))]);

        assert_eq!(formatted, "aaaaa ... aaaaa");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_long_right() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Right("a".repeat(DIFF_CONTEXT_SIZE + 100))]);

        assert_eq!(formatted, "aaaaa ... aaaaa");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_text_line_mixed() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[
            diff::Result::Both("hello".to_owned(), "hello".to_owned()),
            diff::Result::Left(", ".to_owned()),
            diff::Result::Right("world".to_owned()),
        ]);

        assert_eq!(formatted, "hello, world");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_all_both() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_diff_line(&[diff::Result::Both("hello".to_owned(), "hello".to_owned())]);

        assert_eq!(formatted, "     ");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_all_left() {
        console::set_colors_enabled(false);

        let formatted = format_diff_diff_line(&[diff::Result::Left("hello".to_owned())]);

        assert_eq!(formatted, "<<<<<");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_all_right() {
        console::set_colors_enabled(false);

        let formatted = format_diff_diff_line(&[diff::Result::Right("hello".to_owned())]);

        assert_eq!(formatted, ">>>>>");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_long_both() {
        console::set_colors_enabled(false);

        let formatted = format_diff_diff_line(&[diff::Result::Both(
            "a".repeat(DIFF_CONTEXT_SIZE + 100),
            "a".repeat(DIFF_CONTEXT_SIZE + 100),
        )]);

        assert_eq!(formatted, "               ");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_long_left() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_diff_line(&[diff::Result::Left("a".repeat(DIFF_CONTEXT_SIZE + 100))]);

        assert_eq!(formatted, "<<<<<<<<<<<<<<<");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_long_right() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_diff_line(&[diff::Result::Right("a".repeat(DIFF_CONTEXT_SIZE + 100))]);

        assert_eq!(formatted, ">>>>>>>>>>>>>>>");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_diff_line_mixed() {
        console::set_colors_enabled(false);

        let formatted = format_diff_diff_line(&[
            diff::Result::Both("hello".to_owned(), "hello".to_owned()),
            diff::Result::Left(", ".to_owned()),
            diff::Result::Right("world".to_owned()),
        ]);

        assert_eq!(formatted, "     <<>>>>>");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    fn format_diff_simple() {
        console::set_colors_enabled(false);

        let formatted = format_diff("hello, ", "helloworld");

        assert_eq!(formatted, "\"hello, world\"\n         <<>>>>> ");
    }

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
    #[should_panic]
    fn assert_str_eq_failing_empty_some() {
        assert_str_eq!("", "asdf");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic]
    fn assert_str_eq_failing_some_empty() {
        assert_str_eq!("asdf", "");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic]
    fn assert_str_eq_failing_totally_different() {
        assert_str_eq!("hello, world", "asdf");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic]
    fn assert_str_eq_failing_slightly_different() {
        assert_str_eq!("hello, world", "hello! world");
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic]
    fn assert_str_eq_failing_long() {
        assert_str_eq!("a".repeat(100), "b".repeat(100));
    }

    #[cfg(feature = "string-diff")]
    #[test]
    #[should_panic]
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
    #[should_panic]
    fn assert_str_contains_failing() {
        assert_str_contains!("hello, world", "asdf");
    }

    #[test]
    #[should_panic]
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
    #[should_panic]
    fn assert_str_starts_with_failing() {
        assert_str_starts_with!("hello, world", "world");
    }

    #[test]
    #[should_panic]
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
    #[should_panic]
    fn assert_str_ends_with_failing() {
        assert_str_ends_with!("hello, world", "hello");
    }

    #[test]
    #[should_panic]
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
    #[should_panic]
    fn assert_str_matches_failing_partial() {
        assert_str_matches!("hello, world", "[A-Z]+");
    }

    #[cfg(feature = "regex")]
    #[test]
    #[should_panic]
    fn assert_str_matches_failing_whole() {
        assert_str_matches!("hello, world", "^[a-z]+$");
    }

    #[cfg(feature = "regex")]
    #[test]
    #[should_panic]
    fn assert_str_matches_failing_bad_regex() {
        assert_str_matches!("hello, world", "[a-z, ");
    }
}
