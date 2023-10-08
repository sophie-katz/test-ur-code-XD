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

# String assertions

String assertions operate on string types:

```rust
// Compare two strings and diff the results
assert_str_eq!("hello, world", "hello");

// Ensure that the second string is contained within the first
assert_str_contains!("hello, world", "hello");

// Ensure that the first string starts with the second 
assert_str_starts_with!("hello, world", "hello");

// Ensure that the first string ends with the second
assert_str_ends_with!("hello, world", "world");

// Ensure that the first string matches the second regex
assert_str_matches!("hello, world", "[a-z, ]+");
```

Regular expressions for `assert_str_matches` follow the rules for the [`regex` crate](https://docs.rs/regex/latest/regex/).

## Details (advanced)

=== "Traits"

    Both arguments for all these asserts do not have to be a particular string type, but must implement the `AsRef<str>` trait.

=== "Panic conditions"

    | Assertion                | Panic condition                                |
    | ------------------------ | ---------------------------------------------- |
    | `assert_str_contains`    | `!x.as_ref().contains(y.as_ref())`             |
    | `assert_str_starts_with` | `!x.as_ref().starts_with(y.as_ref())`          |
    | `assert_str_ends_with`   | `!x.as_ref().ends_with(y.as_ref())`            |
    | `assert_str_matches`     | `Regex::new(y.as_ref())?.is_match(x.as_ref())` |
