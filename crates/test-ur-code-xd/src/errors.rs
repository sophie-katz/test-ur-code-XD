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

//! Error types for test ur code XD.

use std::fmt;

use thiserror::Error;

/// A general error type for test ur code XD.
#[derive(Error, Debug)]
// Making the enum non-exhaustive as future-proofing.
#[non_exhaustive]
pub enum TestUrCodeXDError {
    /// An error that happens when multiple descriptions are added to a panic message builder.
    ///
    /// # Example
    ///
    /// ```should_panic
    /// # use std::panic::Location;
    /// # use test_ur_code_xd::utilities::panic_message_builder::PanicMessageBuilder;
    /// #
    /// # fn main() -> Result<(), test_ur_code_xd::errors::TestUrCodeXDError> {
    /// PanicMessageBuilder::new("panic", Location::caller())
    ///     .with_description("some description")?
    ///     .with_description("some other description")?;
    /// #
    /// #    Ok(())
    /// # }
    /// ```
    #[error("cannot add multiple descriptions to a panic message")]
    PanicMessageMultipleDescriptions,

    #[error("error while formatting argument: {0}")]
    ArgumentFormatting(#[from] fmt::Error),

    #[error("`MessageType::AssertionFailure` used for something other than an assertion failure")]
    ImproperUseOfAssertionFailureMessage,
}
