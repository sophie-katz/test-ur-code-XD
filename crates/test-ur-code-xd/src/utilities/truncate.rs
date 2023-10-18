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

#![allow(clippy::arithmetic_side_effects, clippy::module_name_repetitions)]

use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

/// Different modes of truncation
#[derive(Clone, Copy)]
// Making the enum non-exhaustive as future-proofing.
#[non_exhaustive]
pub enum TruncationMode {
    /// Truncate text at the start so that the end of the string is guaranteed to be present.
    Start,
    /// Truncate text in the middle so that the start and end of the string are guaranteed to be
    /// present.
    Middle,
    /// Truncate text at the end so that the start of the string is guaranteed to be present.
    End,
}

/// A trait for truncation
pub trait Truncate {
    /// Truncates a string.
    ///
    /// # Example
    ///
    /// ```
    /// # use test_ur_code_xd::utilities::truncate::{Truncate, TruncationMode};
    /// #
    /// let truncated = "This is a string that is long enough to be truncated.".to_truncated(" ... ", TruncationMode::Middle, 18);
    ///
    /// assert_eq!(truncated, "This is ... cated.");
    /// ```
    ///
    /// # Arguments
    ///
    /// * `separator` - The separator to use between the truncated parts of the string.
    /// * `mode` - The truncation mode.
    /// * `max_grapheme_len` - The maximum length of the truncated string in graphemes.
    ///
    /// # Returns
    ///
    /// The truncated string.
    ///
    /// # Panics
    ///
    /// * If the there are issues converting sizes. Since most strings are not gigabytes in length,
    ///   this should not be an issue.
    fn to_truncated(
        &self,
        separator: impl AsRef<str>,
        mode: TruncationMode,
        max_grapheme_len: usize,
    ) -> String;
}

impl Truncate for str {
    // Allow panic because of issues with truncation that shouldn't occur between grapheme
    // boundaries.
    #[allow(clippy::panic)]
    fn to_truncated(
        &self,
        separator: impl AsRef<str>,
        mode: TruncationMode,
        max_grapheme_len: usize,
    ) -> String {
        // Segment graphemes
        let self_graphemes: Vec<&str> = self.graphemes(true).collect();
        let separator_graphemes: Vec<&str> = separator.as_ref().graphemes(true).collect();

        // If the string is already short enough, return it as is
        if self_graphemes.len() <= max_grapheme_len && !self_graphemes.contains(&"\n") {
            return self.to_owned();
        }

        // Generate the grapheme segments for the truncated string
        let truncated_graphemes_iter: Vec<&str> = match mode {
            TruncationMode::Start => {
                // Calculate the context length in graphemes after the separator
                let context_after_grapheme_len = get_context_grapheme_lengths_start_or_end(
                    separator_graphemes.len(),
                    max_grapheme_len,
                );

                // Make sure there are no newlines in the context
                let context_after_grapheme_len = get_context_grapheme_length_or_after_last_newline(
                    &self_graphemes,
                    context_after_grapheme_len,
                );

                // Slice the graphemes
                let context_after_graphemes: &[&str] = self_graphemes
                    .get(context_after_grapheme_len..)
                    .unwrap_or_else(|| {
                        panic!(
                            "unable to truncate from start at index {context_after_grapheme_len}"
                        )
                    });

                // Chain together the graphemes into a vector
                separator_graphemes
                    .into_iter()
                    .chain(context_after_graphemes.iter().copied())
                    .collect()
            }
            TruncationMode::Middle => {
                // Calculate the context length in graphemes before and after the separator
                let (context_before_grapheme_len, context_after_grapheme_len) =
                    get_context_grapheme_lengths_middle(
                        separator_graphemes.len(),
                        max_grapheme_len,
                    );

                // Make sure there are no newlines in the contexts
                let (context_before_grapheme_len, context_after_grapheme_len) = (
                    get_context_grapheme_length_or_before_first_newline(
                        &self_graphemes,
                        context_before_grapheme_len,
                    ),
                    get_context_grapheme_length_or_after_last_newline(
                        &self_graphemes,
                        context_after_grapheme_len,
                    ),
                );

                // Slice the graphemes before the separator
                let context_before_graphemes: &[&str] =
                    self_graphemes.get(..context_before_grapheme_len).unwrap_or_else(|| panic!("unable to truncate in middle starting at index {context_before_grapheme_len}"));

                // Slice the graphemes after the separator
                let context_after_graphemes: &[&str] =
                    self_graphemes.get(context_after_grapheme_len..).unwrap_or_else(|| panic!("unable to truncate in middle ending at index {context_after_grapheme_len}"));

                // Chain together the graphemes into a vector
                context_before_graphemes
                    .iter()
                    .copied()
                    .chain(separator_graphemes)
                    .chain(context_after_graphemes.iter().copied())
                    .collect()
            }
            TruncationMode::End => {
                // Calculate the context length in graphemes before the separator
                let context_before_grapheme_len = get_context_grapheme_lengths_start_or_end(
                    separator_graphemes.len(),
                    max_grapheme_len,
                );

                // Make sure there are no newlines in the context
                let context_before_grapheme_len =
                    get_context_grapheme_length_or_before_first_newline(
                        &self_graphemes,
                        context_before_grapheme_len,
                    );

                // Slice the graphemes
                let context_before_graphemes: &[&str] = self_graphemes
                    .get(..context_before_grapheme_len)
                    .unwrap_or_else(|| {
                        panic!("unable to truncate from end at index {context_before_grapheme_len}")
                    });

                // Chain together the graphemes into a vector
                context_before_graphemes
                    .iter()
                    .copied()
                    .chain(separator_graphemes)
                    .collect()
            }
        };

        // Concatenate the graphemes into a string
        truncated_graphemes_iter.concat()
    }
}

/// Gets the context length for truncating a string from the start while being aware of newlines.
///
/// # Arguments
///
/// * `graphemes` - The graphemes of the string.
/// * `context_grapheme_len` - The context length in graphemes.
///
/// # Returns
///
/// * `context_grapheme` if there are no newlines before `context_grapheme`
/// * or the index just before the first newline if there is one before `context_grapheme`.
fn get_context_grapheme_length_or_before_first_newline(
    graphemes: &[&str],
    context_grapheme_len: usize,
) -> usize {
    graphemes
        .iter()
        .take(context_grapheme_len)
        .copied()
        .position(|value| value == "\n" || value == "\r\n")
        .unwrap_or(context_grapheme_len)
}

/// Gets the context length for truncating a string from the end while being aware of newlines.
///
/// # Arguments
///
/// * `graphemes` - The graphemes of the string.
/// * `context_grapheme_len` - The context length in graphemes.
///
/// # Returns
///
/// * `context_grapheme` if there are no newlines after `context_grapheme`
/// * or the index just before the first newline if there is one before `context_grapheme`.
fn get_context_grapheme_length_or_after_last_newline(
    graphemes: &[&str],
    context_grapheme_len: usize,
) -> usize {
    graphemes
        .iter()
        .skip(graphemes.len() - context_grapheme_len - 1)
        .copied()
        .rposition(|value| value == "\n" || value == "\r\n")
        .unwrap_or(graphemes.len() - context_grapheme_len - 1)
}

/// Gets the context length for truncating a string from the start or the end.
///
/// # Examples
///
/// To get the context length for truncating a string at the start:
///
/// ```ignore
/// // The separator is " ... "
/// let separator_grapheme_len = 5;
///
/// // The maximum truncated string length is 10 graphemes.
/// let max_grapheme_len = 10;
///
/// let after = get_context_grapheme_lengths_start_or_end(
///     separator_grapheme_len,
///     TruncationMode::Start,
///     max_grapheme_len,
/// );
///
/// assert_eq!(after, 5);
/// ```
///
/// # Arguments
///
/// * `separator_grapheme_len` - The length of the separator in graphemes.
/// * `mode` - The truncation mode.
/// * `max_grapheme_len` - The maximum length of the truncated string in graphemes.
///
/// # Returns
///
/// The context length in graphemes.
fn get_context_grapheme_lengths_start_or_end(
    separator_grapheme_len: usize,
    max_grapheme_len: usize,
) -> usize {
    max_grapheme_len - separator_grapheme_len
}

/// Gets the context length for truncating a string in the middle.
///
/// # Examples
///
/// To get the context length for truncating a string:
///
/// ```ignore
/// // The separator is " ... "
/// let separator_grapheme_len = 5;
///
/// // The maximum truncated string length is 10 graphemes.
/// let max_grapheme_len = 10;
///
/// let (before, after) = get_context_grapheme_lengths_middle(
///     separator_grapheme_len,
///     TruncationMode::Start,
///     max_grapheme_len,
/// );
///
/// assert_eq!(before, 5);
/// assert_eq!(after, 5);
/// ```
///
/// # Arguments
///
/// * `separator_grapheme_len` - The length of the separator in graphemes.
/// * `mode` - The truncation mode.
/// * `max_grapheme_len` - The maximum length of the truncated string in graphemes.
///
/// # Returns
///
/// A tuple of context lengths in graphemes `(before, after)`.
fn get_context_grapheme_lengths_middle(
    separator_grapheme_len: usize,
    max_grapheme_len: usize,
) -> (usize, usize) {
    let unrounded = convert_grapheme_len_to_f64(max_grapheme_len - separator_grapheme_len) / 2.0;

    let context_grapheme_len_before = convert_f64_to_grapheme_len(unrounded.ceil());

    (
        context_grapheme_len_before,
        max_grapheme_len - context_grapheme_len_before - separator_grapheme_len - 1,
    )
}

/// Safely converts a grapheme length to a `f64`.
//
// Allow panics for arithmetic overflows because most strings are not gigabytes in length.
#[allow(clippy::panic)]
fn convert_grapheme_len_to_f64(grapheme_len: usize) -> f64 {
    match grapheme_len.to_f64() {
        Some(value) => {
            if value.is_sign_negative() {
                panic!("converting grapheme length {grapheme_len} to f64 yields negative number {value}, so unable to truncate")
            } else if !value.is_finite() {
                panic!("converting grapheme length {grapheme_len} to f64 yields non-finite number {value}, so unable to truncate")
            } else {
                value
            }
        }
        None => {
            panic!("unable to convert grapheme length {grapheme_len} to f64, so unable to truncate")
        }
    }
}

/// Safely converts an `f64` to a grapheme length.
//
// Allow panics for arithmetic overflows because most strings are not gigabytes in length.
#[allow(clippy::panic)]
fn convert_f64_to_grapheme_len(value: f64) -> usize {
    match value.to_usize() {
        Some(value) => value,
        None => {
            panic!("unable to convert context length {value} to usize, so unable to truncate")
        }
    }
}
