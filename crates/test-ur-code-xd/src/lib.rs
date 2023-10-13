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

//! This crate tries to improve testing in Rust with a more full-featured framework. It makes tests
//! more readable with better assertions and with more readable errors:
//!
//! <div><img src="https://raw.githubusercontent.com/sophie-katz/test-ur-code-XD/main/docs/for-users/docs/assets/assertion-screenshot.png" alt="A screenshot of an assertion failure message"></img></div>
//!
//! See the [user guide](https://sophie-katz.github.io/test-ur-code-XD/) for more information about
//! how to use this crate.
//!
//! # Assertions
//!
//! test ur code XD has some basic assertions that are similar to the ones in the standard library:
//!
//! * [`assert`] - Asserts that a boolean is true.
//! * [`assert_not`] - Asserts that a boolean is false.
//! * [`assert_eq`] - Asserts that two expressions are equal.
//! * [`assert_ne`] - Asserts that two expressions are unequal.
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
//! Rust already has the built-in [`assert_panics`] macro, but test ur code XD has a macro
//! which can assert that only specific lines of code panic:
//!
//! ```
//! # #[cfg(feature = "panic")]
//! # use test_ur_code_xd::assert_panics;
//! #
//! // This code runs normally.
//!
//! # #[cfg(feature = "panic")]
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
//! # #[cfg(feature = "output")]
//! # use test_ur_code_xd::assert_outputs;
//! #
//! # #[cfg(feature = "output")]
//! assert_outputs!(|| {
//!     println!("print something to stdout");
//!     eprintln!("print something else to stderr");
//! }, on_stdout = |stdout| {
//!     assert_eq!(stdout, "print something to stdout\n");
//! }, on_stderr = |stderr| {
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
//! # #[cfg(feature = "float")]
//! # use test_ur_code_xd::assert_f32_eq;
//! #
//! # #[cfg(feature = "float")]
//! // assert_eq! would fail here because of floating-point rounding errors, but assert_f32_eq!
//! // takes this into account.
//! assert_f32_eq!(
//!     0.15 + 0.15 + 0.15,
//!     0.1 + 0.1 + 0.25,
//!     ulps = 1,
//!     epsilon_near_zero = 0.0,
//! );
//! ```
//!
//! These assertions all have allowances for floating-point rounding errors:
//! * [`assert_f32_eq`] - Asserts that two `f32` values are equal.
//! * [`assert_f32_ne`] - Asserts that two `f32` values are unequal.
//! * [`assert_f32_le`] - Asserts that the first `f32` value is less than or equal to the second.
//! * [`assert_f32_ge`] - Asserts that the first `f32` value is greater than or equal to the second.
//! * [`assert_f64_eq`] - Asserts that two `f64` values are equal.
//! * [`assert_f64_ne`] - Asserts that two `f64` values are unequal.
//! * [`assert_f64_le`] - Asserts that the first `f64` value is less than or equal to the second.
//! * [`assert_f64_ge`] - Asserts that the first `f64` value is greater than or equal to the second.
//!
//! # Parameterized tests
//!
//! ```
//! # #[cfg(feature = "macros")]
//! # use test_ur_code_xd_macro::test_with_parameter_values;
//! #
//! # #[cfg(feature = "macros")]
//! #[test_with_parameter_values(
//!   x = [5, 6, 7],
//!   y = [1, 2]
//! )]
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

// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::cargo_common_metadata)]
// #![warn(clippy::negative_feature_names)]
// #![warn(clippy::redundant_feature_names)]
// #![warn(clippy::wildcard_dependencies)]
// #![warn(clippy::absolute_paths)]
// #![warn(clippy::allow_attributes)]
// #![warn(clippy::allow_attributes_without_reason)]
// #![warn(clippy::arithmetic_side_effects)]
// #![warn(clippy::as_conversions)]
// #![warn(clippy::as_underscore)]
// #![warn(clippy::assertions_on_result_states)]
// #![warn(clippy::clone_on_ref_ptr)]
// #![warn(clippy::create_dir)]
// #![warn(clippy::dbg_macro)]
// #![warn(clippy::decimal_literal_representation)]
// #![warn(clippy::deref_by_slicing)]
// #![warn(clippy::else_if_without_else)]
// #![warn(clippy::empty_drop)]
// #![warn(clippy::empty_structs_with_brackets)]
// #![warn(clippy::error_impl_error)]
// #![warn(clippy::exhaustive_enums)]
// #![warn(clippy::exhaustive_structs)]
// #![warn(clippy::exit)]
// #![warn(clippy::expect_used)]
// #![warn(clippy::filetype_is_file)]
// #![warn(clippy::float_cmp_const)]
// #![warn(clippy::fn_to_numeric_cast_any)]
// #![warn(clippy::format_push_string)]
// #![warn(clippy::get_unwrap)]
// #![warn(clippy::host_endian_bytes)]
// #![warn(clippy::big_endian_bytes)]
// #![warn(clippy::if_then_some_else_none)]
// #![warn(clippy::indexing_slicing)]
// #![warn(clippy::integer_division)]
// #![warn(clippy::large_include_file)]
// #![warn(clippy::let_underscore_must_use)]
// #![warn(clippy::let_underscore_untyped)]
// #![warn(clippy::little_endian_bytes)]
// #![warn(clippy::lossy_float_literal)]
// #![warn(clippy::map_err_ignore)]
// #![warn(clippy::mem_forget)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::missing_trait_methods)]
// #![warn(clippy::mixed_read_write_in_expression)]
// #![warn(clippy::mod_module_files)]
// #![warn(clippy::multiple_inherent_impl)]
// #![warn(clippy::multiple_unsafe_ops_per_block)]
// #![warn(clippy::mutex_atomic)]
// #![warn(clippy::needless_raw_strings)]
// #![warn(clippy::non_ascii_literal)]
// #![warn(clippy::panic_in_result_fn)]
// #![warn(clippy::partial_pub_fields)]
// #![warn(clippy::print_stderr)]
// #![warn(clippy::print_stdout)]
// #![warn(clippy::pub_without_shorthand)]
// #![warn(clippy::rc_buffer)]
// #![warn(clippy::rc_mutex)]
// #![warn(clippy::redundant_type_annotations)]
// #![warn(clippy::rest_pat_in_fully_bound_structs)]
// #![warn(clippy::same_name_method)]
// #![warn(clippy::semicolon_inside_block)]
// #![warn(clippy::shadow_same)]
// #![warn(clippy::shadow_unrelated)]
// #![warn(clippy::single_char_lifetime_names)]
// #![warn(clippy::str_to_string)]
// #![warn(clippy::string_add)]
// #![warn(clippy::string_lit_chars_any)]
// #![warn(clippy::string_slice)]
// #![warn(clippy::string_to_string)]
// #![warn(clippy::suspicious_xor_used_as_pow)]
// #![warn(clippy::tests_outside_test_module)]
// #![warn(clippy::todo)]
// #![warn(clippy::try_err)]
// #![warn(clippy::undocumented_unsafe_blocks)]
// #![warn(clippy::unimplemented)]
// #![warn(clippy::unnecessary_safety_doc)]
// #![warn(clippy::unnecessary_self_imports)]
// #![warn(clippy::unneeded_field_pattern)]
// #![warn(clippy::unreachable)]
// #![warn(clippy::unseparated_literal_suffix)]
// #![warn(clippy::unwrap_in_result)]
// #![warn(clippy::unwrap_used)]
// #![warn(clippy::verbose_file_reads)]
// #![warn(clippy::wildcard_enum_match_arm)]

pub mod assertions;
pub mod errors;
pub mod utilities;

pub use test_ur_code_xd_macro::test_with_parameter_values;
