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

use crate::errors::TestUrCodeXDError;
use crate::utilities::truncate::Truncate;
use console::{style, Color};
use indent_write::fmt::IndentWriter;
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    fmt::{Debug, Display},
    panic::{self, Location},
};
use std::{fmt::Write, mem};

use super::truncate::TruncationMode;

/// The truncation separator to use for value descriptions.
const VALUE_DESCRIPTION_SEPARATOR: &str = " ...";

/// The maximum value description length before truncating.
const VALUE_DESCRIPTION_MAX_GRAPHEME_LEN: usize = 50;

/// The prefix to use before a debug representation of a value
pub const DEBUGGED_VALUE_PREFIX: &str = "== ";

/// A builder for a formatted panic message.
///
/// # Example
///
/// ```should_panic
/// use std::panic::Location;
/// # use test_ur_code_xd::utilities::panic_message_builder::PanicMessageBuilder;
///
/// let lhs_description = "x";
/// let lhs_value = 5;
///
/// let rhs_description = "y";
/// let rhs_value = 6;
///
/// let assertion_description = "these two things should always be equal";
///
/// PanicMessageBuilder::new("lhs == rhs", Location::caller())
///     .with_argument("lhs", lhs_description, &lhs_value)
///     .with_argument("rhs", rhs_description, &rhs_value)
///     .with_description(assertion_description)
///     .unwrap()
///     .panic();
/// ```
//
// Non-documentation note for developers:
//
//   This isn't the most efficient way to build a formatted message, but the syntax is easier for
//   developers extending this library to add their own assertions. And efficiency shouldn't matter
//   so much for assertion failure printing lol.
pub struct PanicMessageBuilder {
    /// The panic message to use for the [`panic!`] macro.
    ///
    /// This is not displayed in the console because a panic hook is used to print to `stderr`, but
    /// this message can be used for assertions and testing.
    panic_message: String,

    /// A string buffer that is built up through member calls.
    buffer: String,

    /// A flag that is set when the first assertion description is set.
    has_assertion_description: bool,
}

impl PanicMessageBuilder {
    /// Creates a new panic message builder.
    ///
    /// # Arguments
    ///
    /// * `predicate_description` - A description of the predicate. For example, if the assertion is
    ///                             ensuring that two values are equal the predicate description
    ///                             might be `"lhs == rhs"`.
    /// * `location` - The location of the assertion. This should always be `Location::caller()`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::panic::Location;
    /// # use test_ur_code_xd::utilities::panic_message_builder::PanicMessageBuilder;
    /// PanicMessageBuilder::new("lhs == rhs", Location::caller());
    /// ```
    pub fn new(predicate_description: impl Display, location: &'static Location<'static>) -> Self {
        Self {
            panic_message: format!("{predicate_description}"),
            buffer: format!(
                "{} assertion failed {}: {}",
                style("\u{26CC}").fg(Color::Red).bright().bold(),
                style(format!("at {}:{}", location.file(), location.line(),)).dim(),
                style(predicate_description)
                    .fg(Color::White)
                    .bright()
                    .bold(),
            ),
            has_assertion_description: false,
        }
    }

    /// Adds an argument to the panic message.
    ///
    /// This will print the argument's expression and a debug representation of its value.
    ///
    /// # Arguments
    ///
    /// * `argument_description` - The name of the argument. For example, if the predicate
    ///                            description is `"lhs == rhs"`, then the argument description
    ///                            might be `"lhs"` so that the user can understand which part of
    ///                            the predicate is causing issue.
    /// * `value_description` - The stringified expression of the argument. For example, if the
    ///                         argument is `x + y` the argument description would be `"x + y"`.
    /// * `value` - The value of the argument. The debug representation of this value will get
    ///             printed.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::panic::Location;
    /// # use test_ur_code_xd::utilities::panic_message_builder::PanicMessageBuilder;
    /// #
    /// # let lhs_description = "x";
    /// # let lhs_value = 5;
    /// #
    /// # let rhs_description = "y";
    /// # let rhs_value = 6;
    /// #
    /// # let failure_description = "these two things should always be equal";
    /// #
    /// # PanicMessageBuilder::new("lhs == rhs", Location::caller())
    /// .with_argument("lhs", lhs_description, &lhs_value);
    /// ```
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn with_argument(
        mut self,
        argument_description: impl Display,
        value_description: impl Display,
        value: &impl Debug,
    ) -> Self {
        let value_description_string =
            PanicMessageBuilder::format_value_description(value_description);

        let value_string = format!("{value:?}");

        let argument_description_string = format!("{argument_description}:");

        self.buffer.push_str(
            format!(
                "\n  {} {}",
                style(argument_description_string.as_str()),
                style(&value_description_string).fg(if value_description_string == value_string {
                    Color::Cyan
                } else {
                    Color::White
                }),
            )
            .as_str(),
        );

        if value_description_string != value_string {
            let indent = " ".repeat(3 + argument_description_string.len());

            let mut indented = IndentWriter::new(indent.as_str(), String::new());

            write!(
                indented,
                "\n{}{:#?}",
                style(DEBUGGED_VALUE_PREFIX).dim(),
                style(value).fg(Color::Cyan)
            )
            .expect("unable to write to indented writer");

            self.buffer.push_str(indented.get_ref());
        }

        self
    }

    /// Adds a pre-formatted argument to the panic message.
    ///
    /// This will print the argument's expression and the formatted string for its value.
    ///
    /// # Arguments
    ///
    /// * `argument_description` - The name of the argument. For example, if the predicate
    ///                            description is `"lhs == rhs"`, then the argument description
    ///                            might be `"lhs"` so that the user can understand which part of
    ///                            the predicate is causing issue.
    /// * `value_description` - The stringified expression of the argument. For example, if the
    ///                         argument is `x + y` the argument description would be `"x + y"`.
    /// * `value` - The pre-formatted string value of the argument.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::panic::Location;
    /// # use test_ur_code_xd::utilities::panic_message_builder::PanicMessageBuilder;
    /// #
    /// # let lhs_description = "x";
    /// # let lhs_value = 5;
    /// #
    /// # let rhs_description = "y";
    /// # let rhs_value = 6;
    /// #
    /// # let failure_description = "these two things should always be equal";
    /// #
    /// # PanicMessageBuilder::new("lhs == rhs", Location::caller())
    /// .with_argument_formatted("lhs", lhs_description, "5");
    /// ```
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn with_argument_formatted(
        mut self,
        argument_description: impl Display,
        value_description: impl Display,
        value: impl AsRef<str>,
    ) -> Self {
        let argument_description_string = format!("{argument_description}:");

        self.buffer.push_str(
            format!(
                "\n  {} {}",
                style(argument_description_string.as_str()),
                style(PanicMessageBuilder::format_value_description(
                    value_description
                ))
                .fg(Color::White),
            )
            .as_str(),
        );

        let indent = " ".repeat(3 + argument_description_string.len());

        let mut indented = IndentWriter::new(indent.as_str(), String::new());

        write!(
            indented,
            "\n{}{}",
            style(DEBUGGED_VALUE_PREFIX).dim(),
            style(value.as_ref()).fg(Color::Cyan)
        )
        .expect("unable to write to indented writer");

        self.buffer.push_str(indented.get_ref());

        self
    }

    /// Formats a value description
    fn format_value_description(value_description: impl Display) -> String {
        // truncate::truncate_end(format!("{value_description}"), VALUE_DESCRIPTION_CONTEXT)
        format!("{value_description}").to_truncated(
            VALUE_DESCRIPTION_SEPARATOR,
            TruncationMode::End,
            VALUE_DESCRIPTION_MAX_GRAPHEME_LEN,
        )
    }

    /// Adds an assertion description to the panic message.
    ///
    /// This is a user-defined description of what the purpose of the assertion is.
    ///
    /// # Arguments
    ///
    /// * `assertion_description` - The description of the assertion. If this is an empty string,
    ///                             none is added.
    ///
    /// # Errors
    ///
    /// * If the assertion description is already set.
    ///
    /// # Panics
    ///
    /// Panics if assertion description is already set.
    pub fn with_description(
        mut self,
        assertion_description: impl AsRef<str>,
    ) -> Result<Self, TestUrCodeXDError> {
        let assertion_description = assertion_description.as_ref();

        if !assertion_description.is_empty() {
            if self.has_assertion_description {
                return Err(TestUrCodeXDError::PanicMessageMultipleDescriptions);
            }

            self.buffer
                .push_str(format!("\n  info: {assertion_description}").as_str());

            self.has_assertion_description = true;
        }

        Ok(self)
    }

    /// Formats the panic message but does not panic.
    ///
    /// This is the termination of the builder chain.
    ///
    /// # Returns
    ///
    /// The formatted panic message.
    #[must_use]
    pub fn format(mut self) -> String {
        // Format backtrace onto the end of the buffer
        self.buffer
            .push_str(format!("\n\n{}", PanicMessageBuilder::format_backtrace()).as_str());

        // Return the buffer
        self.buffer
    }

    /// Format a backtrace
    ///
    /// # Returns
    ///
    /// * If the backtrace was captured, the formatted backtrace.
    /// * Otherwise, a message telling the user how to enable backtrace capturing.
    fn format_backtrace() -> String {
        let backtrace = Backtrace::capture();

        if backtrace.status() == BacktraceStatus::Captured {
            PanicMessageBuilder::format_backtrace_captured(backtrace)
        } else {
            PanicMessageBuilder::format_backtrace_message()
        }
    }

    /// Format a captured backtrace
    fn format_backtrace_captured(backtrace: Backtrace) -> String {
        style(backtrace).dim().to_string()
    }

    /// Format a message telling the user how to enable backtrace capturing
    fn format_backtrace_message() -> String {
        style("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace")
            .dim()
            .to_string()
    }

    /// Triggers the panic with the built message.
    ///
    /// This is the termination of the builder chain.
    ///
    /// # Returns
    ///
    /// This function never returns. It always panics.
    #[allow(clippy::missing_panics_doc, clippy::print_stderr, clippy::panic)]
    pub fn panic(mut self) -> ! {
        let panic_message = mem::take(&mut self.panic_message);

        let buffer = self.format();

        panic::set_hook(Box::new(move |_| {
            eprintln!("{buffer}");
        }));

        panic!("{panic_message}");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::assert;

    #[cfg(feature = "regex")]
    use crate::assert_str_matches;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct SomeStruct {
        a: i32,
        b: i32,
        c: String,
    }

    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn panics() {
        PanicMessageBuilder::new("lhs == rhs", Location::caller()).panic();
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_minimal() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("lhs == rhs", Location::caller()).format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: lhs == rhs

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_one_argument_description_matches() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("", Location::caller())
            .with_argument("lhs", "5", &5)
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: 
  lhs: 5

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_one_argument_description_does_not_match() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("", Location::caller())
            .with_argument("lhs", "x", &5)
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: 
  lhs: x
       == 5

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_two_arguments() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("", Location::caller())
            .with_argument("lhs", "x", &5)
            .with_argument("rhs", "y", &6)
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: 
  lhs: x
       == 5
  rhs: y
       == 6

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_assertion_description_str() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("", Location::caller())
            .with_description("assertion description")
            .unwrap()
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: 
  info: assertion description

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_assertion_description_string() {
        console::set_colors_enabled(false);

        #[allow(clippy::unnecessary_to_owned)]
        let message = PanicMessageBuilder::new("", Location::caller())
            .with_description("assertion description".to_owned())
            .unwrap()
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: 
  info: assertion description

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[test]
    fn two_assertion_descriptions() {
        assert!(PanicMessageBuilder::new("", Location::caller())
            .with_description("assertion description")
            .unwrap()
            .with_description("assertion description")
            .is_err());
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_pretty_debug_argument() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("predicate description", Location::caller())
            .with_argument(
                "argument",
                "value",
                &SomeStruct {
                    a: 1,
                    b: 2,
                    c: "3".to_owned(),
                },
            )
            .format();

        assert_str_matches!(message, "\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\\.rs:[0-9]+: predicate description
  argument: value
            == SomeStruct \\{
                a: 1,
                b: 2,
                c: \"3\",
            \\}

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace");
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_no_multiline_argument_descriptions() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("predicate description", Location::caller())
            .with_argument("argument", "a\nb", &1)
            .format();

        assert_str_matches!(
            message,
            "\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\\.rs:[0-9]+: predicate description
  argument: a ...
            == 1

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn format_truncate_argument_descriptions() {
        console::set_colors_enabled(false);

        let message = PanicMessageBuilder::new("predicate description", Location::caller())
            .with_argument(
                "argument",
                "a".repeat(VALUE_DESCRIPTION_MAX_GRAPHEME_LEN + 100),
                &1,
            )
            .format();

        assert_str_matches!(
            message,
            r"\u{26cc} assertion failed at crates/test-ur-code-xd/src/utilities/panic_message_builder\.rs:[0-9]+: predicate description
  argument: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa ...
            == 1

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        );
    }
}
