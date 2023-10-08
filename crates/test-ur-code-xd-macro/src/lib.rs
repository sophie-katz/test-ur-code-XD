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

mod errors;
mod parameters;
mod permute;

use crate::parameters::get_permuted_parameter_map_iter;
use parameters::generate_permuted_test_function;
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
    let tokens = proc_macro2::TokenStream::from(attribute);

    let vec_of_parameter_maps: Vec<HashMap<String, Expr>> =
        match get_permuted_parameter_map_iter(tokens) {
            Ok(iter_of_parameter_maps) => iter_of_parameter_maps.collect(),
            Err(error) => return error.into_compile_error().into(),
        };

    let item_fn = parse_macro_input!(item as ItemFn);

    generate_permuted_test_function(item_fn, vec_of_parameter_maps).into()
}
