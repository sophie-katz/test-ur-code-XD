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

# Arithmetic assertions

Arithmetic assertions are those that compare two values together.

## Equality

These assertions compare values that implement the `PartialEq` trait:

```rust
// Ensure that the values are equal
assert_eq!(x, y);

// Ensure that the values are inequal
assert_ne!(x, y);
```

## Ordering

These assertions compare values that implement the `PartialOrd` trait:

```rust
// Ensure that x is less than y
assert_lt!(x, y);

// Ensure that x is less than or equal to y
assert_le!(x, y);

// Ensure that x is greater than y
assert_gt!(x, y);

// Ensure that x is greater than or equal to y
assert_ge!(x, y);
```

## Details (advanced)

=== "Traits"

    The two values do not have to be of the same type. If the first value is of type `T` and the second of type `U`, then the following traits must be implemented:

    ```rust
    // for the equality assertions to work
    impl PartialEq<U> for T { ... }

    // for the order assertions to work
    impl PartialOrd<U> for T { ... }
    ```

    Both types, additionally, must implement the `Debug` trait.

=== "Panic conditions"

    | Assertion   | Panic condition |
    | ----------- | --------------- |
    | `assert_eq` | `!x.eq(y)`      |
    | `assert_ne` | `!x.ne(y)`      |
    | `assert_lt` | `!x.lt(y)`      |
    | `assert_le` | `!x.le(y)`      |
    | `assert_gt` | `!x.gt(y)`      |
    | `assert_ge` | `!x.ge(y)`      |
