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

//! Error types for test ur code XD macros.

use proc_macro2::Span;
use quote::quote_spanned;
use syn::{spanned::Spanned, Expr, FnArg, PatType, Receiver};
use thiserror::Error;

/// A general error type for test ur code XD.
#[derive(Error, Debug)]
pub enum TestUrCodeXDMacroError {
    /// An error that occurs when parsing a test function that has a `self` argument.
    ///
    /// Test functions should never be struct members.
    #[error("unexpected `self` argument in test function")]
    SelfArgumentInTest(Receiver),

    /// If a parameterized test function has an argument that is not parameterized, this error is
    /// emitted.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// # use test_ur_code_xd_macro as test_ur_code_xd;
    /// #
    /// #[test_ur_code_xd::test_with_parameter_values(
    ///     x = [1, 2]
    /// )]
    /// fn example(y: i32) {
    ///     // `y` is not in the parameter list above
    ///     assert_eq!(y, 1);
    /// }
    /// ```
    #[error("argument has no parameter set")]
    ArgumentHasNoParameter(PatType),

    /// An error that occurs when the left-hand side of a parameter is not a simple identifier.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// # use test_ur_code_xd_macro as test_ur_code_xd;
    /// #
    /// #[test_ur_code_xd::test_with_parameter_values(
    ///     (x, y) = [1, 2]
    /// )]
    /// fn example(x: i32, y: i32) {
    ///     // ...
    /// }
    /// ```
    #[error("parameter's assignment left hand side is not an identifier")]
    ParameterAssignmentLeftHandSideIsNotIdentifier(Expr),

    /// An error that occurs when the right-hand side of a parameter is not an array literal.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// # use test_ur_code_xd_macro as test_ur_code_xd;
    /// #
    /// #[test_ur_code_xd::test_with_parameter_values(
    ///     (x, y) = [1, 2]
    /// )]
    /// fn example(x: i32, y: i32) {
    ///     // ...
    /// }
    /// ```
    #[error("parameter's assignment right hand side is not an array literal")]
    ParameterAssignmentRightHandSideIsNotArrayLiteral(Expr),

    /// An error that occurs when the pattern of a function argument is not a single identifier.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// # use test_ur_code_xd_macro as test_ur_code_xd;
    /// #
    /// #[test_ur_code_xd::test_with_parameter_values(
    ///     x = [1, 2]
    /// )]
    /// fn example((x, _): (i32, i32)) {
    ///     // ...
    /// }
    /// ```
    #[error("parameter is not a single identifier")]
    ArgumentPatternIsNotSingleIdentifier(FnArg),

    /// Wrapper for [`syn`] parsing errors.
    #[error("parsing error: {0}")]
    ParsingError(#[from] syn::Error),

    /// Emitted when too many permutations are generated for a parameterized test.
    ///
    /// This count is [`DEFAULT_MAX_PERMUTATION_COUNT`] by default, but can be overridden with the
    /// `TEST_UR_CODE_XD_MAX_PERMUTATION_COUNT` environment variable.
    #[error("too many permutations generated for parameterized test (limit is {limit}, but {actual} permutations were generated)")]
    TooManyPermutations {
        /// The span to use for the compile-time error
        span: Span,

        /// The maximum number of permutations allowed
        limit: usize,

        /// The actual number of permutations generated
        actual: usize,
    },
}

impl TestUrCodeXDMacroError {
    /// Converts the error into a [`proc_macro2::TokenStream`] that can be used with
    /// [`compile_error`].
    pub fn to_compile_error(&self) -> proc_macro2::TokenStream {
        match self {
            Self::SelfArgumentInTest(receiver) => {
                quote_spanned!(receiver.span() => compile_error!("test functions cannot have `self` as an argument"))
            }
            Self::ArgumentHasNoParameter(argument_pattern) => {
                quote_spanned!(argument_pattern.span() => compile_error!("no parameterization exists for argument"))
            }
            Self::ParameterAssignmentLeftHandSideIsNotIdentifier(expr) => {
                quote_spanned!(expr.span() => compile_error!("parameter's left-hand side must be an identifier"))
            }
            Self::ParameterAssignmentRightHandSideIsNotArrayLiteral(expr) => {
                quote_spanned!(expr.span() => compile_error!("parameter's right-hand side must be an array literal"))
            }
            Self::ArgumentPatternIsNotSingleIdentifier(fn_arg) => {
                quote_spanned!(fn_arg.span() => compile_error!("argument must be a single identifier, not a pattern"))
            }
            Self::ParsingError(error) => error.to_compile_error(),
            Self::TooManyPermutations { span, .. } => {
                quote_spanned!(*span => compile_error!("too many permutations generated for parameterized test"))
            }
        }
    }

    /// Converts the error into a [`proc_macro2::TokenStream`] that can be used with
    /// [`compile_error`].
    pub fn into_compile_error(self) -> proc_macro2::TokenStream {
        self.to_compile_error()
    }
}
