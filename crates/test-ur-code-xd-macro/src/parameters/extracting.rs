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

use std::{collections::HashMap, mem};

use syn::{Attribute, Expr, ExprAssign, FnArg, ItemFn, Meta, Pat, Type};

use crate::errors::Error;

/// Extracts an identifier name from an identifier expression.
///
/// # Example
///
/// ```ignore
/// assert_eq!(
///     get_identifier_name_from_expr(&parse_quote! { a }).unwrap(),
///     "a"
/// );
/// ```
///
/// # Returns
///
/// * `Some(identifier)` if the expression is an identifier.
/// * `None` otherwise.
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

/// Extracts an identifier name from a pattern.
///
/// # Example
///
/// ```ignore
/// assert_eq!(
///     get_identifier_name_from_pat(&parse_quote! { a }).unwrap(),
///     "a"
/// );
/// ```
///
/// # Returns
///
/// * `Some(identifier)` if the expression is an identifier.
/// * `None` otherwise.
pub fn get_identifier_name_from_pat(pat: &Pat) -> Option<String> {
    match pat {
        Pat::Ident(ident) => Some(ident.ident.to_string()),
        _ => None,
    }
}

/// Extracts an iterator of expressions from an array literal expression.
///
/// # Example
///
/// ```ignore
/// assert_eq!(
///     iter_expr_literal_array(&parse_quote! { [1, 2, 3] }).unwrap(),
///     vec![
///         parse_quote! { 1 },
///         parse_quote! { 2 },
///         parse_quote! { 3 }
///     ],
/// );
/// ```
///
/// # Returns
///
/// * `Some(expressions)` if the expression is an array literal.
/// * `None` otherwise.
fn iter_expr_literal_array(expr: &Expr) -> Option<impl Iterator<Item = &Expr>> {
    match expr {
        Expr::Array(array) => Some(array.elems.iter()),
        _ => None,
    }
}

/// Extracts a parameter map from an expression vector.
///
/// # Example
///
/// ```ignore
/// assert_eq!(
///     get_map_of_parameter_vecs_from_expr_vec(
///         vec![
///             parse_quote! { a = [1, 2, 3] },
///             parse_quote! { b = [4, 5, 6] },
///         ]
///     ),
///     HashMap::from([
///         (
///             "a".to_owned(),
///             vec![
///                 parse_quote! { 1 },
///                 parse_quote! { 2 },
///                 parse_quote! { 3 },
///             ]
///         ),
///         (
///             "b".to_owned(),
///             vec![
///                 parse_quote! { 4 },
///                 parse_quote! { 5 },
///                 parse_quote! { 6 },
///             ]
///         ),
///     ])
/// );
/// ```
///
/// # Arguments
///
/// * `expr_assign_iter` - An iterator of assignment expressions. This can be taken from
///   [`iter_expr_literal_array`].
///
/// # Returns
///
/// A hash map of parameter names to expression vectors.
pub fn get_map_of_parameter_vecs_from_expr_assign_iter(
    expr_assign_iter: impl Iterator<Item = ExprAssign>,
) -> HashMap<String, Vec<Expr>> {
    expr_assign_iter
        .map(|assign| {
            (
                get_identifier_name_from_expr(&assign.left)
                    .expect("expected left hand side of assignment to be identifier"),
                iter_expr_literal_array(&assign.right)
                    .expect("expected right hand side of assignment to be array")
                    .cloned()
                    .collect::<Vec<Expr>>(),
            )
        })
        .collect::<HashMap<String, Vec<Expr>>>()
}

/// Iterates over the names and types of function arguments.
///
/// # Example
///
/// ```ignore
/// let iter = iter_fn_inputs(
///     &parse_quote! {
///         fn test(a: i32, b: String) {}
///     }
/// );
/// ```
///
/// # Arguments
///
/// * `item` - The function whose inputs to iterate over.
///
/// # Returns
///
/// An iterator of tuples of the argument name and type.
///
/// # Errors
///
/// * Returns a [`Error::SelfArgumentInTest`] if the function has a `self` argument.
fn iter_fn_inputs(item: &ItemFn) -> impl Iterator<Item = Result<(String, &Type), Error>> + '_ {
    item.sig.inputs.iter().map(|input| match input {
        FnArg::Typed(pat_type) => {
            let identifier_name = get_identifier_name_from_pat(&pat_type.pat)
                .expect("expected argument pattern to be simple identifier");

            Ok((identifier_name, &(*pat_type.ty)))
        }
        _ => Err(Error::SelfArgumentInTest),
    })
}

/// Iterates over the names and types of function arguments along with their parameterized values.
///
/// # Example
///
/// ```ignore
/// let iter = iter_parameterized_fn_inputs(
///     &parse_quote! {
///         fn test(a: i32, b: String) {}
///     },
///     &HashMap::from([
///         (
///             "a".to_owned(),
///             parse_quote! { [1, 2, 3] }
///         ),
///         (
///             "b".to_owned(),
///             parse_quote! { ["x".to_owned(), "y".to_owned()] }
///         )
///     ])
/// );
/// ```
///
/// # Arguments
///
/// * `item` - The function whose inputs to iterate over.
/// * `parameter_map` - A map of parameter names to parameterized values.
///
/// # Returns
///
/// An iterator of tuples of the argument name and type.
///
/// # Errors
///
/// * Returns a [`Error::SelfArgumentInTest`] if the function has a `self` argument.
/// * Returns a [`Error::ArgumentHasNoParameter`] if an argument has no parameter.
pub fn iter_parameterized_fn_inputs<'item, 'parameter_map>(
    item: &'item ItemFn,
    parameter_map: &'parameter_map HashMap<String, Expr>,
) -> impl Iterator<Item = Result<(String, &'item Type, &'parameter_map Expr), Error>> {
    iter_fn_inputs(item).map(|input| match input {
        Ok((name, ty)) => {
            let expression = parameter_map
                .get(&name)
                .ok_or_else(|| Error::ArgumentHasNoParameter(name.clone()))?;

            Ok((name, ty, expression))
        }
        Err(error) => Err(error),
    })
}

/// Removes the attributes for a function and returns them as an iterator.
pub fn take_fn_attrs(item: &mut ItemFn) -> impl Iterator<Item = Attribute> {
    let mut attrs = Vec::new();

    mem::swap(&mut item.attrs, &mut attrs);

    attrs.into_iter()
}

/// Filters attributes to omit the `#[test_with_parameter_values]` attribute.
pub fn filter_fn_attrs_without_this_macro(
    attribute: impl Iterator<Item = Attribute>,
) -> impl Iterator<Item = Attribute> {
    attribute.filter(|attribute| match &attribute.meta {
        Meta::List(meta_list) => !meta_list
            .path
            .is_ident(stringify!(test_with_parameter_values)),
        _ => true,
    })
}

#[cfg(test)]
mod tests {
    use std::iter;

    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn get_identifier_name_from_expr_path() {
        assert_eq!(
            get_identifier_name_from_expr(&parse_quote! { a }).unwrap(),
            "a"
        );
    }

    #[test]
    fn get_identifier_name_from_expr_not_path() {
        assert!(get_identifier_name_from_expr(&parse_quote! { 5 }).is_none());
    }

    #[test]
    fn get_identifier_name_from_expr_nested_path() {
        assert!(get_identifier_name_from_expr(&parse_quote! { a + b }).is_none());
    }

    #[test]
    fn get_identifier_name_from_pat_pat() {
        assert_eq!(
            get_identifier_name_from_pat(&parse_quote! { a }).unwrap(),
            "a"
        );
    }

    #[test]
    fn get_identifier_name_from_pat_nested() {
        assert!(get_identifier_name_from_pat(&parse_quote! { (a, b) }).is_none());
    }

    #[test]
    fn get_expr_vec_from_array_array_full() {
        let expressions: Vec<Expr> = iter_expr_literal_array(&parse_quote! { [1, 2, 3] })
            .unwrap()
            .cloned()
            .collect();

        assert_eq!(expressions.len(), 3);
        assert_eq!(expressions[0].to_token_stream().to_string(), "1");
        assert_eq!(expressions[1].to_token_stream().to_string(), "2");
        assert_eq!(expressions[2].to_token_stream().to_string(), "3");
    }

    #[test]
    fn get_expr_vec_from_array_array_empty() {
        assert_eq!(
            iter_expr_literal_array(&parse_quote! { [] })
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn get_expr_vec_from_array_array_non_array() {
        assert!(iter_expr_literal_array(&parse_quote! { 1 }).is_none());
    }

    #[test]
    fn get_map_of_parameter_vecs_from_expr_assign_iter_empty() {
        let map = get_map_of_parameter_vecs_from_expr_assign_iter(vec![].into_iter());

        assert!(map.is_empty());
    }

    #[test]
    fn get_map_of_parameter_vecs_from_expr_assign_iter_one_empty() {
        let map = get_map_of_parameter_vecs_from_expr_assign_iter(
            vec![parse_quote! { a = [] }].into_iter(),
        );

        assert_eq!(map.len(), 1);
        assert!(map["a"].is_empty());
    }

    #[test]
    fn get_map_of_parameter_vecs_from_expr_assign_iter_one_full() {
        let map = get_map_of_parameter_vecs_from_expr_assign_iter(
            vec![parse_quote! { a = [1, 2, 3] }].into_iter(),
        );

        assert_eq!(map.len(), 1);
        assert_eq!(map["a"].len(), 3);
        assert_eq!(map["a"][0].to_token_stream().to_string(), "1");
        assert_eq!(map["a"][1].to_token_stream().to_string(), "2");
        assert_eq!(map["a"][2].to_token_stream().to_string(), "3");
    }

    #[test]
    fn get_map_of_parameter_vecs_from_expr_assign_iter_two_full() {
        let map = get_map_of_parameter_vecs_from_expr_assign_iter(
            vec![
                parse_quote! { a = [1, 2, 3] },
                parse_quote! { b = [4, 5, 6] },
            ]
            .into_iter(),
        );

        assert_eq!(map.len(), 2);
        assert_eq!(map["a"].len(), 3);
        assert_eq!(map["a"][0].to_token_stream().to_string(), "1");
        assert_eq!(map["a"][1].to_token_stream().to_string(), "2");
        assert_eq!(map["a"][2].to_token_stream().to_string(), "3");
        assert_eq!(map["b"].len(), 3);
        assert_eq!(map["b"][0].to_token_stream().to_string(), "4");
        assert_eq!(map["b"][1].to_token_stream().to_string(), "5");
        assert_eq!(map["b"][2].to_token_stream().to_string(), "6");
    }

    #[test]
    fn iter_fn_inputs_empty() {
        let item = parse_quote! {
            fn test() {}
        };

        let inputs = iter_fn_inputs(&item)
            .map(|input| match input {
                Ok((name, ty)) => (name, ty.clone()),
                Err(error) => panic!("error in iter_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type)>>();

        assert!(inputs.is_empty());
    }

    #[test]
    fn iter_fn_inputs_one() {
        let item = parse_quote! {
            fn test(a: i32) {}
        };

        let inputs = iter_fn_inputs(&item)
            .map(|input| match input {
                Ok((name, ty)) => (name, ty.clone()),
                Err(error) => panic!("error in iter_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type)>>();

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].0, "a");
        assert_eq!(inputs[0].1.to_token_stream().to_string(), "i32");
    }

    #[test]
    fn iter_fn_inputs_two() {
        let item = parse_quote! {
            fn test(a: i32, b: String) {}
        };

        let inputs = iter_fn_inputs(&item)
            .map(|input| match input {
                Ok((name, ty)) => (name, ty.clone()),
                Err(error) => panic!("error in iter_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type)>>();

        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].0, "a");
        assert_eq!(inputs[0].1.to_token_stream().to_string(), "i32");
        assert_eq!(inputs[1].0, "b");
        assert_eq!(inputs[1].1.to_token_stream().to_string(), "String");
    }

    #[test]
    fn iter_fn_inputs_self() {
        let item = parse_quote! {
            fn test(self, a: i32) {}
        };

        let inputs = iter_fn_inputs(&item).collect::<Vec<Result<(String, &Type), Error>>>();

        assert_eq!(inputs.len(), 2);
        assert!(inputs[0].is_err());
        assert!(inputs[1].is_ok());
        assert_eq!(inputs[1].as_ref().unwrap().0, "a");
        assert_eq!(
            inputs[1].as_ref().unwrap().1.to_token_stream().to_string(),
            "i32"
        );
    }

    #[test]
    fn iter_parameterized_fn_inputs_empty() {
        let item = parse_quote! {
            fn test() {}
        };

        let parameter_map = HashMap::new();

        let inputs = iter_parameterized_fn_inputs(&item, &parameter_map)
            .map(|input| match input {
                Ok((name, ty, expr)) => (name, ty.clone(), expr.clone()),
                Err(error) => panic!("error in iter_parameterized_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type, Expr)>>();

        assert!(inputs.is_empty());
    }

    #[test]
    fn iter_parameterized_fn_inputs_one() {
        let item = parse_quote! {
            fn test(a: i32) {}
        };

        let parameter_map = HashMap::from([("a".to_owned(), parse_quote! { 1 })]);

        let inputs = iter_parameterized_fn_inputs(&item, &parameter_map)
            .map(|input| match input {
                Ok((name, ty, expr)) => (name, ty.clone(), expr.clone()),
                Err(error) => panic!("error in iter_parameterized_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type, Expr)>>();

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].0, "a");
        assert_eq!(inputs[0].1.to_token_stream().to_string(), "i32");
        assert_eq!(inputs[0].2.to_token_stream().to_string(), "1");
    }

    #[test]
    fn iter_parameterized_fn_inputs_two() {
        let item = parse_quote! {
            fn test(a: i32, b: String) {}
        };

        let parameter_map = HashMap::from([
            ("a".to_owned(), parse_quote! { 1 }),
            ("b".to_owned(), parse_quote! { "x".to_owned() }),
        ]);

        let inputs = iter_parameterized_fn_inputs(&item, &parameter_map)
            .map(|input| match input {
                Ok((name, ty, expr)) => (name, ty.clone(), expr.clone()),
                Err(error) => panic!("error in iter_parameterized_fn_inputs: {}", error),
            })
            .collect::<Vec<(String, Type, Expr)>>();

        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].0, "a");
        assert_eq!(inputs[0].1.to_token_stream().to_string(), "i32");
        assert_eq!(inputs[0].2.to_token_stream().to_string(), "1");
        assert_eq!(inputs[1].0, "b");
        assert_eq!(inputs[1].1.to_token_stream().to_string(), "String");
        assert_eq!(
            inputs[1].2.to_token_stream().to_string(),
            "\"x\" . to_owned ()"
        );
    }

    #[test]
    fn iter_parameterized_fn_inputs_self() {
        let item = parse_quote! {
            fn test(self, a: i32) {}
        };

        let parameter_map = HashMap::from([("a".to_owned(), parse_quote! { 1 })]);

        let inputs = iter_parameterized_fn_inputs(&item, &parameter_map)
            .collect::<Vec<Result<(String, &Type, &Expr), Error>>>();

        assert_eq!(inputs.len(), 2);
        assert!(inputs[0].is_err());
        assert!(inputs[1].is_ok());
        assert_eq!(inputs[1].as_ref().unwrap().0, "a");
        assert_eq!(
            inputs[1].as_ref().unwrap().1.to_token_stream().to_string(),
            "i32"
        );
        assert_eq!(
            inputs[1].as_ref().unwrap().2.to_token_stream().to_string(),
            "1"
        );
    }

    #[test]
    fn iter_parameterized_fn_inputs_missing() {
        let item = parse_quote! {
            fn test(a: i32) {}
        };

        let parameter_map = HashMap::new();

        let inputs = iter_parameterized_fn_inputs(&item, &parameter_map)
            .collect::<Vec<Result<(String, &Type, &Expr), Error>>>();

        assert_eq!(inputs.len(), 1);
        assert!(inputs[0].is_err());
    }

    #[test]
    fn take_fn_attrs_none() {
        let mut item = parse_quote! {
            fn test() {}
        };

        let attributes: Vec<Attribute> = take_fn_attrs(&mut item).collect();

        assert!(attributes.is_empty());

        assert_eq!(item.to_token_stream().to_string(), "fn test () { }");
    }

    #[test]
    fn take_fn_attrs_one() {
        let mut item = parse_quote! {
            #[doc(hidden)]
            fn test() {}
        };

        let attributes: Vec<Attribute> = take_fn_attrs(&mut item).collect();

        assert_eq!(attributes.len(), 1);
        assert_eq!(
            attributes[0].to_token_stream().to_string(),
            "# [doc (hidden)]"
        );

        assert_eq!(item.to_token_stream().to_string(), "fn test () { }");
    }

    #[test]
    fn take_fn_attrs_two() {
        let mut item = parse_quote! {
            #[doc(hidden)]
            #[cfg(target_family = "unix")]
            fn test() {}
        };

        let attributes: Vec<Attribute> = take_fn_attrs(&mut item).collect();

        assert_eq!(attributes.len(), 2);
        assert_eq!(
            attributes[0].to_token_stream().to_string(),
            "# [doc (hidden)]"
        );
        assert_eq!(
            attributes[1].to_token_stream().to_string(),
            "# [cfg (target_family = \"unix\")]"
        );

        assert_eq!(item.to_token_stream().to_string(), "fn test () { }");
    }

    #[test]
    fn filter_fn_attrs_without_this_macro_empty() {
        let attributes: Vec<Attribute> =
            filter_fn_attrs_without_this_macro(iter::empty()).collect();

        assert!(attributes.is_empty());
    }

    #[test]
    fn filter_fn_attrs_without_this_macro_two() {
        let attributes: Vec<Attribute> = filter_fn_attrs_without_this_macro(
            vec![
                parse_quote! { #[doc(hidden)] },
                parse_quote! { #[cfg(target_family = "unix")] },
            ]
            .into_iter(),
        )
        .collect();

        assert_eq!(attributes.len(), 2);
    }

    #[test]
    fn filter_fn_attrs_without_this_macro_two_with_this_macro() {
        let attributes: Vec<Attribute> = filter_fn_attrs_without_this_macro(
            vec![
                parse_quote! { #[doc(hidden)] },
                parse_quote! { #[test_with_parameter_values()] },
            ]
            .into_iter(),
        )
        .collect();

        assert_eq!(attributes.len(), 1);
        assert_eq!(
            attributes[0].to_token_stream().to_string(),
            "# [doc (hidden)]"
        );
    }
}
