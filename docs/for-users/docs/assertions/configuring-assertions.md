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

# Configuring assertions

All assertions have some optional keyword arguments that you can pass in to configure their behavior.

## Negation

Add in the `negate = true` argument to make the assertion have the opposite of its normal behavior. For example:

```rust
assert_str_contains!("hello, world", "asdf", negate = true);
```

This ensures that `"hello, world"` does *not* contain `"asdf"`.

## Descriptions

You can add a description to an assertion by passing in the `description = <message>` argument. For example:

```rust
let x = 1.0;
let y = 1.05;

assert_le!(
    (x - y).abs(),
    0.1,
    description = "x should be within 0.1 of y"
);
```

You can also add formatting with the `description_owned = <message>` argument:

```rust
const THRESHOLD: f32 = 0.1;

let x = 1.0;
let y = 1.05;

assert_le!(
    (x - y).abs(),
    THRESHOLD,
    description_owned = format!("x should be within {} of y", THRESHOLD)
);
```

`description` accepts as `&str` value while `description_owned` accepts a `String` value.
