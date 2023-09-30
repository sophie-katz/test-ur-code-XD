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

use gag::BufferRedirect;
use std::io::{self, Read, Write};

use super::{captured_output::CapturedOutputs, error::OutputStream, Error};

/// A single output-capturing instance.
///
/// This is **NOT** thread safe! Only one of these may be used at any given time.
///
/// # Example
///
/// ```ignore
/// println!("this is NOT captured");
///
/// let mut output_capturer = OutputCapturer::default();
/// output_capturer.start().unwrap();
///
/// println!("this is captured");
///
/// let output = output_capturer.stop();
///
/// println!("this is also NOT captured");
///
/// assert_eq!(output.stdout, "this is captured");
/// ```
#[derive(Default)]
pub struct OutputCapturer {
    buffer_stdout: Option<BufferRedirect>,
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
    pub fn start(&mut self) -> Result<(), Error> {
        // Flush stdout/stderr before redirecting them so we don't get any extra output in the
        // buffers
        Self::flush_streams()?;

        // Start the redirects
        self.buffer_stdout = Some(
            BufferRedirect::stdout()
                .map_err(|error| Error::OutputStreamRedirectError(OutputStream::Stdout, error))?,
        );
        self.buffer_stderr = Some(
            BufferRedirect::stderr()
                .map_err(|error| Error::OutputStreamRedirectError(OutputStream::Stderr, error))?,
        );

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
    pub fn stop(&mut self) -> Result<CapturedOutputs<String>, Error> {
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
    pub fn stop_raw(&mut self) -> Result<CapturedOutputs<Vec<u8>>, Error> {
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
    fn flush_streams() -> Result<(), Error> {
        std::io::stdout()
            .flush()
            .map_err(|error| Error::OutputStreamFlushError(OutputStream::Stdout, error))?;
        std::io::stderr()
            .flush()
            .map_err(|error| Error::OutputStreamFlushError(OutputStream::Stderr, error))
    }

    /// Reads a [`gag`] [`BufferRedect`] as a string.
    fn read_buffer_as_string(
        buffer_redirect: &mut Option<BufferRedirect>,
        output_stream: OutputStream,
    ) -> Result<String, Error> {
        let mut string = String::new();

        buffer_redirect
            .as_mut()
            .expect("OutputCapturer::stop() called before OutputCapturer::start()")
            .read_to_string(&mut string)
            .map_err(|error| Error::OutputStreamBufferReadingError(output_stream, error))?;

        Ok(string)
    }

    /// Reads a [`gag`] [`BufferRedect`] as a vector of bytes.
    fn read_buffer_as_bytes(
        buffer_redirect: &mut Option<BufferRedirect>,
        output_stream: OutputStream,
    ) -> Result<Vec<u8>, Error> {
        buffer_redirect
            .as_mut()
            .expect("OutputCapturer::stop() called before OutputCapturer::start()")
            .bytes()
            .collect::<Result<Vec<u8>, io::Error>>()
            .map_err(|error| Error::OutputStreamBufferReadingError(output_stream, error))
    }

    /// Drop redirects to stop capturing output
    fn drop_redirects(&mut self) {
        self.buffer_stdout = None;
        self.buffer_stderr = None;
    }
}
