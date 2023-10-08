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

# Filesystem assertions

Filesystem assertions check properties of files and directories.

## Paths

These assertions check the properties of specific paths:

```rust
// Ensure that the path exists
assert_path_exists!("some_path");

// Ensure that the path exists and is a file
assert_path_is_file!("some_file");

// Ensure that the path exists and is a symlink
assert_path_is_symlink!("some_symlink");

// Ensure that the path exists and is a directory
assert_path_is_dir!("some_dir");

// Ensure that the path is relative
assert_path_is_relative!("some_path");

// Ensure that the path is absolute
assert_path_is_absolute!("/some_path");

// Ensure that the first path is prefixed by the second
assert_path_starts_with!("a/b/c", "a");

// Ensure that the first path is suffixed by the second
assert_path_ends_with!("a/b/c", "b/c");
```

## File text

There is an assertion to check the contents of a file:

```rust
assert_file_text!(
    "hello_world.txt",
    on_text = |text| {
        assert_eq!("hello, world");
    }
);
```

Instead of directly making an assertion on the file contents, it accept a closure. The closure's single argument is the file's contents as a `String`.

!!! warning

    This will read the whole file into memory at once. This is fine for smaller files but beware of running this for very large files!

## Details (advanced)

=== "Traits"

    All path arguments for these asserts do not have to be a particular path type, but must implement the `AsRef<Path>` trait.

=== "Panic conditions"

    | Assertion                    | Panic condition                               |
    | ---------------------------- | --------------------------------------------- |
    | `assert_path_exists`         | `!path.as_ref().exists()`                     |
    | `assert_path_is_file`        | `!path.as_ref().is_file()`                    |
    | `assert_path_is_symlink`     | `!path.as_ref().is_symlink()`                 |
    | `assert_path_is_dir`         | `!path.as_ref().is_dir()`                     |
    | `assert_path_is_relative`    | `!path.as_ref().is_relative()`                |
    | `assert_path_is_absolute`    | `!path.as_ref().is_absolute()`                |
    | `assert_path_is_starts_with` | `!path.as_ref().starts_with(prefix.as_ref())` |
    | `assert_path_is_ends_with`   | `!path.as_ref().ends_with(suffix.as_ref())`   |
