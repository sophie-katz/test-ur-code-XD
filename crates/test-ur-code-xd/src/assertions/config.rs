// Copyright (c) 2023 Sophie Katz
//
// This file is part of test-ur-code-XD.
//
// test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
// not, see <https://www.gnu.org/licenses/>.

//! All assertion macros accept a variable list of `<key> = <value>` arguments. The keys in these
//! arguments are identifiers that correspond to the fields of the [`Config`] struct. The [`Config`]
//! instance is then used to modify the behavior of the assertion in various ways. See the fields
//! of teh structure for details on how to use these arguments.

use std::{fmt::Display, panic::Location};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

/// The configuration for an assertion.
///
/// Contains modifiers that can be applied to the assertion to change its behavior.
//
// Non-documentation note for developers:
//
//   Make sure to put <br /> tags after all field doc comments except for the last one. This is to
//   work around Rustdoc's formatting with examples for fields. It just makes it more readable.
#[derive(Default)]
pub struct Config {
    /// A flag that negates the assertion.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(true);
    ///
    /// // The above assertion is equivalent to:
    ///
    /// assert!(false, negate = true);
    /// ```
    ///
    /// <br />
    pub negate: bool,

    /// A description of what the assertion means.
    ///
    /// This is used in the panic message. Only one of `description` and
    /// `description_owned` can be used.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(
    ///     some_function(),
    ///     description = "`some_function` is always expected to return true"
    /// );
    /// ```
    ///
    /// Note that you cannot use both of these at the same time:
    ///
    /// ```should_panic
    /// assert!(
    ///     some_function(),
    ///     description = "..."
    ///     description_owned = "...".to_owned() // this will panic without running the
    ///                                                    // assertion
    /// );
    /// ```
    ///
    /// <br />
    pub description: &'static str,

    /// A description of what the assertion means.
    ///
    /// This is used in the panic message. Only one of `description` and
    /// `description_owned` can be used.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(
    ///     some_function(),
    ///     description_owned = format!(
    ///         "`some_function` is always expected to return {}",
    ///         true
    ///     )
    /// );
    /// ```
    ///
    /// Note that you cannot use both of these at the same time:
    ///
    /// ```should_panic
    /// assert!(
    ///     some_function(),
    ///     description = "..."
    ///     description_owned = "...".to_owned() // this will panic without running the
    ///                                                    // assertion
    /// );
    /// ```
    pub description_owned: String,
}

impl Config {
    /// A helper function for executing assertions. This will almost always be wrapped by the
    /// `assert_custom` macro.
    ///
    /// # Arguments
    ///
    /// * `predicate_description` - A description of the predicate. An assertion that checks for
    ///                             equality might have a predicate description like `"lhs == rhs"`.
    /// * `predicate_value` - The value of the predicate. When this is true the assertion passes.
    ///                       When this is false the assertion fails. An assertion that checks for
    ///                       equality might use the expression `lhs.eq(rhs)` to check the equality
    ///                       of the two values.
    /// * `location` - The calling location of the assertion. This is used in the panic message.
    ///                This should always be `std::panic::Location::caller()`.
    /// * `configure_panic_message` - A closure that takes a [`PanicMessageBuilder`] and returns an
    ///                               optionally modified [`PanicMessageBuilder`]. This is used to
    ///                               configure the panic message, usually to add arguments to it.
    ///
    /// # Example
    ///
    /// ```
    /// # use test_ur_code_xd::assertions::config::Config;
    /// use std::panic::Location;
    ///
    /// let lhs = 5;
    /// let rhs = 6;
    ///
    /// Config {
    ///     negate: true,
    ///     ..Default::default()
    /// }.execute_assertion(
    ///     "lhs == rhs",
    ///     lhs.eq(&rhs),
    ///     Location::caller(),
    ///     |panic_message_builder| {
    ///         panic_message_builder
    ///             .with_argument("lhs", "lhs", &lhs)
    ///             .with_argument("rhs", "rhs", &rhs)
    ///     }
    /// );
    /// ```
    pub fn execute_assertion<
        ConfigurePanicMessage: FnOnce(PanicMessageBuilder) -> PanicMessageBuilder,
    >(
        self,
        predicate_description: impl Display,
        predicate_value: bool,
        location: &'static Location,
        configure_panic_message: ConfigurePanicMessage,
    ) {
        // Here is the truth table of whether or not to panic:
        //
        // |--------|-----------|-------|
        // | negate | predicate | panic |
        // |--------|-----------|-------|
        // | false  | false     | true  |
        // | false  | true      | false |
        // | true   | false     | false |
        // | true   | true      | true  |
        // |--------|-----------|-------|
        //
        // This truth table is the same as `negate == predicate`, which is used as the condition
        // below. It's hard to read, but efficient!
        if self.negate == predicate_value {
            let panic_message_builder =
                self.create_panic_message_builder(predicate_description, location);

            let panic_message_builder = configure_panic_message(panic_message_builder);

            panic_message_builder.panic();
        }
    }

    /// Helper method to create a panic message from the configuration.
    fn create_panic_message_builder(
        self,
        predicate_description: impl Display,
        location: &'static Location,
    ) -> PanicMessageBuilder {
        PanicMessageBuilder::new(predicate_description, location)
            .with_description(self.description)
            .with_description(self.description_owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn using_struct_no_panic() {
        Config {
            ..Default::default()
        }
        .execute_assertion(
            "value is true",
            true,
            Location::caller(),
            |panic_message_builder| panic_message_builder,
        );
    }

    #[test]
    #[should_panic]
    fn using_struct_does_panic() {
        Config {
            ..Default::default()
        }
        .execute_assertion(
            "value is true",
            false,
            Location::caller(),
            |panic_message_builder| panic_message_builder,
        );
    }

    #[test]
    fn using_struct_no_panic_negated() {
        Config {
            negate: true,
            ..Default::default()
        }
        .execute_assertion(
            "value is true",
            false,
            Location::caller(),
            |panic_message_builder| panic_message_builder,
        );
    }

    #[test]
    #[should_panic]
    fn using_struct_does_panic_negated() {
        Config {
            negate: true,
            ..Default::default()
        }
        .execute_assertion(
            "value is true",
            true,
            Location::caller(),
            |panic_message_builder| panic_message_builder,
        );
    }
}
