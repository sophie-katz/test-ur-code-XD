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

use gag::BufferRedirect;
use lazy_static::lazy_static;
use std::{
    io::{Read, Write},
    result::Result,
    {io, sync::Mutex},
};

#[derive(Default)]
pub(crate) struct OutputCapturer {
    buffer_stdout: Option<BufferRedirect>,
    buffer_stderr: Option<BufferRedirect>,
}

impl OutputCapturer {
    pub fn start(&mut self) -> Result<(), io::Error> {
        // Flush stdout/stderr before redirecting them so we don't get any extra output in the
        // buffers
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();

        // Start the redirects
        self.buffer_stdout = Some(BufferRedirect::stdout()?);
        self.buffer_stderr = Some(BufferRedirect::stderr()?);

        Ok(())
    }

    pub fn stop(&mut self) -> CapturedOutputs {
        // Flush stdout/stderr before stopping so we get all of the output in the buffers
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();

        // Allocate strings to store the output
        let mut string_stdout = String::new();
        let mut string_stderr = String::new();

        // Read the buffers into the output strings
        self.buffer_stdout
            .as_mut()
            .expect("OutputCapturer::stop() called before OutputCapturer::start()")
            .read_to_string(&mut string_stdout)
            .unwrap();
        self.buffer_stderr
            .as_mut()
            .expect("OutputCapturer::stop() called before OutputCapturer::start()")
            .read_to_string(&mut string_stderr)
            .unwrap();

        // Drop the redirects
        self.buffer_stdout = None;
        self.buffer_stderr = None;

        // Return the output strings
        CapturedOutputs {
            stdout: string_stdout,
            stderr: string_stderr,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CapturedOutputs {
    pub stdout: String,
    pub stderr: String,
}

lazy_static! {
    static ref OUTPUT_CAPTURER: Mutex<OutputCapturer> = Mutex::new(OutputCapturer::default());
}

pub(crate) fn capture_output<ActionType: FnOnce()>(action: ActionType) -> CapturedOutputs {
    let mut output_captuerer = OUTPUT_CAPTURER
        .lock()
        .expect("failed to lock output capturer mutex");

    output_captuerer
        .start()
        .expect("failed to start output capturer");

    action();

    output_captuerer.stop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            capture_output(|| ()),
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
            }),
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
            }),
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
            }),
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
            }),
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
            }),
            CapturedOutputs {
                stdout: "hello, world\n".to_owned(),
                stderr: "hello, world\n".to_owned(),
            }
        );
    }
}
