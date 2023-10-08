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

The output assertion checks for `stdout` and `stderr` output from specific pieces of code:

=== "Text"

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

    The closure arguments `stdout` and `stderr` are of type `String`.

=== "Raw"

    ```rust hl_lines="6 7"
    assert_outputs_raw!(
        || {
            println!("hello, world");
        },
        on_stdout = |stdout| {
            assert_eq!(stdout, b"hello, world\n");
            //                 ↑
        }
    );
    ```

    The closure arguments `stdout` and `stderr` are of type `&[u8]`.

You can use `on_stdout = <closure>`, `on_stderr = <closure>`, or both to check the output streams.

!!! warning

    To use this assertion you need to configure Cargo as described below.

## Avoiding Cargo issues

To use this assertion, you must create or modify [the `.cargo/config.toml` file](https://doc.rust-lang.org/cargo/reference/config.html). Add this to it:

```toml
[env]
RUST_TEST_NOCAPTURE = "1"
RUST_TEST_THREADS   = "1"
```

By default, `cargo test` will capture output and also run tests in parallel. These both cause issues for `assert_outputs!(...)`.

* Cargo's output capturing means that `stdout` and `stderr` will be empty when the code actually runs. This means that `assert_outputs!(...)` becomes useless.

* Although the assertion is thread safe, it cannot stop other threads from using `stdout` and `stderr`. This means that `assert_outputs!(...)` becomes unreliable.
