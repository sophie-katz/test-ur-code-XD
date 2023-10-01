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

//! Assertions that capture output to `stdout` and `stderr`.

use crate::utilities::capture_output::{capture_output, capture_output_raw};

#[doc(hidden)]
pub fn assert_outputs_impl<ActionType: FnOnce()>(
    action: ActionType,
    on_stdout: Option<Box<dyn FnOnce(String)>>,
    on_stderr: Option<Box<dyn FnOnce(String)>>,
) {
    let captured_outputs = capture_output(action).unwrap();

    if let Some(on_stdout) = on_stdout {
        on_stdout(captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(captured_outputs.stderr);
    }
}

/// Assertion wrapper for capturing `stdout` and `stderr` output.
///
/// # Arguments
///
/// * `action` - A function with no arguments or returns whose output will be captured.
/// * Optional: `on_stdout = <value>` - A closure that accepts a `String` as an argument and returns
///                                     nothing. The `String` is the content of `stdout` that was
///                                     outputted by `action`.
/// * Optional: `on_stderr = <value>` - A closure that accepts a `String` as an argument and returns
///                                     nothing. The `String` is the content of `stderr` that was
///                                     outputted by `action`.
///
/// **NOTE:** At least one of `on_stdout` and `on_stderr` must be passed. `on_stdout` must always
/// come before `on_stderr`.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::{assert_outputs, assert_eq};
/// #
/// assert_outputs!(
///     || {
///         println!("hello, world");
///     },
///     on_stdout = |stdout| {
///         assert_eq!(stdout, "hello, world\n");
///     }
/// );
/// ```
#[macro_export]
macro_rules! assert_outputs {
    ($action:expr, on_stdout = $on_stdout:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::None,
        )
    };

    ($action:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::None,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };

    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub fn assert_outputs_raw_impl<ActionType: FnOnce()>(
    action: ActionType,
    on_stdout: Option<Box<dyn FnOnce(&[u8])>>,
    on_stderr: Option<Box<dyn FnOnce(&[u8])>>,
) {
    let captured_outputs = capture_output_raw(action).unwrap();

    if let Some(on_stdout) = on_stdout {
        on_stdout(&captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(&captured_outputs.stderr);
    }
}

/// Assertion wrapper for capturing raw `stdout` and `stderr` output.
///
/// # Arguments
///
/// * `action` - A function with no arguments or returns whose output will be captured.
/// * Optional: `on_stdout = <value>` - A closure that accepts a `u8` array as an argument and
///                                     returns nothing. The `u8` array is the content of `stdout`
///                                     that was outputted by `action`.
/// * Optional: `on_stderr = <value>` - A closure that accepts a `u8` array as an argument and
///                                     returns nothing. The `u8` array is the content of `stderr`
///                                     that was outputted by `action`.
///
/// **NOTE:** At least one of `on_stdout` and `on_stderr` must be passed. `on_stdout` must always
/// come before `on_stderr`.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::{assert_outputs_raw, assert_eq};
/// #
/// assert_outputs_raw!(
///     || {
///         println!("hello, world");
///     },
///     on_stdout = |stdout| {
///         assert_eq!(stdout, "hello, world\n".as_bytes());
///     }
/// );
/// ```
#[macro_export]
macro_rules! assert_outputs_raw {
    ($action:expr, on_stdout = $on_stdout:expr) => {
        $crate::assertions::output_assertions::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::None,
        )
    };

    ($action:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::None,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };

    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };
}
