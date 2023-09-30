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

# Output assertion

The output assertion checks for `stdout` and `stderr` output from specific pieces of code.

```rust
assert_outputs!(
    || {
        println!("hello, world");
    },
    on_stdout = |stdout| {
        assert_eq!(stdout, "hello, world\n");
    }
);
```

You can use `on_stdout = <closure>`, `on_stderr = <closure>`, or both to check both output streams.

!!! warning

    To use this assertion you need to configure Cargo as described below.

## Avoiding Cargo issues

By default, `cargo test` will capture output and also run tests in parallel. These both cause issues for `assert_outputs!(...)`.

* Capturing output means that `stdout` and `stderr` will be empty when the code under test runs. This means that `assert_outputs!(...)` becomes useless.

* Although the assertion is thread safe, it cannot stop other threads from using `stdout` and `stderr`.

You must add or modify the `.cargo/config.toml` file to the root of your Rust crate or workspace to have these lines:

```toml
[env]
RUST_TEST_NOCAPTURE = "1"
RUST_TEST_THREADS   = "1"
```
