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

//! Assertions are configurable in test-ur-code-XD. This is to allow for flexible arguments to
//! assertions.
//!
//! # Negating assertions
//!
//! You can take any test-ur-code-XD assertion and negate it like this:
//!
//! ```ignore
//! assert_str_contains!("hello, world", "hi", negate = true);
//! ```
//!
//! This will pass because `"hello, world"` does not contain `"hi"`.
//!
//! # Implementation
//!
//! This works because all assertion macros allow `<key> = <value>` arguments with the same keys as
//! the [`Config`] struct.

use std::{fmt::Display, panic::Location};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

/// The configuration for an assertion.
///
/// Contains modifiers that can be applied to the assertion to change its behavior.
#[derive(Default)]
pub struct Config {
    pub negate: bool,
    pub assertion_description: &'static str,
    pub assertion_description_owned: String,
}

impl Config {
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
            // (executor.on_panic)(self);
            let panic_message_builder =
                self.create_panic_message_builder(predicate_description, location);

            let panic_message_builder = configure_panic_message(panic_message_builder);

            panic_message_builder.panic();
        }
    }

    fn create_panic_message_builder(
        self,
        predicate_description: impl Display,
        location: &'static Location,
    ) -> PanicMessageBuilder {
        PanicMessageBuilder::new(predicate_description, location)
            .with_assertion_description(self.assertion_description)
            .with_assertion_description(self.assertion_description_owned)
    }
}

#[macro_export]
macro_rules! execute_assertion {
    ($predicate_description:expr, $predicate_value:expr, $configure_panic_message:expr $(, $keys:ident = $values:expr)* $(,)?) => {
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
    use super::*;

    // #[test]
    // fn no_panic() {
    //     AssertionConfig {
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| true, |config| panic!());
    // }

    // #[test]
    // #[should_panic]
    // fn does_panic() {
    //     AssertionConfig {
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| false, |config| panic!());
    // }

    // #[test]
    // fn no_panic_negated_by_flag() {
    //     AssertionConfig {
    //         negate: true,
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| false, |config| panic!());
    // }

    // #[test]
    // #[should_panic]
    // fn does_panic_negated_by_flag() {
    //     AssertionConfig {
    //         negate: true,
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| true, |config| panic!());
    // }

    // #[test]
    // fn no_panic_negated_by_operation() {
    //     !AssertionConfig {
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| false, |config| panic!());
    // }

    // #[test]
    // #[should_panic]
    // fn does_panic_negated_by_operation() {
    //     !AssertionConfig {
    //         ..Default::default()
    //     } | make_assertion_part_executor!(|| true, |config| panic!());
    // }
}
