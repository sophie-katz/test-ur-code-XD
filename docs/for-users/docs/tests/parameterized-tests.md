<!--
Copyright (c) 2023 Sophie Katz

This file is part of test ur code XD.

test ur code XD is free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

test ur code XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along with test ur code XD. If
not, see <https://www.gnu.org/licenses/>.
-->

# Parameterized tests

test ur code XD provides an attribute to parameterize tests:

```rust
#[test_with_parameter_values(
    x = [5, 6, 7],
    y = [1, 2])
]
fn example(x: i32, y: i32) {
    assert!(x + y > 0);
}
```

This will permute the values of `x` and `y` and run the test for each permutation. In this case, the test will run 6 times:

| `x` | `y` |
| --- | --- |
| 5   | 1   |
| 5   | 2   |
| 6   | 1   |
| 6   | 2   |
| 7   | 1   |
| 7   | 2   |

!!! warning

    The values must be array literals. Vectors or other dynamically generated values are not supported.

## Other attributes

Any other attributes on the test function will be applied to each permutation. For example, this test using the `#[should_panic(expected = "explicit panic")]` attribute:

```rust hl_lines="5"
#[test_with_parameter_values(
    x = [5, 6, 7],
    y = [1, 2])
]
#[should_panic(expected = "explicit panic")]
fn example(x: i32, y: i32) {
    assert!(x + y < 0);
}
```

Will be converted to these functions:

```rust hl_lines="6"
fn _test_ur_code_xd_example_parameter_function(x: i32, y: i32) {
    assert!(x + y < 0);
}

#[test]
#[should_panic(expected = "explicit panic")]
fn example_0() {
    let x = 5;
    let y = 1;

    _test_ur_code_xd_example_parameter_function(x, y);
}

// ...
```

!!! warning

    This means that attributes like `#[cfg(target_family = "unix")]` will only be applied to the permutation functions like `example_0`, not the parameter function `_test_ur_code_xd_example_parameter_function`.
