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

Then, add this to your crate root:

```rust
#[cfg(test)]
#[macro_use]
extern crate test_ur_code_xd;
```

Then, you can use the macros in your tests like this:

```rust hl_lines="3"
#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // ...

        assert_str_eq!(hello_world, "hello, world");

        // ...
    }
}
```

This will give you access to [the assertions](assertions/boolean.md) and to [the test annotations](tests.md).

## Crate features

There are a number of crate features you can enable or disable. All of them are enabled by default.

| Feature       | Description                                                            |
| ------------- | ---------------------------------------------------------------------- |
| `filesystem`  | Enables filesystem assertions                                          |
| `float`       | Enables floating-point assertions                                      |
| `macros`      | Enables the procedural macro used for test parameterization            |
| `output`      | Enables output assertions                                              |
| `panic`       | Enables panic assertions                                               |
| `regex`       | Enables the use of the [`regex`](https://crates.io/crates/regex) crate |
| `string-diff` | Enables the use of string diffing assertions (`assert_str_eq!(...)`)   |

See test ur code XD's [`Cargo.toml`](https://github.com/sophie-katz/test-ur-code-XD/blob/main/crates/test-ur-code-xd/Cargo.toml) to see what dependencies are used for each feature.
