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

//! This crate provides:
//! * More readable assertion messages
//! * More assertions including:
//!     * Floating-point assertions
//!     * String assertions
//!     * Filesystem assertions
//!     * Assertions for `stdout` and `stderr`
//!     * Inline panic assertions
//! * Parameterized tests
//!
//! 

#[doc(hidden)]
pub mod assertions;
mod capture_output;
