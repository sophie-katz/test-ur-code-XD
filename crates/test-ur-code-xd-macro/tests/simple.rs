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

use test_ur_code_xd_macro::test_with_parameter_values;

// WARNING: Rust Analyzer displays a false negative error here. This is due to a bug in Rust
//          Analyzer, not an actual issue with the code.
//
//          See https://github.com/rust-lang/rust-analyzer/issues/12450 for more info.
#[test_with_parameter_values(
    x = [5, 6, 7],
    y = [1, 2]
)]
fn example(x: i32, y: i32) {
    #[allow(clippy::arithmetic_side_effects)]
    let z = x + y;

    assert!(z > 0);
}

// WARNING: Rust Analyzer displays a false negative error here. This is due to a bug in Rust
//          Analyzer, not an actual issue with the code.
//
//          See https://github.com/rust-lang/rust-analyzer/issues/12450 for more info.
#[test_with_parameter_values(
    x = [5, 6, 7],
    y = [1, 2]
)]
#[allow(clippy::arithmetic_side_effects)]
#[should_panic(expected = "asdf")]
fn failure(x: i32, y: i32) {
    #[allow(clippy::arithmetic_side_effects)]
    let z = x + y;

    assert!(z < 0);
}
