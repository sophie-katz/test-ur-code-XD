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

/// The separator used when truncating a string.
const TRUNCATION_SEPARATOR: &str = " ... ";

/// Gets the length of a string after truncating in the middle.
///
/// **NOTE:** This does not properly support newlines in the truncated string.
///
/// # Arguments
///
/// * `text` - The string to truncate.
/// * `context_len` - The amount of context to give both before and after the `" ... "`.
///
/// # Returns
///
/// * The original string's length if the length is less than or equal to
///   `TRUNCATION_SEPARATOR.len() + 2 * context_len`.
/// * Otherwise, `TRUNCATION_SEPARATOR.len() + 2 * context_len`.
pub fn middle_truncated_len(text: impl AsRef<str>, context_len: usize) -> usize {
    text.as_ref()
        .len()
        .min(TRUNCATION_SEPARATOR.len() + 2 * context_len)
}

/// Truncates a string in the middle.
///
/// **NOTE:** This does not properly support newlines in the truncated string.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::utilities::truncate;
/// #
/// let longer = "This is a relatively long string that can be truncated.";
///
/// let shorter = truncate::truncate_middle(
///     longer,
///     5,
/// );
///
/// assert_eq!(
///     shorter,
///     "This  ... ated."
/// );
/// ```
///
/// # Arguments
///
/// * `text` - The string to truncate.
/// * `context_len` - The amount of context to give both before and after the `" ... "`.
///
/// # Returns
///
/// * The original string if the length is less than or equal to `TRUNCATION_SEPARATOR.len() + 2 * context_len`.
/// * Otherwise, the truncated string.
pub fn truncate_middle(text: impl AsRef<str>, context_len: usize) -> String {
    let text = text.as_ref();

    if text.len() > TRUNCATION_SEPARATOR.len() + 2 * context_len {
        format!(
            "{}{}{}",
            &text[..context_len],
            TRUNCATION_SEPARATOR,
            &text[text.len() - context_len..]
        )
    } else {
        text.to_owned()
    }
}

/// Truncates a string at the end.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::utilities::truncate;
/// #
/// let longer = "This is a relatively long string.";
///
/// let shorter = truncate::truncate_end(
///     longer,
///     5
/// );
///
/// assert_eq!(
///     shorter,
///     "This  ..."
/// );
/// ```
///
/// # Arguments
///
/// * `text` - The string to truncate.
///
/// # Returns
///
/// * The original string if the length is less then or equal to `context_len + 4`.
/// * Otherwise, the truncated string:
///     * Which will be cut off after `context_len` characters.
///     * Or the first newline character, whichever comes first.
pub fn truncate_end(text: impl AsRef<str>, context_len: usize) -> String {
    let mut text = text.as_ref().to_owned();

    if text.len() > context_len + 4 {
        text = format!("{} ...", &text[..context_len]);
    }

    if let Some(newline_index) = text.find('\n') {
        text = format!("{} ...", &text[..newline_index]);
    }

    text
}
