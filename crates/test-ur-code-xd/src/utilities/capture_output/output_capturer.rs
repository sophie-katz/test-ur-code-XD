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

//! Defines a struct to help with capturing output.

use gag::BufferRedirect;
use std::io::{self, Read, Write};

#[allow(unused_imports)]
use std::io::{stderr, stdout};

use super::{captured_output::CapturedOutputs, errors::OutputStream, OutputCapturingError};

/// A struct to help with capturing output.
#[derive(Default)]
pub struct OutputCapturer {
    /// A buffer redirect for [`stdout`]
    buffer_stdout: Option<BufferRedirect>,
    /// A buffer redirect for [`stdout`]
    buffer_stderr: Option<BufferRedirect>,
}

impl OutputCapturer {
    /// Starts capturing output.
    ///
    /// It will flush `stdout` and `stderr` before redirecting them so we don't get any extra output
    /// in the capture.
    ///
    /// # Errors
    ///
    /// * If there are issues with flushing `stdout` or `stderr`, this function will return an
    ///   error.
    /// * If more than one [`OutputCapturer`] is started in the same process, it will return an
    ///   error.
    pub fn start(&mut self) -> Result<(), OutputCapturingError> {
        // Flush stdout/stderr before redirecting them so we don't get any extra output in the
        // buffers
        Self::flush_streams()?;

        // Start the redirects
        self.buffer_stdout = Some(BufferRedirect::stdout().map_err(|error| {
            OutputCapturingError::OutputStreamRedirectError(OutputStream::Stdout, error)
        })?);
        self.buffer_stderr = Some(BufferRedirect::stderr().map_err(|error| {
            OutputCapturingError::OutputStreamRedirectError(OutputStream::Stderr, error)
        })?);

        Ok(())
    }

    /// Stops capturing output.
    ///
    /// It will flush `stdout` and `stderr` before stopping so we don't get any missing output in
    /// the capture.
    ///
    /// # Returns
    ///
    /// A [`CapturedOutputs`] instance containing the captured output for both `stdout` and
    /// `stderr`.
    ///
    /// # Errors
    ///
    /// * If there are issues with flushing `stdout` or `stderr`, this function will return an
    ///   error.
    /// * If there are any issues with reading the buffers, this function will return an error.
    pub fn stop(&mut self) -> Result<CapturedOutputs<String>, OutputCapturingError> {
        // Flush stdout/stderr before stopping so we get all of the output in the buffers
        Self::flush_streams()?;

        // Read the buffers into the output strings
        let string_stdout =
            Self::read_buffer_as_string(&mut self.buffer_stdout, OutputStream::Stdout)?;
        let string_stderr =
            Self::read_buffer_as_string(&mut self.buffer_stderr, OutputStream::Stderr)?;

        // Drop the redirects
        self.drop_redirects();

        // Return the output strings
        Ok(CapturedOutputs {
            stdout: string_stdout,
            stderr: string_stderr,
        })
    }

    /// Stops capturing output and return as raw buffers.
    ///
    /// It will flush `stdout` and `stderr` before stopping so we don't get any missing output in
    /// the capture.
    ///
    /// # Returns
    ///
    /// A [`CapturedOutputs`] instance containing the captured output for both `stdout` and
    /// `stderr`.
    ///
    /// # Errors
    ///
    /// * If there are issues with flushing `stdout` or `stderr`, this function will return an
    ///   error.
    /// * If there are any issues with reading the buffers, this function will return an error.
    pub fn stop_raw(&mut self) -> Result<CapturedOutputs<Vec<u8>>, OutputCapturingError> {
        // Flush stdout/stderr before stopping so we get all of the output in the buffers
        Self::flush_streams()?;

        // Read the buffers into the output strings
        let bytes_stdout =
            Self::read_buffer_as_bytes(&mut self.buffer_stdout, OutputStream::Stdout)?;
        let bytes_stderr =
            Self::read_buffer_as_bytes(&mut self.buffer_stderr, OutputStream::Stderr)?;

        // Drop the redirects
        self.drop_redirects();

        // Return the output strings
        Ok(CapturedOutputs {
            stdout: bytes_stdout,
            stderr: bytes_stderr,
        })
    }

    /// Flush both `stdout` and `stderr`.
    ///
    /// # Errors
    ///
    /// * If there is an IO error while flushing either stream, this function will return an error.
    fn flush_streams() -> Result<(), OutputCapturingError> {
        stdout().flush().map_err(|error| {
            OutputCapturingError::OutputStreamFlushError(OutputStream::Stdout, error)
        })?;
        stderr().flush().map_err(|error| {
            OutputCapturingError::OutputStreamFlushError(OutputStream::Stderr, error)
        })
    }

    /// Reads a [`gag`] [`BufferRedirect`] as a string.
    fn read_buffer_as_string(
        buffer_redirect: &mut Option<BufferRedirect>,
        output_stream: OutputStream,
    ) -> Result<String, OutputCapturingError> {
        let mut string = String::new();

        buffer_redirect
            .as_mut()
            .ok_or(OutputCapturingError::StopCalledBeforeStart)?
            .read_to_string(&mut string)
            .map_err(|error| {
                OutputCapturingError::OutputStreamBufferReadingError(output_stream, error)
            })?;

        Ok(string)
    }

    /// Reads a [`gag`] [`BufferRedirect`] as a vector of bytes.
    fn read_buffer_as_bytes(
        buffer_redirect: &mut Option<BufferRedirect>,
        output_stream: OutputStream,
    ) -> Result<Vec<u8>, OutputCapturingError> {
        buffer_redirect
            .as_mut()
            .ok_or(OutputCapturingError::StopCalledBeforeStart)?
            .bytes()
            .collect::<Result<Vec<u8>, io::Error>>()
            .map_err(|error| {
                OutputCapturingError::OutputStreamBufferReadingError(output_stream, error)
            })
    }

    /// Drop redirects to stop capturing output
    fn drop_redirects(&mut self) {
        self.buffer_stdout = None;
        self.buffer_stderr = None;
    }
}

#[cfg(test)]
#[allow(clippy::print_stdout, clippy::print_stderr)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{assert, assert_eq};

    #[test]
    fn capture_none() {
        let mut output_capturer = OutputCapturer::default();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        output_capturer.start().unwrap();

        let captured_output = output_capturer.stop().unwrap();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(captured_output.stdout, "");
        assert_eq!(captured_output.stderr, "");
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: StopCalledBeforeStart")]
    fn capture_stop_without_start() {
        let mut output_capturer = OutputCapturer::default();

        let _: CapturedOutputs<String> = output_capturer.stop().unwrap();
    }

    #[test]
    fn capture_some() {
        let mut output_capturer = OutputCapturer::default();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        output_capturer.start().unwrap();

        println!("this IS captured (stdout)");
        eprintln!("this IS captured (stderr)");

        let captured_output = output_capturer.stop().unwrap();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(captured_output.stdout, "this IS captured (stdout)\n");
        assert_eq!(captured_output.stderr, "this IS captured (stderr)\n");
    }

    #[test]
    fn capture_none_raw() {
        let mut output_capturer = OutputCapturer::default();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        output_capturer.start().unwrap();

        let captured_output = output_capturer.stop_raw().unwrap();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(captured_output.stdout, "".as_bytes());
        assert_eq!(captured_output.stderr, "".as_bytes());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: StopCalledBeforeStart")]
    fn capture_stop_without_start_raw() {
        let mut output_capturer = OutputCapturer::default();

        let _: CapturedOutputs<Vec<u8>> = output_capturer.stop_raw().unwrap();
    }

    #[test]
    fn capture_some_raw() {
        let mut output_capturer = OutputCapturer::default();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        output_capturer.start().unwrap();

        println!("this IS captured (stdout)");
        eprintln!("this IS captured (stderr)");

        let captured_output = output_capturer.stop_raw().unwrap();

        println!("this is NOT captured (stdout)");
        eprintln!("this is NOT captured (stderr)");

        assert_eq!(
            captured_output.stdout,
            "this IS captured (stdout)\n".as_bytes()
        );
        assert_eq!(
            captured_output.stderr,
            "this IS captured (stderr)\n".as_bytes()
        );
    }

    #[test]
    fn multiple_captures() {
        let mut output_capturer_0 = OutputCapturer::default();

        let mut output_capturer_1 = OutputCapturer::default();

        output_capturer_0.start().unwrap();

        assert!(output_capturer_1.start().is_err());
    }
}
