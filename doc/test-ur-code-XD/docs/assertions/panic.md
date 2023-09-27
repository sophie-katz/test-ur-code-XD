<!--
Copyright (c) 2023 Sophie Katz

This file is part of test-ur-code-XD.

test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
not, see <https://www.gnu.org/licenses/>.
-->

# Panic assertion

The panic assertion checks if a specific piece of code panics.

```rust
assert_panics!(
    || {
        panic!();
    }
);

assert_panics!(
    || {
        panic!("hello, world");
    },
    on_message = |message| {
        assert_eq!(message, "hello, world");
    }
);
```

The second call to `assert_panics!` takes an `on_message` argument. This argument is a closure
that takes a single argument of type `String` representing the panic message.

## Why not `#[should_panic]`

Rust has a built-in attribute called `#[should_panic]` that can be used to check if a unit test will panic:

```rust
#[test]
#[should_panic]
fn unit_test() {
    panic!();

    // Code here will not run
}
```

There's no reason not to use this. `assert_panics!(...)` provides a bit more granularity and can allow code to continue after the panic:

```rust
#[test]
fn unit_test() {
    assert_panics!(
        || {
            panic!();
        }
    );

    // This code will still run
}
```
