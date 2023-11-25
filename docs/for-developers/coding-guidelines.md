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

# Coding guidelines

Please follow these guidelines when contributing so that the codebase can be as maintainable, readable, and stable as possible.

## Rust

There are many Clippy lints enabled and checked by CI, but there are some things that Clippy will not enforce.

* **Document `#[allow(...)]`:** If you are using `#[allow(...)]` to silence a lint, please document why you are doing so just above the allow statement, like this:

  ```rust
  #[allow(
    // Expect allowed because it should only fail if the function is written incorrectly, and is not
    // dependent on the arguments passed in.
    clippy::expect_used
  )]
  ```

* **Use `#[must_use]`:** Use this attribute liberally to add in additional sanity checks.

* **Break up large functions:** There's no concrete threshold for this, but if a function has more than moderate complexity, it should be broken up to make testing easier.
