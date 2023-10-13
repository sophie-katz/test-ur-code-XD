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

//! Error types specific to output capturing.

use std::{
    io,
    sync::{MutexGuard, PoisonError},
};
use thiserror::Error;

use super::output_capturer::OutputCapturer;

/// Differentiator between output streams.
#[derive(Debug, PartialEq)]
#[allow(clippy::missing_docs_in_private_items)]
#[non_exhaustive]
pub enum OutputStream {
    Stdout,
    Stderr,
}

/// An error that can occur when capturing output.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OutputCapturingError {
    #[error("error while flushing {0:?}: {1}")]
    OutputStreamFlushError(OutputStream, io::Error),

    #[error("error while redirecting {0:?}: {1}")]
    OutputStreamRedirectError(OutputStream, io::Error),

    #[error("error while reading buffer for {0:?}: {1}")]
    OutputStreamBufferReadingError(OutputStream, io::Error),

    /// A wrapper for an error which occurs when locking a mutex.
    #[error("guard poison error: {0}")]
    CapturerMutexError(PoisonError<MutexGuard<'static, OutputCapturer>>),

    #[error("nested calls to capture_output or capture_output_raw")]
    NestedCaptureError,

    #[error("OutputCapturer::stop() was called before OutputCapturer::start()")]
    StopCalledBeforeStart,
}
