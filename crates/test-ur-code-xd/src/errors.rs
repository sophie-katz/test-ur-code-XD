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

use thiserror::Error;

/// A general error type for test ur code XD.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum TestUrCodeXDError {
    /// An error that happens when multiple descriptions are added to a panic message builder.
    ///
    /// # Example
    ///
    /// ```
    /// // Either two of the same call
    /// PanicMessageBuilder::new("panic", Location::caller())
    ///     .with_description("some description")
    ///     .with_description("some other description");
    ///
    /// // Or two different calls
    /// PanicMessageBuilder::new("panic", Location::caller())
    ///     .with_description("some description")
    ///     .with_description_owned("some other description".to_owned());
    /// ```
    #[error("cannot add multiple descriptions to a panic message")]
    PanicMessageMultipleDescriptions,
}
