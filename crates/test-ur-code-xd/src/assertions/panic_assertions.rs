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

use std::panic::{self, AssertUnwindSafe, Location, UnwindSafe};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

#[doc(hidden)]
pub fn assert_panics_impl<
    ActionType: FnOnce() + UnwindSafe,
    MessageCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_message: Option<MessageCallbackType>,
) {
    if let Err(error) = panic::catch_unwind(AssertUnwindSafe(action)) {
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
/// # use test_ur_code_xd::{assert_panics, assert_eq};
/// #
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

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn assert_panics_passing_no_message_text_no_message_assertions() {
        assert_panics!(|| {
            panic!();
        });
    }

    #[test]
    #[should_panic]
    fn assert_panics_failing_no_panic() {
        assert_panics!(|| {});
    }

    #[test]
    fn assert_panics_passing_no_message_text_with_message_assertion() {
        assert_panics!(
            || {
                panic!();
            },
            on_message = |message| {
                assert_eq!(message, "explicit panic");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_panics_failing_no_message_text_message_assertion() {
        assert_panics!(
            || {
                panic!();
            },
            on_message = |message| {
                assert_eq!(message, "asdf");
            }
        );
    }

    #[test]
    fn assert_panics_passing_with_message_text_no_message_assertions() {
        assert_panics!(|| {
            panic!("hello, world");
        });
    }

    #[test]
    fn assert_panics_passing_with_message_text_with_message_assertion() {
        assert_panics!(
            || {
                panic!("hello, world");
            },
            on_message = |message| {
                assert_eq!(message, "hello, world");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_panics_failing_with_message_text_message_assertion() {
        assert_panics!(
            || {
                panic!("hello, world");
            },
            on_message = |message| {
                assert_eq!(message, "asdf");
            }
        );
    }

    #[test]
    fn assert_panics_passing_hook_stdout_no_message_text() {
        assert_panics!(
            || {
                panic::set_hook(Box::new(move |_| {
                    println!("hello, world");
                }));

                panic!();
            },
            on_message = |message| {
                assert_eq!(message, "explicit panic");
            }
        );
    }

    #[test]
    fn assert_panics_passing_hook_stderr_no_message_text() {
        assert_panics!(
            || {
                panic::set_hook(Box::new(move |_| {
                    eprintln!("hello, world");
                }));

                panic!();
            },
            on_message = |message| {
                assert_eq!(message, "explicit panic");
            }
        );
    }

    #[test]
    fn assert_panics_passing_hook_stdout_with_message_text() {
        assert_panics!(
            || {
                panic::set_hook(Box::new(move |_| {
                    println!("hello, world");
                }));

                panic!("some panic message");
            },
            on_message = |message| {
                assert_eq!(message, "some panic message");
            }
        );
    }

    #[test]
    fn assert_panics_passing_hook_stderr_with_message_text() {
        assert_panics!(
            || {
                panic::set_hook(Box::new(move |_| {
                    eprintln!("hello, world");
                }));

                panic!("some panic message");
            },
            on_message = |message| {
                assert_eq!(message, "some panic message");
            }
        );
    }

    #[test]
    fn assert_panics_passing_with_core_assert_eq() {
        assert_panics!(
            || {
                core::assert_eq!(1, 2);
            },
            on_message = |message| {
                assert_eq!(
                    message,
                    "assertion `left == right` failed\n  left: 1\n right: 2"
                );
            }
        );
    }

    #[test]
    fn assert_panics_passing_with_crate_assert_eq() {
        assert_panics!(
            || {
                crate::assert_eq!(1, 2);
            },
            on_message = |message| {
                assert_eq!(message, "explicit panic");
            }
        );
    }

    #[test]
    fn assert_panics_passing_nested() {
        assert_panics!(|| {
            assert_panics!(|| {});
        });
    }

    #[test]
    #[should_panic]
    fn assert_panics_failing_nested() {
        assert_panics!(|| {
            assert_panics!(|| {
                panic!();
            });
        });
    }
}
