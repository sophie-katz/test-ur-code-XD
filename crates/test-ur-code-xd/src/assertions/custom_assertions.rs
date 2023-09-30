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

//! A macro to allow you to write your own custom assertions, either inline or as macros.

/// Helper macro to execute an assertion.
///
///
/// # Arguments
///
/// * `predicate_description` - A description of the predicate. An assertion that checks for
///                             equality might have a predicate description like `"lhs == rhs"`.
/// * `predicate_value` - The value of the predicate. When this is true the assertion passes. When
///                       this is false the assertion fails. An assertion that checks for equality
///                       might use the expression `lhs.eq(rhs)` to check the equality of the two
///                       values.
/// * `configure_panic_message` - A closure that takes a [`PanicMessageBuilder`] and returns an
///                               optionally modified [`PanicMessageBuilder`]. This is used to
///                               configure the panic message, usually to add arguments to it.
/// * `key = value` pairs - A variable number of `key = value` pair expressions, separated by
///                         commas. These are used to set the fields of the [`Config`] instance.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_custom;
/// #
/// let lhs = 5;
/// let rhs = 6;
///
/// assert_custom!(
///     "lhs == rhs",
///     lhs.eq(&rhs),
///     |panic_message_builder| {
///         panic_message_builder
///             .with_argument("lhs", "lhs", &lhs)
///             .with_argument("rhs", "rhs", &rhs)
///     },
///     negate = true,
/// )
/// ```
#[macro_export]
macro_rules! assert_custom {
    (
        $predicate_description:expr,
        $predicate_value:expr,
        $configure_panic_message:expr
        $(, $keys:ident = $values:expr)* $(,)?
    ) => {
        $crate::assertions::config::Config {
            $($keys: $values ,)*
            ..::std::default::Default::default()
        }.execute_assertion(
            $predicate_description,
            $predicate_value,
            ::std::panic::Location::caller(),
            $configure_panic_message,
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::assert_custom;

    #[test]
    fn assert_passing() {
        // let captured_outputs = capture_output(|| {
        assert_custom!("value is true", true, |panic_message_builder| {
            panic_message_builder.with_argument("value", "value", &true)
        });
        assert_custom!(
            "value is true",
            false,
            |panic_message_builder| {
                panic_message_builder.with_argument("value", "value", &true)
            },
            negate = true
        );
        // })
        // .unwrap();

        // std::assert!(captured_outputs.stdout.is_empty());
        // std::assert!(captured_outputs.stderr.is_empty());
    }

    #[test]
    #[should_panic]
    fn assert_custom_fail() {
        assert_custom!("value is true", false, |panic_message_builder| {
            panic_message_builder.with_argument("value", "value", &true)
        });
    }

    #[test]
    #[should_panic]
    fn assert_fail_negated() {
        assert_custom!(
            "value is true",
            true,
            |panic_message_builder| {
                panic_message_builder.with_argument("value", "value", &true)
            },
            negate = true
        );
    }
}
