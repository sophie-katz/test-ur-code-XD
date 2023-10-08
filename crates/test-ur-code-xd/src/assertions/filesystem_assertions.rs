// Copyright (c) 2023 Sophie Katz
//
// This file is part of test ur code XD.
//
// test ur code XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test ur code XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test ur code XD. If
// not, see <https://www.gnu.org/licenses/>.

//! Assertions for the filesystem.
//!
//! See
//! [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
//! for a usage guide.

use std::{fs, panic::Location, path::Path};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

#[doc(hidden)]
pub fn assert_path_exists_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// Asserts that the path exists on the filesystem.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_exists;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a file within it
/// # fs::File::create("some_file").unwrap();
/// #
/// assert_path_exists!("some_file");
/// ```
#[macro_export]
macro_rules! assert_path_exists {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path exists",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_is_file_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

/// Asserts that the path exists on the filesystem and is a regular file.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_is_file;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a file within it
/// # fs::File::create("some_file").unwrap();
/// #
/// assert_path_is_file!("some_file");
/// ```
#[macro_export]
macro_rules! assert_path_is_file {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is file",
            $crate::assertions::filesystem_assertions::assert_path_is_file_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_is_symlink_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_symlink()
}

/// Asserts that the path exists on the filesystem and is a symbolic link.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_is_symlink;
/// #
/// # #[cfg(target_family = "unix")]
/// # use std::os::unix::fs::symlink;
/// #
/// # #[cfg(target_family = "windows")]
/// # use std::os::windows::fs::symlink_file as symlink;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a file and a symlink within it
/// # fs::File::create("some_file").unwrap();
/// #
/// # symlink("some_file", "some_symlink").unwrap();
/// #
/// assert_path_is_symlink!("/etc/localtime");
/// ```
#[macro_export]
macro_rules! assert_path_is_symlink {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is symlink",
            $crate::assertions::filesystem_assertions::assert_path_is_symlink_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_is_dir_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

/// Asserts that the path exists on the filesystem and is a directory.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_is_dir;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a directory within it
/// # fs::create_dir("some_dir").unwrap();
/// #
/// assert_path_is_dir!("some_dir");
/// ```
#[macro_export]
macro_rules! assert_path_is_dir {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is directory",
            $crate::assertions::filesystem_assertions::assert_path_is_dir_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_is_relative_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_relative()
}

/// Asserts that the path is relative.
///
/// Note that the path must also exist for this assertion to pass.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_path_is_relative;
/// #
/// assert_path_is_relative!("../");
/// ```
#[macro_export]
macro_rules! assert_path_is_relative {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is relative",
            $crate::assertions::filesystem_assertions::assert_path_is_relative_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_is_absolute_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_absolute()
}

/// Asserts that the path is absolute.
///
/// Note that the path must also exist for this assertion to pass.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use test_ur_code_xd::assert_path_is_absolute;
/// #
/// # #[cfg(target_family = "unix")]
/// assert_path_is_absolute!("/etc");
/// ```
#[macro_export]
macro_rules! assert_path_is_absolute {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is absolute",
            $crate::assertions::filesystem_assertions::assert_path_is_absolute_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_starts_with_impl(path: impl AsRef<Path>, base: impl AsRef<Path>) -> bool {
    path.as_ref().starts_with(base.as_ref())
}

/// Asserts that the path starts with a base path.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * `base` - The base path to be used as a prefix.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_starts_with;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create some nested directories with a file at the end
/// # fs::create_dir("a").unwrap();
/// # fs::create_dir("a/b").unwrap();
/// # fs::File::create("a/b/c").unwrap();
/// #
/// assert_path_starts_with!("a/b/c", "a");
/// ```
#[macro_export]
macro_rules! assert_path_starts_with {
    ($path:expr, $base:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is starts with base",
            $crate::assertions::filesystem_assertions::assert_path_starts_with_impl(&$path, &$base),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
                    .with_argument("base", stringify!($base), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$base))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_path_ends_with_impl(path: impl AsRef<Path>, child: impl AsRef<Path>) -> bool {
    path.as_ref().ends_with(child.as_ref())
}

/// Asserts that the path ends with a child path.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * `child` - The child path to be used as a suffix.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::assert_path_ends_with;
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create some nested directories with a file at the end
/// # fs::create_dir("a").unwrap();
/// # fs::create_dir("a/b").unwrap();
/// # fs::File::create("a/b/c").unwrap();
/// #
/// assert_path_ends_with!("a/b/c", "b/c");
/// ```
#[macro_export]
macro_rules! assert_path_ends_with {
    ($path:expr, $child:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is ends with child",
            $crate::assertions::filesystem_assertions::assert_path_ends_with_impl(&$path, &$child),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$path))
                    .with_argument("child", stringify!($child), &::std::convert::AsRef::<::std::path::Path>::as_ref(&$child))
            }
            $(, $keys = $values)*
        )
    };
}

#[doc(hidden)]
pub fn assert_file_text_impl<OnTextType: FnOnce(String)>(
    path: impl AsRef<Path>,
    on_text: OnTextType,
) {
    if !path.as_ref().is_file() {
        PanicMessageBuilder::new("path is file", Location::caller())
            .with_argument("path", "--", &path.as_ref())
            .panic();
    }

    match fs::read_to_string(path.as_ref()) {
        Ok(file_text) => on_text(file_text),
        Err(error) => {
            PanicMessageBuilder::new(format!("error reading file: {}", error), Location::caller())
                .with_argument("path", "--", &path.as_ref())
                .panic()
        }
    }
}

/// Asserts that the file contains text that matches assertions.
///
/// **Warning:** this will read the whole file into memory as a string! Do not use on very large
/// files.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path of the file to read.
/// * `on_text` - A closure that takes the file content string as an argument.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::{assert_file_text, assert_eq};
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a file within it
/// # fs::write("hello_world_file.txt", "hello, world").unwrap();
/// #
/// assert_file_text!(
///     "hello_world_file.txt",
///     on_text = |text| {
///         assert_eq!(text, "hello, world");
///     }
/// );
/// ```
#[macro_export]
macro_rules! assert_file_text {
    ($path:expr, on_text = $on_text:expr) => {
        // TODO: Add a max file size limit
        $crate::assertions::filesystem_assertions::assert_file_text_impl($path, $on_text)
    };
}

#[doc(hidden)]
pub fn assert_file_text_raw_impl<OnTextType: FnOnce(&[u8])>(
    path: impl AsRef<Path>,
    on_text: OnTextType,
) {
    if !path.as_ref().is_file() {
        PanicMessageBuilder::new("path is file", Location::caller())
            .with_argument("path", "--", &path.as_ref())
            .panic();
    }

    match fs::read(path.as_ref()) {
        Ok(file_text) => on_text(&file_text),
        Err(error) => {
            PanicMessageBuilder::new(format!("error reading file: {}", error), Location::caller())
                .with_argument("path", "--", &path.as_ref())
                .panic()
        }
    }
}

/// Asserts that the raw file contains text that matches assertions.
///
/// **Warning:** this will read the whole file into memory as a string! Do not use on very large
/// files.
///
/// See
/// [sophie-katz.github.io/test-ur-code-XD/assertions/filesystem](https://sophie-katz.github.io/test-ur-code-XD/assertions/filesystem/)
/// for a usage guide.
///
/// # Arguments
///
/// * `path` - The path of the file to read.
/// * `on_text` - A closure that takes the file content byte array as an argument.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// # use std::{env, fs};
/// # use tempfile::tempdir;
/// # use test_ur_code_xd::{assert_file_text_raw, assert_eq};
/// #
/// # // Create a temporary directory and "cd" into it
/// # let temp_dir = tempdir().unwrap();
/// # env::set_current_dir(temp_dir.path()).unwrap();
/// #
/// # // Create a file within it
/// # fs::write("hello_world_file.txt", "hello, world").unwrap();
/// #
/// assert_file_text_raw!(
///     "hello_world_file.txt",
///     on_text = |text| {
///         assert_eq!(text, "hello, world".as_bytes());
///     }
/// );
/// ```
#[macro_export]
macro_rules! assert_file_text_raw {
    ($path:expr, on_text = $on_text:expr) => {
        // TODO: Add a max file size limit
        $crate::assertions::filesystem_assertions::assert_file_text_raw_impl($path, $on_text)
    };
}

#[cfg(test)]
mod tests {
    use crate::assert_eq;
    use std::{env, fs, io::Write};
    use tempfile::tempdir;

    #[cfg(target_family = "unix")]
    use std::os::unix::fs::symlink;

    #[cfg(target_family = "windows")]
    use std::os::windows::fs::symlink_file as symlink;

    #[test]
    fn assert_path_exists_passing_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_exists!("some_file");
    }

    #[test]
    fn assert_path_exists_passing_symlink() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();
        symlink("some_file", "some_symlink").unwrap();

        assert_path_exists!("some_symlink");
    }

    #[test]
    fn assert_path_exists_passing_directory() {
        let temp_dir = tempdir().unwrap();

        assert_path_exists!(temp_dir.path());
    }

    #[test]
    #[should_panic]
    fn assert_path_exists_failing_bad_name() {
        assert_path_exists!("a_file_that_does_not_exist");
    }

    #[test]
    #[should_panic]
    fn assert_path_exists_failing_bad_nest() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_exists!("some_file/bad_nesting");
    }

    #[test]
    fn assert_path_is_file_passing() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_file!("some_file");
    }

    #[test]
    fn assert_path_is_file_passing_symlink_to_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();
        symlink("some_file", "some_symlink").unwrap();

        assert_path_is_file!("some_symlink");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_file_failing_symlink_to_dir() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("some_dir").unwrap();
        symlink("some_dir", "some_symlink").unwrap();

        assert_path_is_file!("some_symlink");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_file_failing_directory() {
        let temp_dir = tempdir().unwrap();

        assert_path_is_file!(temp_dir.path());
    }

    #[test]
    #[should_panic]
    fn assert_path_is_file_failing_bad_name() {
        assert_path_is_file!("a_file_that_does_not_exist");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_file_failing_bad_nest() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_file!("some_file/bad_nesting");
    }

    #[test]
    fn assert_path_is_symlink_passing_symlink_to_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();
        symlink("some_file", "some_symlink").unwrap();

        assert_path_is_symlink!("some_symlink");
    }

    #[test]
    fn assert_path_is_symlink_passing_symlink_to_dir() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("some_dir").unwrap();
        symlink("some_dir", "some_symlink").unwrap();

        assert_path_is_symlink!("some_symlink");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_symlink_failing_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_symlink!("some_file");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_symlink_failing_directory() {
        let temp_dir = tempdir().unwrap();

        assert_path_is_symlink!(temp_dir.path());
    }

    #[test]
    #[should_panic]
    fn assert_path_is_symlink_failing_bad_name() {
        assert_path_is_symlink!("a_file_that_does_not_exist");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_symlink_failing_bad_nest() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_symlink!("some_file/bad_nesting");
    }

    #[test]
    fn assert_path_is_dir_passing() {
        let temp_dir = tempdir().unwrap();

        assert_path_is_dir!(temp_dir.path());
    }

    #[test]
    #[should_panic]
    fn assert_path_is_dir_failing_symlink_to_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();
        symlink("some_file", "some_symlink").unwrap();

        assert_path_is_dir!("some_symlink");
    }

    #[test]
    fn assert_path_is_dir_passing_symlink_to_dir() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("some_dir").unwrap();
        symlink("some_dir", "some_symlink").unwrap();

        assert_path_is_dir!("some_symlink");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_dir_failing_file() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_dir!("some_file");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_dir_failing_bad_name() {
        assert_path_is_dir!("a_file_that_does_not_exist");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_dir_failing_bad_nest() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_dir!("some_file/bad_nesting");
    }

    #[test]
    fn assert_path_is_relative_passing() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_relative!("some_file");
    }

    #[cfg(target_family = "unix")]
    #[test]
    #[should_panic]
    fn assert_path_is_relative_failing() {
        assert_path_is_relative!("/etc");
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn assert_path_is_relative_passing_at_root() {
        env::set_current_dir("/").unwrap();
        assert_path_is_relative!("etc");
    }

    #[test]
    #[should_panic]
    fn assert_path_is_absolute_failing() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::File::create("some_file").unwrap();

        assert_path_is_absolute!("some_file");
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn assert_path_is_absolute_passing() {
        assert_path_is_absolute!("/etc");
    }

    #[cfg(target_family = "unix")]
    #[test]
    #[should_panic]
    fn assert_path_is_absolute_failing_at_root() {
        env::set_current_dir("/").unwrap();
        assert_path_is_absolute!("etc");
    }

    #[test]
    fn assert_path_starts_with_passing_flat() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "a");
    }

    #[test]
    fn assert_path_starts_with_passing_nested() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "a/b");
    }

    #[test]
    fn assert_path_starts_with_passing_equal() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "a/b/c");
    }

    #[test]
    fn assert_path_starts_with_passing_empty() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "");
    }

    #[test]
    #[should_panic]
    fn assert_path_starts_with_failing_flat() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "d");
    }

    #[test]
    #[should_panic]
    fn assert_path_starts_with_failing_nested() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "a/d");
    }

    #[test]
    #[should_panic]
    fn assert_path_starts_with_failing_full() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "a/b/d");
    }

    #[test]
    #[should_panic]
    fn assert_path_starts_with_failing_wrong_prefix() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_starts_with!("a/b/c", "d/b/c");
    }

    #[test]
    fn assert_path_ends_with_passing_flat() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "c");
    }

    #[test]
    fn assert_path_ends_with_passing_nested() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "b/c");
    }

    #[test]
    fn assert_path_ends_with_passing_equal() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "a/b/c");
    }

    #[test]
    fn assert_path_ends_with_passing_empty() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "");
    }

    #[test]
    #[should_panic]
    fn assert_path_ends_with_failing_flat() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "d");
    }

    #[test]
    #[should_panic]
    fn assert_path_ends_with_failing_nested() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "d/c");
    }

    #[test]
    #[should_panic]
    fn assert_path_ends_with_failing_full() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "d/b/c");
    }

    #[test]
    #[should_panic]
    fn assert_path_ends_with_failing_wrong_suffix() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        fs::create_dir("a").unwrap();
        fs::create_dir("a/b").unwrap();
        fs::File::create("a/b/c").unwrap();

        assert_path_ends_with!("a/b/c", "a/b/d");
    }

    #[test]
    fn assert_file_text_passing() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text!(
            "some_file",
            on_text = |text| {
                assert_eq!(text, "hello, world");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_file_text_failing_assertion() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text!(
            "some_file",
            on_text = |text| {
                assert_eq!(text, "asdf");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_file_text_failing_bad_path() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text!(
            "asdf",
            on_text = |text| {
                assert_eq!(text, "hello, world");
            }
        );
    }

    #[test]
    fn assert_file_text_raw_passing() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text_raw!(
            "some_file",
            on_text = |text| {
                assert_eq!(text, b"hello, world");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_file_text_raw_failing_assertion() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text_raw!(
            "some_file",
            on_text = |text| {
                assert_eq!(text, b"asdf");
            }
        );
    }

    #[test]
    #[should_panic]
    fn assert_file_text_raw_failing_bad_path() {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        let mut file = fs::File::create("some_file").unwrap();
        file.write_all(b"hello, world").unwrap();

        assert_file_text_raw!(
            "asdf",
            on_text = |text| {
                assert_eq!(text, b"hello, world");
            }
        );
    }
}
