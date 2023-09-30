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

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parser, punctuated::Punctuated, Expr, ExprAssign, FnArg, ItemFn, Pat, Token, Type,
};

use super::permute::permute_maps;

fn parse_attribute(attribute: TokenStream) -> Vec<ExprAssign> {
    Parser::parse(
        Punctuated::<ExprAssign, Token![,]>::parse_terminated,
        attribute,
    )
    .unwrap()
    .into_iter()
    .collect::<Vec<ExprAssign>>()
}

fn get_identifier_name_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(path) => {
            if path.path.segments.len() == 1 {
                Some(path.path.segments[0].ident.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_expr_vec_from_array(expr: &Expr) -> Option<Vec<Expr>> {
    match expr {
        Expr::Array(array) => Some(array.elems.iter().cloned().collect()),
        _ => None,
    }
}

fn get_unpermuted_parameters_from_assign_vec(
    assign_vec: Vec<ExprAssign>,
) -> HashMap<String, Vec<Expr>> {
    assign_vec
        .into_iter()
        .map(|assign| {
            (
                get_identifier_name_from_expr(&assign.left)
                    .expect("expected left hand side of assignment to be identifier"),
                get_expr_vec_from_array(&assign.right)
                    .expect("expected right hand side of assignment to be array"),
            )
        })
        .collect::<HashMap<String, Vec<Expr>>>()
}

pub fn get_paraneterizations(attribute: TokenStream) -> Vec<HashMap<String, Expr>> {
    let assign_vec = parse_attribute(attribute);
    let unpermuted_parameters = get_unpermuted_parameters_from_assign_vec(assign_vec);
    permute_maps(unpermuted_parameters)
}

fn get_identifier_name_from_pat(pat: &Pat) -> Option<String> {
    match pat {
        Pat::Ident(ident) => Some(ident.ident.to_string()),
        _ => None,
    }
}

fn get_let_expressions_from_inputs(
    item: &ItemFn,
    parameterization: HashMap<String, Expr>,
) -> Vec<(String, &Box<Type>, Expr)> {
    item.sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Typed(pat_type) => {
                let identifier_name = get_identifier_name_from_pat(&pat_type.pat)
                    .expect("expected argument pattern to be simple identifier");

                let expression = parameterization
                    .get(&identifier_name)
                    .expect("unexpected parameter name");

                (identifier_name, &pat_type.ty, expression.clone())
            }
            _ => panic!("unexpected 'self' argument"),
        })
        .collect()
}

pub fn parameterize_test_function(
    mut item: ItemFn,
    parameterizations: Vec<HashMap<String, Expr>>,
) -> proc_macro2::TokenStream {
    let mut attrs = Vec::new();
    std::mem::swap(&mut item.attrs, &mut attrs);

    let original_ident = item.sig.ident.clone();
    let with_parameters_ident = format_ident!("_{}_with_parameters", original_ident);

    item.sig.ident = with_parameters_ident.clone();

    let mut result = quote! {
        #item
    };

    for (counter, parameterization) in parameterizations.into_iter().enumerate() {
        let ident = format_ident!("{}_{}", original_ident, counter);

        let let_expressions = get_let_expressions_from_inputs(&item, parameterization);

        let let_expression_idents_0 = let_expressions
            .iter()
            .map(|(ident, _, _)| format_ident!("{}", ident));
        let let_expression_idents_1 = let_expressions
            .iter()
            .map(|(ident, _, _)| format_ident!("{}", ident));
        let let_expression_types = let_expressions.iter().map(|(_, r#type, _)| &**r#type);
        let let_expression_values = let_expressions.iter().map(|(_, _, expr)| expr);

        result.extend(quote! {
            #[test]
            fn #ident () {
                #(let #let_expression_idents_0: #let_expression_types = #let_expression_values;)*

                #with_parameters_ident ( #( #let_expression_idents_1 ),* );
            }
        });
    }

    println!("result: {}", result);

    result
}
