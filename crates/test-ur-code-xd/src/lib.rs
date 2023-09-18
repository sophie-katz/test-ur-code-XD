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

//! This crate tries to improve testing in Rust with a more full-featured framework. It makes tests
//! more readable with better assertions and with more readable errors:
//!
//! <div><img src="https://raw.githubusercontent.com/sophie-katz/test-ur-code-XD/main/doc/assertion-screenshot.png" alt="A screenshot of an assertion failure message"></img></div>
//!
//! # Assertions
//!
//! [test-ur-code-XD](#) has some basic assertions that are similar to the ones in the standard library:
//!
//! * [`assert`] - Asserts that a boolean is true.
//! * [`assert_not`] - Asserts that a boolean is false.
//! * [`assert_eq`] - Asserts that two expressions are equal.
//! * [`assert_ne`] - Asserts that two expressions are inequal.
//! * [`assert_lt`] - Asserts that the first expression is less than the second expression.
//! * [`assert_le`] - Asserts that the first expression is less than or equal to the second.
//! * [`assert_gt`] - Asserts that the first expression is greater than the second expression.
//! * [`assert_ge`] - Asserts that the first expression is greater than or equal to the second.
//!
//! ## String assertions
//!
//! * [`assert_str_contains`] - Asserts that a string contains a substring.
//! * [`assert_str_starts_with`] - Asserts that a string starts with a substring.
//! * [`assert_str_ends_with`] - Asserts that a string ends with a substring.
//! * [`assert_str_matches`] - Asserts that a string matches a regular expression.
//!
//! ## Panic assertions
//!
//! Rust already has the built-in [`assert_panics`] macro, but [test-ur-code-XD](#) has a macro
//! which can assert that only specific lines of code panic:
//!
//! ```
//! // This code runs normally.
//!
//! assert_panics!(|| {
//!     // This code panics.
//!     panic!();
//! });
//!
//! // This code also runs normally
//! ```
//!
//! ## Output assertions for `stdout` and `stderr`
//!
//! This assertion allows you to write custom assertions for `stdout` and `stderr`:
//!
//! ```
//! assert_outputs!(|| {
//!     println!("print something to stdout");
//!     eprintln!("print something else to stderr");
//! }, |stdout| {
//!     assert_eq!(stdout, "print something to stdout\n");
//! }, |stderr| {
//!     assert_eq!(stderr, "print something else to stderr\n");
//! });
//! ```
//!
//! ## Filesystem assertions
//!
//! There are some assertions for simple filesystem checks:
//!
//! * [`assert_path_exists`] - Asserts that a path exists, whether it be a file or directory or
//!                            whatever.
//! * [`assert_path_is_relative`] - Asserts that a path is relative.
//! * [`assert_path_is_absolute`] - Asserts that a path is absolute.
//! * [`assert_path_is_file`] - Asserts that a path is a file.
//! * [`assert_path_is_symlink`] - Asserts that a path is a symlink.
//! * [`assert_path_is_dir`] - Asserts that a path is a directory.
//! * [`assert_path_starts_with`] - Asserts that a path starts with a prefix.
//! * [`assert_path_ends_with`] - Asserts that a path ends with a suffix.
//!
//! And there are also assertions about file contents:
//!
//! * [`assert_file_text_eq`] - Asserts that the contents of a file are equal to a string.
//! * [`assert_file_text_matches`] - Asserts that the contents of a file match a regular expression.
//!
//! ## Floating-point assertions
//!
//! ```
//! // assert_eq! would fail here because of floating-point rounding errors, but assert_f32_eq!
//! // takes this into account.
//! assert_f32_eq!(
//!     0.15 + 0.15 + 0.15,
//!     0.1 + 0.1 + 0.25
//! );
//! ```
//!
//! These assertions all have allowances for floating-point rounding errors:
//! * [`assert_f32_eq`] - Asserts that two `f32` values are equal.
//! * [`assert_f32_ne`] - Asserts that two `f32` values are inequal.
//! * [`assert_f32_le`] - Asserts that the first `f32` value is less than or equal to the second.
//! * [`assert_f32_ge`] - Asserts that the first `f32` value is greater than or equal to the second.
//! * [`assert_f64_eq`] - Asserts that two `f64` values are equal.
//! * [`assert_f64_ne`] - Asserts that two `f64` values are inequal.
//! * [`assert_f64_le`] - Asserts that the first `f64` value is less than or equal to the second.
//! * [`assert_f64_ge`] - Asserts that the first `f64` value is greater than or equal to the second.
//!
//! # Parameterized tests
//!
//! ```
//! #[test_with_parameter_values(
//!   x = [5, 6, 7],
//!   y = [1, 2])
//! ]
//! fn example(x: i32, y: i32) {
//!   // This will permute the values and automatically run all of these cases:
//!   //   x == 5, y == 1
//!   //   x == 5, y == 2
//!   //   x == 6, y == 1
//!   //   x == 6, y == 2
//!   //   x == 7, y == 1
//!   //   x == 7, y == 2
//! }
//! ```

#[doc(hidden)]
pub mod assertions;
mod capture_output;
