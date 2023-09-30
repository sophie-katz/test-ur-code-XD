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

// The screenshot at docs/for-users/docs/assets/assertion-screenshot.png was taken by running this command:
//
// $ cargo run demo; echo; echo

use test_ur_code_xd::assert_gt;

fn main() {
    let x = 5;
    println!();
    println!();
    assert_gt!(x, 10);
    println!();
    println!();
}
