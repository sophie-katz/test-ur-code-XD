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

//! The procedural macro crate for
//! [test ur code XD](https://github.com/sophie-katz/test-ur-code-XD).
//!
//! See the [user guide](https://sophie-katz.github.io/test-ur-code-XD/) for more information about
//! how to use this crate.

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

mod errors;
mod parameters;
mod permute;

use crate::parameters::get_permuted_parameter_map_iter;
use parameters::{generate_permuted_test_function, get_max_permutation_count};
use std::collections::HashMap;
use syn::{parse_macro_input, Expr, ItemFn};

/// Permutes a test case.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/tests/parameterized-tests](https://sophie-katz.github.io/test-ur-code-XD/tests/parameterized-tests/)
/// for a usage guide.
#[proc_macro_attribute]
pub fn test_with_parameter_values(
    attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Convert attribute token stream into proc_macro2 tokens
    let tokens = proc_macro2::TokenStream::from(attribute);

    // Parse the function item
    let item_fn = parse_macro_input!(item as ItemFn);

    // Parse the attribute's parameters into a vector of permuted parameter maps
    let vector_of_parameter_maps: Vec<HashMap<String, Expr>> =
        match get_permuted_parameter_map_iter(tokens, get_max_permutation_count()) {
            Ok(iter_of_parameter_maps) => iter_of_parameter_maps.collect(),
            Err(error) => {
                return error.into_compile_error().into();
            }
        };

    // Generate the permuted test function
    match generate_permuted_test_function(item_fn, vector_of_parameter_maps) {
        Ok(generated) => generated.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
