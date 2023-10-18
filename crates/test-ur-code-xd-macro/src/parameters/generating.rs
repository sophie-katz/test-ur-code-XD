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

//! Utility functions for generating token stream.
//!
//! A single function that is decorated with the `#[test_with_parameter_values]` attribute gets
//! broken down into multiple functions after evaluation of the macro:
//!
//! * **Parameter function** - The original function that the attribute has been applied to, but
//!                            renamed for clarity. This is the function that gets called for every
//!                            permutation of the parameters.
//! * **Permutation functions** - The set of functions for every permutation of the parameters.
//!                               These take no arguments, but have the `#[test]` attribute while
//!                               the parameter function does not.

use quote::{format_ident, quote};
use syn::{Attribute, Expr, Ident, ItemFn, Type};

/// Creates an identifier for the parameter function.
///
/// # Arguments
///
/// * `item` - The test case's original function.
///
/// # Returns
///
/// The identifier to be used for the parameter function.
#[must_use]
pub fn get_parameter_function_ident(item: &ItemFn) -> Ident {
    format_ident!("_test_ur_code_xd_{}_parameter_function", item.sig.ident)
}

/// Generates the parameter function for a given test function.
#[must_use]
pub fn generate_parameter_function(mut item: ItemFn) -> proc_macro2::TokenStream {
    item.sig.ident = get_parameter_function_ident(&item);

    quote! {
        #item
    }
}

/// Generates a permutation function for a given test function and parameterization.
///
/// # Arguments
///
/// * `item` - The test case's original function.
/// * `parameterization` - The parameterization to use for the permutation function.
/// * `index` - An integer index used to differentiate the permutations.
#[must_use]
pub fn generate_permutation_function(
    attributes: &[Attribute],
    item: &ItemFn,
    parameterized_fn_inputs: &[(String, Type, Expr)],
    index: usize,
) -> proc_macro2::TokenStream {
    // Generate test function identifier
    let test_function_ident = format_ident!("{}_{}", item.sig.ident, index);

    // Get test function with parameters identifier
    let test_function_with_parameters_ident = get_parameter_function_ident(item);

    // Generate let expression iterators
    let let_expression_identifiers: Vec<Ident> = parameterized_fn_inputs
        .iter()
        .map(|(ident, _, _)| format_ident!("{}", ident))
        .collect();

    let let_expression_types = parameterized_fn_inputs.iter().map(|(_, ty, _)| ty);

    let let_expression_values = parameterized_fn_inputs.iter().map(|(_, _, expr)| expr);

    // Generate token stream
    quote! {
        #[test]
        #( #attributes )*
        fn #test_function_ident () {
            #(let #let_expression_identifiers: #let_expression_types = #let_expression_values;)*

            #test_function_with_parameters_ident ( #( #let_expression_identifiers ),* );
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn get_parameter_function_ident_simple() {
        let item = parse_quote! {
            fn asdf() {}
        };

        let token_stream = generate_parameter_function(item);

        assert_eq!(
            token_stream.to_string(),
            "fn _test_ur_code_xd_asdf_parameter_function () { }"
        );
    }

    #[test]
    fn generate_parameter_function_simple() {
        let item = parse_quote! {
            #[doc(hidden)]
            fn asdf() {
                println!("hi");
            }
        };

        let parameter_function = generate_parameter_function(item);

        assert_eq!(
            parameter_function.to_token_stream().to_string(),
            "# [doc (hidden)] fn _test_ur_code_xd_asdf_parameter_function () { println ! (\"hi\") ; }"
        );
    }

    #[test]
    fn generate_permutation_function_empty() {
        let attributes = Vec::new();

        let item = parse_quote! {
            fn asdf() {}
        };

        let permutation_function =
            generate_permutation_function(&attributes, &item, &Vec::new(), 0);

        assert_eq!(
            permutation_function.to_token_stream().to_string(),
            "# [test] fn asdf_0 () { _test_ur_code_xd_asdf_parameter_function () ; }"
        );
    }

    #[test]
    fn generate_permutation_function_two() {
        let attributes = Vec::new();

        let item = parse_quote! {
            fn asdf(a: u32, b: u32) {
                assert_eq!(a, b);
            }
        };

        let permutation_function = generate_permutation_function(
            &attributes,
            &item,
            &vec![
                ("a".to_owned(), parse_quote! { u32 }, parse_quote! { 1 }),
                ("b".to_owned(), parse_quote! { u32 }, parse_quote! { 2 }),
            ],
            0,
        );

        assert_eq!(permutation_function.to_token_stream().to_string(), "# [test] fn asdf_0 () { let a : u32 = 1 ; let b : u32 = 2 ; _test_ur_code_xd_asdf_parameter_function (a , b) ; }");
    }

    #[test]
    fn generate_permutation_function_attributes() {
        let attributes = vec![parse_quote! { #[doc(hidden)] }, parse_quote! { #[ignore] }];

        let item = parse_quote! {
            fn asdf(a: u32, b: u32) {
                assert_eq!(a, b);
            }
        };

        let permutation_function = generate_permutation_function(
            &attributes,
            &item,
            &vec![
                ("a".to_owned(), parse_quote! { u32 }, parse_quote! { 1 }),
                ("b".to_owned(), parse_quote! { u32 }, parse_quote! { 2 }),
            ],
            0,
        );

        assert_eq!(permutation_function.to_token_stream().to_string(), "# [test] # [doc (hidden)] # [ignore] fn asdf_0 () { let a : u32 = 1 ; let b : u32 = 2 ; _test_ur_code_xd_asdf_parameter_function (a , b) ; }");
    }
}
