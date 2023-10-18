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

//! Some utilities to extend the [`diff`] crate.

#![allow(clippy::absolute_paths)]

use crate::utilities::truncate::Truncate;
use console::{style, Color};
use unicode_segmentation::UnicodeSegmentation;

use super::truncate::TruncationMode;

/// Separator to use for diff truncation.
const DIFF_TRUNCATION_SEPARATOR: &str = " ... ";

/// The amount of context in characters to show around a diff.
const DIFF_MAX_GRAPHEME_LEN: usize = 20;

/// Formats the diff between two strings.
///
/// # Arguments
///
/// * `lhs` - The left-hand side of the diff.
/// * `rhs` - The right-hand side of the diff.
/// * `indent` - The amount of indentation to use for the second line.
///
/// # Returns
///
/// * The formatted diff.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn format_diff(lhs: &str, rhs: &str, indent: usize) -> String {
    // Make strings diffable
    let lhs_diffable = convert_str_to_diffable_string(lhs);
    let rhs_diffable = convert_str_to_diffable_string(rhs);

    // Diff strings character-by-character
    let char_diffs = diff::chars(lhs_diffable.as_str(), rhs_diffable.as_str());

    // Merge character-by-character diffs into string-by-string diffs
    let string_diffs = merge_char_diffs(&char_diffs);

    // Format string-by-string diffs
    format!(
        "{}\n{}{}",
        format_diff_text_line(&string_diffs),
        " ".repeat(indent),
        format_diff_marker_line(&string_diffs)
    )
}

/// Takes a string and converts it to a diffable string.
fn convert_str_to_diffable_string(string: &str) -> String {
    format!("{string:?}")
}

/// Formats the first line of the diff, where the text is just displayed.
///
/// # Arguments
///
/// * `diffs` - The sequence of diffs to format.
///
/// # Returns
///
/// * The formatted text line.
fn format_diff_text_line(diffs: &[diff::Result<String>]) -> String {
    let mut result = String::new();

    for diff in diffs {
        match diff {
            diff::Result::Left(left) => {
                result.push_str(
                    style(left.to_truncated(
                        DIFF_TRUNCATION_SEPARATOR,
                        TruncationMode::Middle,
                        DIFF_MAX_GRAPHEME_LEN,
                    ))
                    .fg(Color::Green)
                    .to_string()
                    .as_str(),
                );
            }
            diff::Result::Right(right) => {
                result.push_str(
                    style(right.to_truncated(
                        DIFF_TRUNCATION_SEPARATOR,
                        TruncationMode::Middle,
                        DIFF_MAX_GRAPHEME_LEN,
                    ))
                    .fg(Color::Red)
                    .to_string()
                    .as_str(),
                );
            }
            diff::Result::Both(both, _) => {
                result.push_str(
                    both.to_truncated(
                        DIFF_TRUNCATION_SEPARATOR,
                        TruncationMode::Middle,
                        DIFF_MAX_GRAPHEME_LEN,
                    )
                    .as_str(),
                );
            }
        }
    }
    result
}

/// Formats the second line of the diff, where the difference markers are displayed.
///
/// # Arguments
///
/// * `diffs` - The sequence of diffs to format.
///
/// # Returns
///
/// * The formatted marker line.
fn format_diff_marker_line(diffs: &[diff::Result<String>]) -> String {
    let mut result = String::new();

    for diff in diffs {
        match diff {
            diff::Result::Left(left) => {
                let left_graphemes_len = left.graphemes(true).count();

                result.push_str(
                    style("<".repeat(left_graphemes_len.min(DIFF_MAX_GRAPHEME_LEN)))
                        .fg(Color::Green)
                        .to_string()
                        .as_str(),
                );
            }
            diff::Result::Right(right) => {
                let right_graphemes_len = right.graphemes(true).count();

                result.push_str(
                    style(">".repeat(right_graphemes_len.min(DIFF_MAX_GRAPHEME_LEN)))
                        .fg(Color::Red)
                        .to_string()
                        .as_str(),
                );
            }
            diff::Result::Both(both, _) => {
                let both_graphemes_len = both.graphemes(true).count();

                result.push_str(
                    " ".repeat(both_graphemes_len.min(DIFF_MAX_GRAPHEME_LEN))
                        .as_str(),
                );
            }
        }
    }

    result
}

/// Converts a character diff to a string diff containing just that one character.
fn convert_char_diff_to_string_diff(diff: &diff::Result<char>) -> diff::Result<String> {
    match diff {
        diff::Result::Left(left) => diff::Result::Left(left.to_string()),
        diff::Result::Right(right) => diff::Result::Right(right.to_string()),
        diff::Result::Both(both, _) => diff::Result::Both(both.to_string(), both.to_string()),
    }
}

/// Checks if two diffs with different value types contain the same variant.
fn are_diffs_same_variant<T, U>(lhs: &diff::Result<T>, rhs: &diff::Result<U>) -> bool {
    matches!(
        (lhs, rhs),
        (diff::Result::Left(_), diff::Result::Left(_))
            | (diff::Result::Both(_, _), diff::Result::Both(_, _))
            | (diff::Result::Right(_), diff::Result::Right(_))
    )
}

/// Appends a character diff to a string diff.
///
/// # Returns
///
/// * `Some(appended_string_diff)` if the two diffs are of the same variant.
/// * `None` if the two diffs are not of the same variant.
fn append_char_diff_to_string_diff(
    mut string_diff: diff::Result<String>,
    char_diff: &diff::Result<char>,
) -> Option<diff::Result<String>> {
    match (&mut string_diff, char_diff) {
        (diff::Result::Left(string_value), diff::Result::Left(char_value))
        | (diff::Result::Right(string_value), diff::Result::Right(char_value)) => {
            string_value.push(*char_value);
            Some(string_diff)
        }
        (
            diff::Result::Both(string_value_left, string_value_right),
            diff::Result::Both(char_value, _),
        ) => {
            string_value_left.push(*char_value);
            string_value_right.push(*char_value);
            Some(string_diff)
        }
        _ => None,
    }
}

/// Merges a sequence of character diffs into a sequence of string diffs.
#[allow(clippy::expect_used)]
fn merge_char_diffs(diffs: &[diff::Result<char>]) -> Vec<diff::Result<String>> {
    let mut result: Vec<diff::Result<String>> = Vec::new();

    let mut current: Option<diff::Result<String>> = None;

    for diff in diffs {
        if let Some(current_value) = current {
            if are_diffs_same_variant(&current_value, diff) {
                current = Some(
                    append_char_diff_to_string_diff(current_value, diff)
                        .expect("both diffs to be of the same variant"),
                );
            } else {
                result.push(current_value.clone());
                current = Some(convert_char_diff_to_string_diff(diff));
            }
        } else {
            current = Some(convert_char_diff_to_string_diff(diff));
        }
    }

    if let Some(current_value) = current {
        result.push(current_value);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_diff_text_line_all_both() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Both("hello".to_owned(), "hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[test]
    fn format_diff_text_line_all_left() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Left("hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[test]
    fn format_diff_text_line_all_right() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Right("hello".to_owned())]);

        assert_eq!(formatted, "hello");
    }

    #[test]
    fn format_diff_text_line_long_both() {
        console::set_colors_enabled(false);

        let formatted = format_diff_text_line(&[diff::Result::Both(
            "a".repeat(DIFF_MAX_GRAPHEME_LEN + 100),
            "a".repeat(DIFF_MAX_GRAPHEME_LEN + 100),
        )]);

        assert_eq!(formatted, "aaaaaaaa ... aaaaaaa");
    }

    #[test]
    fn format_diff_text_line_long_left() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Left("a".repeat(DIFF_MAX_GRAPHEME_LEN + 100))]);

        assert_eq!(formatted, "aaaaaaaa ... aaaaaaa");
    }

    #[test]
    fn format_diff_text_line_long_right() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_text_line(&[diff::Result::Right("a".repeat(DIFF_MAX_GRAPHEME_LEN + 100))]);

        assert_eq!(formatted, "aaaaaaaa ... aaaaaaa");
    }

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

    #[test]
    fn format_diff_marker_line_all_both() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_marker_line(&[diff::Result::Both("hello".to_owned(), "hello".to_owned())]);

        assert_eq!(formatted, "     ");
    }

    #[test]
    fn format_diff_marker_line_all_left() {
        console::set_colors_enabled(false);

        let formatted = format_diff_marker_line(&[diff::Result::Left("hello".to_owned())]);

        assert_eq!(formatted, "<<<<<");
    }

    #[test]
    fn format_diff_marker_line_all_right() {
        console::set_colors_enabled(false);

        let formatted = format_diff_marker_line(&[diff::Result::Right("hello".to_owned())]);

        assert_eq!(formatted, ">>>>>");
    }

    #[test]
    fn format_diff_marker_line_long_both() {
        console::set_colors_enabled(false);

        let formatted = format_diff_marker_line(&[diff::Result::Both(
            "a".repeat(DIFF_MAX_GRAPHEME_LEN + 100),
            "a".repeat(DIFF_MAX_GRAPHEME_LEN + 100),
        )]);

        assert_eq!(formatted, "                    ");
    }

    #[test]
    fn format_diff_marker_line_long_left() {
        console::set_colors_enabled(false);

        let formatted =
            format_diff_marker_line(&[diff::Result::Left("a".repeat(DIFF_MAX_GRAPHEME_LEN + 100))]);

        assert_eq!(formatted, "<<<<<<<<<<<<<<<<<<<<");
    }

    #[test]
    fn format_diff_marker_line_long_right() {
        console::set_colors_enabled(false);

        let formatted = format_diff_marker_line(&[diff::Result::Right(
            "a".repeat(DIFF_MAX_GRAPHEME_LEN + 100),
        )]);

        assert_eq!(formatted, ">>>>>>>>>>>>>>>>>>>>");
    }

    #[test]
    fn format_diff_marker_line_mixed() {
        console::set_colors_enabled(false);

        let formatted = format_diff_marker_line(&[
            diff::Result::Both("hello".to_owned(), "hello".to_owned()),
            diff::Result::Left(", ".to_owned()),
            diff::Result::Right("world".to_owned()),
        ]);

        assert_eq!(formatted, "     <<>>>>>");
    }

    #[test]
    fn format_diff_simple() {
        console::set_colors_enabled(false);

        let formatted = format_diff("hello, ", "helloworld", 3);

        assert_eq!(formatted, "\"hello, world\"\n         <<>>>>> ");
    }
}
