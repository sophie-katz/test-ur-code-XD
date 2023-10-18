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

//! A demo executable for test ur code XD.
//!
//! # How to run
//!
//! The screenshots in docs/for-users/docs/assets was taken by running this command:
//!
//! ```shell
//! cargo run demo; echo; echo
//! ```
//!
//! Just uncomment the assertions you want to demo.
//!
//! # How to run as test
//!
//! Run this to see how an assertion would look like in an actual crate:
//!
//! ```shell
//! cd crates/demo
//! RUST_TEST_NOCAPTURE="0" cargo test
//! ```
//!
//! And then uncomment any assertions you want to demo.

#[macro_use]
extern crate test_ur_code_xd;

// Because some assertions will be commented out
//
// To add in some newlines for the demo
#[allow(unused_variables, clippy::print_stdout)]
fn main() {
    let x = 5;
    println!();
    println!();
    // assert_gt!(x, 10);
    assert_str_eq!("red fish", "two fish");
    println!();
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // assert_eq!(5, 6);
    }
}
