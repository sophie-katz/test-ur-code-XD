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
//!
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/output](https://sophie-katz.github.io/test-ur-code-XD/assertions/output/)
//! for a usage guide.

use std::panic::Location;

use crate::utilities::{
    capture_output::{capture_output, capture_output_raw, CapturedOutputs, OutputCapturingError},
    panic_message_builder::PanicMessageBuilder,
};

/// Helper function to unwrap captured output wrapped in an error and panic.
fn unwrap_captured_outputs<OutputType>(
    result: Result<CapturedOutputs<OutputType>, OutputCapturingError>,
) -> CapturedOutputs<OutputType> {
    match result {
        Ok(value) => value,
        Err(error) => PanicMessageBuilder::new("failed to capture output", Location::caller())
            .with_argument("error", "--", &error.to_string())
            .panic(),
    }
}

#[doc(hidden)]
pub fn assert_outputs_impl<ActionType: FnOnce()>(
    action: ActionType,
    on_stdout: Option<Box<dyn FnOnce(String)>>,
    on_stderr: Option<Box<dyn FnOnce(String)>>,
) {
    let captured_outputs = unwrap_captured_outputs(capture_output(action));

    if let Some(on_stdout) = on_stdout {
        on_stdout(captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(captured_outputs.stderr);
    }
}

/// Assertion wrapper for capturing `stdout` and `stderr` output.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/output](https://sophie-katz.github.io/test-ur-code-XD/assertions/output/)
/// for a usage guide.
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
/// **Note:** At least one of `on_stdout` and `on_stderr` must be passed. `on_stdout` must always
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
    ($action:expr, on_stdout = $on_stdout:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::None,
        )
    };

    ($action:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_impl(
            $action,
            ::std::option::Option::None,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };

    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_impl(
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
    let captured_outputs = unwrap_captured_outputs(capture_output_raw(action));

    if let Some(on_stdout) = on_stdout {
        on_stdout(&captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(&captured_outputs.stderr);
    }
}

/// Assertion wrapper for capturing raw `stdout` and `stderr` output.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/output](https://sophie-katz.github.io/test-ur-code-XD/assertions/output/)
/// for a usage guide.
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
/// **Note:** At least one of `on_stdout` and `on_stderr` must be passed. `on_stdout` must always
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
    ($action:expr, on_stdout = $on_stdout:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::None,
        )
    };

    ($action:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::None,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };

    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::output::assert_outputs_raw_impl(
            $action,
            ::std::option::Option::Some(::std::boxed::Box::new($on_stdout)),
            ::std::option::Option::Some(::std::boxed::Box::new($on_stderr)),
        )
    };
}

#[cfg(test)]
#[allow(clippy::print_stdout, clippy::print_stderr)]
mod tests {
    use crate::assert_eq;

    #[test]
    fn assert_outputs_passing_empty_stdout_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {},
            on_stdout = |stdout| {
                assert_eq!(stdout, "");
            },
        );
    }

    #[test]
    fn assert_outputs_passing_empty_stderr_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {},
            on_stderr = |stderr| {
                assert_eq!(stderr, "");
            },
        );
    }

    #[test]
    fn assert_outputs_passing_empty_both() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {},
            on_stdout = |stdout| {
                assert_eq!(stdout, "");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "");
            }
        );
    }

    #[test]
    fn assert_outputs_passing_stdout_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {
                println!("hello, world");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, "hello, world\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "");
            }
        );
    }

    #[test]
    fn assert_outputs_passing_stderr_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {
                eprintln!("hello, world");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, "");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "hello, world\n");
            }
        );
    }

    #[test]
    fn assert_outputs_passing_both() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, "hello, world (stdout)\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "hello, world (stderr)\n");
            }
        );
    }

    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_outputs_failing_stdout_assertion() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, "asdf");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "hello, world (stderr)\n");
            }
        );
    }

    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_outputs_failing_stderr_assertion() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, "hello, world (stdout)\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, "asdf");
            }
        );
    }

    // TODO: Get this to work
    // #[test]
    // #[should_panic(expected = "explicit panic")]
    // fn assert_outputs_nested() {
    //     assert_outputs!(
    //         || {
    //             assert_outputs!(
    //                 || {
    //                     println!("hello, world (stdout)");
    //                     eprintln!("hello, world (stderr)");
    //                 },
    //                 on_stdout = |stdout| {
    //                     assert_eq!(stdout, "hello, world (stdout)\n");
    //                 }
    //             );
    //         },
    //         on_stdout = |stdout| {
    //             assert_eq!(stdout, "");
    //         }
    //     );
    // }

    #[test]
    fn assert_outputs_raw_passing_empty_stdout_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {},
            on_stdout = |stdout| {
                assert_eq!(stdout, b"");
            },
        );
    }

    #[test]
    fn assert_outputs_raw_passing_empty_stderr_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {},
            on_stderr = |stderr| {
                assert_eq!(stderr, b"");
            },
        );
    }

    #[test]
    fn assert_outputs_raw_passing_empty_both() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {},
            on_stdout = |stdout| {
                assert_eq!(stdout, b"");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"");
            }
        );
    }

    #[test]
    fn assert_outputs_raw_passing_stdout_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {
                println!("hello, world");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, b"hello, world\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"");
            }
        );
    }

    #[test]
    fn assert_outputs_raw_passing_stderr_only() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {
                eprintln!("hello, world");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, b"");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"hello, world\n");
            }
        );
    }

    #[test]
    fn assert_outputs_raw_passing_both() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, b"hello, world (stdout)\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"hello, world (stderr)\n");
            }
        );
    }

    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_outputs_raw_failing_stdout_assertion() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, b"asdf");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"hello, world (stderr)\n");
            }
        );
    }

    #[test]
    #[should_panic(expected = "lhs == rhs")]
    fn assert_outputs_raw_failing_stderr_assertion() {
        println!("this is NOT captured");
        eprintln!("this is NOT captured");

        assert_outputs_raw!(
            || {
                println!("hello, world (stdout)");
                eprintln!("hello, world (stderr)");
            },
            on_stdout = |stdout| {
                assert_eq!(stdout, b"hello, world (stdout)\n");
            },
            on_stderr = |stderr| {
                assert_eq!(stderr, b"asdf");
            }
        );
    }

    // TODO: Get this to work
    // #[test]
    // #[should_panic(expected = "explicit panic")]
    // fn assert_outputs_raw_nested() {
    //     assert_outputs_raw!(
    //         || {
    //             assert_outputs_raw!(
    //                 || {
    //                     println!("hello, world (stdout)");
    //                     eprintln!("hello, world (stderr)");
    //                 },
    //                 on_stdout = |stdout| {
    //                     assert_eq!(stdout, b"hello, world (stdout)\n");
    //                 }
    //             );
    //         },
    //         on_stdout = |stdout| {
    //             assert_eq!(stdout, b"");
    //         }
    //     );
    // }
}
