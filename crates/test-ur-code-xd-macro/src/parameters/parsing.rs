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

//! Parsing functions

use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, ExprAssign, Token};

/// Parses a sequence of assignment expressions.
///
/// # Arguments
///
/// * `tokens` - The token stream to parse.
///
/// # Returns
///
/// An iterator of of [`ExprAssign`] instances.
///
/// # Errors
///
/// * Returns a [`syn::Error`] if the token stream cannot be parsed as expected.
pub fn parse_expr_assign_iter(
    tokens: TokenStream,
) -> Result<impl Iterator<Item = ExprAssign>, syn::Error> {
    Ok(Parser::parse2(
        Punctuated::<ExprAssign, Token![,]>::parse_terminated,
        tokens,
    )?
    .into_iter())
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn parse_expr_assign_iter_empty() {
        let expressions: Vec<ExprAssign> = parse_expr_assign_iter(quote! {}).unwrap().collect();

        assert!(expressions.is_empty());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::indexing_slicing)]
    fn parse_expr_assign_iter_one_empty() {
        let expressions: Vec<ExprAssign> =
            parse_expr_assign_iter(quote! { a = [] }).unwrap().collect();

        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0].left.to_token_stream().to_string(), "a");
        assert_eq!(expressions[0].right.to_token_stream().to_string(), "[]");
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::indexing_slicing)]
    fn parse_expr_assign_iter_one_full() {
        let expressions: Vec<ExprAssign> = parse_expr_assign_iter(quote! { a = [1, 2, 3] })
            .unwrap()
            .collect();

        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0].left.to_token_stream().to_string(), "a");
        assert_eq!(
            expressions[0].right.to_token_stream().to_string(),
            "[1 , 2 , 3]"
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::indexing_slicing)]
    fn parse_expr_assign_iter_two_full() {
        let expressions: Vec<ExprAssign> =
            parse_expr_assign_iter(quote! { a = [1, 2, 3], b = [4, 5, 6] })
                .unwrap()
                .collect();

        assert_eq!(expressions.len(), 2);
        assert_eq!(expressions[0].left.to_token_stream().to_string(), "a");
        assert_eq!(
            expressions[0].right.to_token_stream().to_string(),
            "[1 , 2 , 3]"
        );
        assert_eq!(expressions[1].left.to_token_stream().to_string(), "b");
        assert_eq!(
            expressions[1].right.to_token_stream().to_string(),
            "[4 , 5 , 6]"
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::indexing_slicing)]
    fn parse_expr_assign_iter_comma_after() {
        let expressions: Vec<ExprAssign> =
            parse_expr_assign_iter(quote! { a = [1, 2, 3], b = [4, 5, 6], })
                .unwrap()
                .collect();

        assert_eq!(expressions.len(), 2);
        assert_eq!(expressions[0].left.to_token_stream().to_string(), "a");
        assert_eq!(
            expressions[0].right.to_token_stream().to_string(),
            "[1 , 2 , 3]"
        );
        assert_eq!(expressions[1].left.to_token_stream().to_string(), "b");
        assert_eq!(
            expressions[1].right.to_token_stream().to_string(),
            "[4 , 5 , 6]"
        );
    }

    #[test]
    fn parse_expr_assign_iter_two_commas_after() {
        assert!(parse_expr_assign_iter(quote! { a = [1, 2, 3], b = [4, 5, 6],, }).is_err());
    }

    #[test]
    fn parse_expr_assign_iter_no_commas() {
        assert!(parse_expr_assign_iter(quote! { a = [1, 2, 3] b = [4, 5, 6] }).is_err());
    }
}
