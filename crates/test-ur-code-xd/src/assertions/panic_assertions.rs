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

//! Assertions that catch panics.

use std::panic::{self, Location, UnwindSafe};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

#[doc(hidden)]
pub fn assert_panics_impl<
    ActionType: FnOnce() + UnwindSafe,
    MessageCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_message: Option<MessageCallbackType>,
) {
    if let Err(error) = panic::catch_unwind(action) {
        if let Some(on_message) = on_message {
            on_message(panic_message::panic_message(&error).to_owned());
        }
    } else {
        PanicMessageBuilder::new("action panics", Location::caller()).panic();
    }
}

/// Assertion wrapper for panics.
///
/// # Arguments
///
/// * `action` - A function with no arguments or returns whose panic will be captured.
/// * Optional: `on_message = <value>` - A closure that accepts a `String` as an argument and
///                                      returns nothing. The `String` is the content of the panic
///                                      message that was raised by `action`.
///
/// # Example
///
/// ```
/// assert_panics!(
///     || {
///         panic!("hello, world");
///     },
///     on_message = |message| {
///         assert_eq!(message, "hello, world");
///     }
/// );
/// ```
#[macro_export]
macro_rules! assert_panics {
    ($action:expr, on_message = $on_message:expr) => {
        $crate::assertions::panic_assertions::assert_panics_impl(
            $action,
            ::std::option::Option::Some($on_message),
        )
    };

    ($action:expr) => {
        $crate::assertions::panic_assertions::assert_panics_impl(
            $action,
            ::std::option::Option::<fn(String)>::None,
        )
    };
}
