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

mod captured_output;
mod errors;
mod output_capturer;

use lazy_static::lazy_static;
use output_capturer::OutputCapturer;
use std::{
    result::Result,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
};

pub use captured_output::CapturedOutputs;
pub use errors::OutputCapturingError;

lazy_static! {
    /// A singleton [`OutputCapturer`] instance for the process.
    static ref OUTPUT_CAPTURER: Mutex<OutputCapturer> = Mutex::new(OutputCapturer::default());

    /// A flag to prevent nesting of calls to [`capture_output`] and [`capture_output_raw`].
    static ref IS_IN_CAPTURE_OUTPUT: AtomicBool = AtomicBool::new(false);
}

/// Helper function to prevent nesting of calls to [`capture_output`] and [`capture_output_raw`].
///
/// This prevents deadlocks from happening with the mutex, and instead just returns an error
/// immediately.
///
/// This is thread-safe.
///
/// # Arguments
///
/// * `action` - The action to wrap.
///
/// # Returns
///
/// The result of the action wrapped in a [`Result`] instance.
///
/// # Errors
///
/// * If calls to [`capture_output`] or [`capture_output_raw`] are nested, this function will
///   return an error.
/// * Also returns any errors from the action.
fn non_nesting_helper<
    ActionType: FnOnce() -> Result<ResultType, OutputCapturingError>,
    ResultType,
>(
    action: ActionType,
) -> Result<ResultType, OutputCapturingError> {
    if IS_IN_CAPTURE_OUTPUT.load(Ordering::SeqCst) {
        return Err(OutputCapturingError::NestedCaptureError);
    }

    IS_IN_CAPTURE_OUTPUT.store(true, Ordering::SeqCst);

    let result = action();

    IS_IN_CAPTURE_OUTPUT.store(false, Ordering::SeqCst);

    result
}

/// Gets the single instance of the output capturer for the process.
///
/// This is thread-safe.
///
/// # Returns
///
/// A reference to the [`OutputCapturer`] instance, wrapped in a [`MutexGuard`].
///
/// # Errors
///
/// * If there are any issues with locking mutexes, this function will return an error.
fn get_output_capturer() -> Result<MutexGuard<'static, OutputCapturer>, OutputCapturingError> {
    OUTPUT_CAPTURER
        .lock()
        .map_err(OutputCapturingError::CapturerMutexError)
}

/// Captures `stdout` and `stderr` output from a closure in a thread-safe manner.
///
/// It is essentially a thread-safe wrapper on top of the excellent [gag] crate. It works by
/// synchronizing code with captured output so that only one captured action can run at a time.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::utilities::capture_output::capture_output;
/// #
/// let output = capture_output(|| {
///    println!("print something to stdout");
/// }).expect("error while capturing output");
///
/// assert_eq!(output.stdout, "print something to stdout\n");
/// ```
///
/// # Returns
///
/// A [`CapturedOutputs`] instance containing the captured output for both `stdout` and `stderr`.
///
/// # Errors
///
/// * If there are any issues with redirecting `stdout` or `stderr`, this function will return an
///   error.
/// * If there are any issues with flushing `stdout` or `stderr`, this function will return an
///   error.
/// * If there are any issues with reading the buffers, this function will return an error.
/// * If there are any issues with locking mutexes, this function will return an error.
/// * If calls to [`capture_output`] or [`capture_output_raw`] are nested, this function will
///   return an error.
pub fn capture_output<ActionType: FnOnce()>(
    action: ActionType,
) -> Result<CapturedOutputs<String>, OutputCapturingError> {
    non_nesting_helper(|| {
        // Lock the output capturer for the process to this thread.
        let mut output_capturer = get_output_capturer()?;

        // Start capturing
        output_capturer.start()?;

        // Run the closure
        action();

        // Stop the capture and return the captured output
        let captured_outputs = output_capturer.stop()?;

        Ok(captured_outputs)
    })
}

/// Captures raw `stdout` and `stderr` output from a closure in a thread-safe manner.
///
/// It is essentially a thread-safe wrapper on top of the excellent [gag] crate. It works by
/// synchronizing code with captured output so that only one captured action can run at a time.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::utilities::capture_output::capture_output_raw;
/// #
/// let output = capture_output_raw(|| {
///    println!("print something to stdout");
/// }).expect("error while capturing output");
///
/// assert_eq!(output.stdout, "print something to stdout\n".as_bytes());
/// ```
///
/// # Returns
///
/// A [`CapturedOutputs`] instance containing the captured output for both `stdout` and `stderr`.
///
/// # Errors
///
/// * If there are any issues with redirecting `stdout` or `stderr`, this function will return an
///   error.
/// * If there are any issues with flushing `stdout` or `stderr`, this function will return an
///   error.
/// * If there are any issues with reading the buffers, this function will return an error.
/// * If there are any issues with locking mutexes, this function will return an error.
/// * If calls to [`capture_output`] or [`capture_output_raw`] are nested, this function will
///   return an error.
#[allow(clippy::module_name_repetitions)]
pub fn capture_output_raw<ActionType: FnOnce()>(
    action: ActionType,
) -> Result<CapturedOutputs<Vec<u8>>, OutputCapturingError> {
    non_nesting_helper(|| {
        // Lock the output capturer for the process to this thread.
        let mut output_capturer = get_output_capturer()?;

        // Start capturing
        output_capturer.start()?;

        // Run the closure
        action();

        // Stop the capture and return the captured output
        let captured_outputs = output_capturer.stop_raw()?;

        Ok(captured_outputs)
    })
}

#[cfg(test)]
// Stdout and stderr printing are allowed in order to generate output for tests.
//
// Unwrap allowed to reduce length of test code.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{assert, assert_eq};

    #[test]
    fn none() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| ()).unwrap(),
            CapturedOutputs {
                stdout: String::new(),
                stderr: String::new(),
            }
        );
    }

    #[test]
    fn stdout() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                println!("this IS captured (stdout)");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "this IS captured (stdout)\n".to_owned(),
                stderr: String::new(),
            }
        );
    }

    #[test]
    fn stderr() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                eprintln!("this IS captured (stderr)");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: String::new(),
                stderr: "this IS captured (stderr)\n".to_owned(),
            }
        );
    }

    #[test]
    fn both() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                println!("this IS captured (stdout)");
                eprintln!("this IS captured (stderr)");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "this IS captured (stdout)\n".to_owned(),
                stderr: "this IS captured (stderr)\n".to_owned(),
            }
        );
    }

    #[test]
    fn both_twice() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                println!("this IS captured (stdout)");
                eprintln!("this IS captured (stderr)");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "this IS captured (stdout)\n".to_owned(),
                stderr: "this IS captured (stderr)\n".to_owned(),
            }
        );

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                println!("this IS captured (stdout)");
                eprintln!("this IS captured (stderr)");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "this IS captured (stdout)\n".to_owned(),
                stderr: "this IS captured (stderr)\n".to_owned(),
            }
        );
    }

    #[test]
    fn nested() {
        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            capture_output(|| {
                println!("this IS captured (stdout)");
                eprintln!("this IS captured (stderr)");

                assert!(capture_output(|| {
                    println!("this is invalid (stdout)");
                    eprintln!("this is invalid (stderr)");
                })
                .is_err());
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "this IS captured (stdout)\n".to_owned(),
                stderr: "this IS captured (stderr)\n".to_owned(),
            }
        );
    }
}
