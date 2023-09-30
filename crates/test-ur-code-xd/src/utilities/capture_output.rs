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

mod captured_output;
mod error;
mod output_capturer;

use lazy_static::lazy_static;
use output_capturer::OutputCapturer;
use std::{result::Result, sync::Mutex};

pub use captured_output::CapturedOutputs;
pub use error::Error;

lazy_static! {
    static ref OUTPUT_CAPTURER: Mutex<OutputCapturer> = Mutex::new(OutputCapturer::default());
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
pub fn capture_output<ActionType: FnOnce()>(
    action: ActionType,
) -> Result<CapturedOutputs<String>, Error> {
    // Lock the output capturer for the process to this thread.
    let mut output_captuerer = OUTPUT_CAPTURER.lock().map_err(Error::CapturerMutexError)?;

    // Start capturing
    output_captuerer.start()?;

    // Run the closure
    action();

    // Stop the capture and return the captured output
    let captured_outputs = output_captuerer.stop()?;

    Ok(captured_outputs)
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
pub fn capture_output_raw<ActionType: FnOnce()>(
    action: ActionType,
) -> Result<CapturedOutputs<Vec<u8>>, Error> {
    // Lock the output capturer for the process to this thread.
    let mut output_captuerer = OUTPUT_CAPTURER.lock().map_err(Error::CapturerMutexError)?;

    // Start capturing
    output_captuerer.start()?;

    // Run the closure
    action();

    // Stop the capture and return the captured output
    let captured_outputs = output_captuerer.stop_raw()?;

    Ok(captured_outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            capture_output(|| ()).unwrap(),
            CapturedOutputs {
                stdout: "".to_owned(),
                stderr: "".to_owned(),
            }
        );
    }

    #[test]
    fn stdout() {
        assert_eq!(
            capture_output(|| {
                println!("hello, world");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "hello, world\n".to_owned(),
                stderr: "".to_owned(),
            }
        );
    }

    #[test]
    fn stderr() {
        assert_eq!(
            capture_output(|| {
                eprintln!("hello, world");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "".to_owned(),
                stderr: "hello, world\n".to_owned(),
            }
        );
    }

    #[test]
    fn both() {
        assert_eq!(
            capture_output(|| {
                println!("hello, world");
                eprintln!("hello, world");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "hello, world\n".to_owned(),
                stderr: "hello, world\n".to_owned(),
            }
        );
    }

    #[test]
    fn both_twice() {
        assert_eq!(
            capture_output(|| {
                println!("hello, world");
                eprintln!("hello, world");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "hello, world\n".to_owned(),
                stderr: "hello, world\n".to_owned(),
            }
        );

        println!("asdf");
        eprintln!("asdf");

        assert_eq!(
            capture_output(|| {
                println!("hello, world");
                eprintln!("hello, world");
            })
            .unwrap(),
            CapturedOutputs {
                stdout: "hello, world\n".to_owned(),
                stderr: "hello, world\n".to_owned(),
            }
        );
    }
}
