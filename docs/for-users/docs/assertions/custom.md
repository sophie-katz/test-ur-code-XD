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

# Custom assertions

You can make custom assertions by using the `assert_custom!(...)` macro. Let us look at an example:

```rust
// Instead of this:
let x = 3 + 5;
let y = 8;
assert_eq!(x, y);

// We can write this:
assert_custom!(
    "lhs == rhs",
    x == y,
    |panic_message_builder| {
        panic_message_builder
            .with_argument("lhs", "x", &x)
            .with_argument("rhs", "y", &y)
    }
)
```

Let's break it apart. There are three arguments to `assert_custom!(...)`:

* `"lhs == rhs"` - This is the assertion failure message, a description of what the assertion is trying to check for.
* `x == y` - A boolean expression that is the assertion predicate. When this is `true`, the assertion passes. When it is `false`, the assertion panics.
* `|panic_message_builder| { ... }` - A closure that configures the assertion's panic message to display detailed information about the assertion's arguments.

This allows you to make an assertion with any predicate and print any variables that are relevant.