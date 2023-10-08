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

# Getting Started

## Installation

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
test-ur-code-xd = "0.1.0"
```

## Usage

Then, add `use test_ur_code_xd::*;` to your tests:

```rust hl_lines="3"
#[cfg(test)]
mod tests {
    use test_ur_code_xd::*;

    #[test]
    fn example() {
        // ...

        assert_str_eq!(hello_world, "hello, world");

        // ...
    }
}
```

This will give you access to [the assertions](assertions/boolean.md) and to [the test annotations](tests.md).