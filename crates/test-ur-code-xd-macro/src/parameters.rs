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

pub mod extracting;
pub mod generating;
pub mod parsing;

use crate::{
    errors::Error,
    parameters::extracting::{filter_fn_attrs_without_this_macro, take_fn_attrs},
};

use self::{
    extracting::iter_parameterized_fn_inputs,
    generating::{generate_parameter_function, generate_permutation_function},
};

use super::permute::permute_map_of_vecs;
use extracting::get_map_of_parameter_vecs_from_expr_assign_iter;
use parsing::parse_expr_assign_iter;
use quote::quote_spanned;
use std::collections::HashMap;
use syn::{Attribute, Expr, ItemFn};

/// Gets an iterator over parameter maps from the token stream taken from a given attribute.
///
/// It takes a token stream from the attribute, loads the parameter values from it, and then
/// permutes them to get an iterator over every permutation of the parameter values.
///
/// # Example
///
/// ```ignore
/// get_permuted_parameter_map_iter(
///     quote! {
///         a = [1, 2],
///         b = [3, 4]
///     }
/// );
/// ```
///
/// This will result in an iterator over maps that look like this:
///
/// ```json
/// [
///     {
///         "a": 1,
///         "b": 3
///     },
///     {
///         "a": 2,
///         "b": 3
///     },
///     {
///         "a": 1,
///         "b": 4
///     },
///     {
///         "a": 2,
///         "b": 4
///     }
/// ]
/// ```
///
/// # Arguments
///
/// * `tokens` - a token stream taken from the attribute
///
/// # Returns
///
/// An iterator over parameter maps.
///
/// # Errors
///
/// * Returns a [`syn::Error`] if the token stream cannot be parsed as expected.
pub fn get_permuted_parameter_map_iter(
    tokens: proc_macro2::TokenStream,
) -> Result<impl Iterator<Item = HashMap<String, Expr>>, syn::Error> {
    let map_of_parameter_vecs =
        get_map_of_parameter_vecs_from_expr_assign_iter(parse_expr_assign_iter(tokens)?);
    Ok(permute_map_of_vecs(map_of_parameter_vecs).into_iter())
}

/// Generates a permutation function for a given test function and parameterization. This is the
/// top-level generation function that gets called by the macro.
///
/// It does not return any errors since the errors are embedded into the token stream using the
/// [`compile_error`] macro.
///
/// # Arguments
///
/// * `item` - The test case's original function.
/// * `vec_of_parameter_maps` - The vector of parameter maps parsed from the attribute.
///
/// # Returns
///
/// A token stream.
pub fn generate_permuted_test_function(
    mut item: ItemFn,
    vec_of_parameter_maps: Vec<HashMap<String, Expr>>,
) -> proc_macro2::TokenStream {
    // Take attribute list
    //
    // TODO: Do something with attributes
    let _attributes: Vec<Attribute> =
        filter_fn_attrs_without_this_macro(take_fn_attrs(&mut item)).collect();

    // Initialize token stream
    let mut result = proc_macro2::TokenStream::new();

    // For each permutation, generate a permutation function
    for (counter, parameter_map) in vec_of_parameter_maps.into_iter().enumerate() {
        // Initialize vector for parameterized function inputs
        let mut paramterized_fn_inputs = Vec::new();

        // Iterate over the parameterized function inputs and populate the vector, generating
        // compiler errors as needed
        for input in iter_parameterized_fn_inputs(&item, &parameter_map) {
            match input {
                Ok((name, ty, expr)) => {
                    paramterized_fn_inputs.push((name, ty.clone(), expr.clone()))
                }
                Err(error) => match error {
                    Error::ArgumentHasNoParameter(name) => {
                        result.extend(quote_spanned!(item.sig.ident.span() => {
                            compile_error!(concat!("argument `", #name, "` must also be defined in the attribute"));
                        }));
                        return result;
                    }
                    Error::SelfArgumentInTest => {
                        result.extend(quote_spanned!(item.sig.ident.span() => {
                            compile_error!(concat!("test functions cannot take `self` as an argument"));
                        }));
                        return result;
                    }
                },
            }
        }

        // Generate the permutation function
        result.extend(generate_permutation_function(
            &item,
            &paramterized_fn_inputs,
            counter as u32,
        ));
    }

    // Generate the parameter function
    result.extend(generate_parameter_function(item));

    // Return results
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{quote, ToTokens};

    #[test]
    fn get_permuted_parameter_map_iter_empty() {
        let vec_of_maps: Vec<HashMap<String, Expr>> = get_permuted_parameter_map_iter(quote! {})
            .unwrap()
            .collect();

        assert!(vec_of_maps.is_empty());
    }

    #[test]
    fn get_permuted_parameter_map_iter_one_empty() {
        let vec_of_maps: Vec<HashMap<String, Expr>> =
            get_permuted_parameter_map_iter(quote! {a = []})
                .unwrap()
                .collect();

        assert!(vec_of_maps.is_empty());
    }

    #[test]
    fn get_permuted_parameter_map_iter_one_full() {
        let vec_of_maps: Vec<HashMap<String, Expr>> =
            get_permuted_parameter_map_iter(quote! {a = [1, 2]})
                .unwrap()
                .collect();

        assert_eq!(vec_of_maps.len(), 2);
        assert_eq!(vec_of_maps[0].len(), 1);
        assert_eq!(vec_of_maps[0]["a"].to_token_stream().to_string(), "1");
        assert_eq!(vec_of_maps[1].len(), 1);
        assert_eq!(vec_of_maps[1]["a"].to_token_stream().to_string(), "2");
    }

    #[test]
    fn get_permuted_parameter_map_iter_two_full() {
        let vec_of_maps: Vec<HashMap<String, Expr>> =
            get_permuted_parameter_map_iter(quote! {a = [1, 2], b = [3, 4]})
                .unwrap()
                .collect();

        assert_eq!(vec_of_maps.len(), 4);
        assert_eq!(vec_of_maps[0].len(), 2);
        assert_eq!(vec_of_maps[1].len(), 2);
        assert_eq!(vec_of_maps[2].len(), 2);
        assert_eq!(vec_of_maps[3].len(), 2);
    }
}
