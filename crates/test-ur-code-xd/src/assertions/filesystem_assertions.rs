// Copyright (c) 2023 Sophie Katz
//
// This file is part of test-ur-code-XD.
//
// test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
// not, see <https://www.gnu.org/licenses/>.

//! Assertions for the filesystem.

use std::{fs, panic::Location, path::Path};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

#[doc(hidden)]
pub fn assert_path_exists_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// Asserts that the path exists on the filesystem.
///
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_exists!("/dev/null");
/// ```
#[macro_export]
macro_rules! assert_path_exists {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path exists",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_is_file!("/dev/null");
/// ```
#[macro_export]
macro_rules! assert_path_is_file {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is file",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_is_symlink!("/etc/localtime");
/// ```
#[macro_export]
macro_rules! assert_path_is_symlink {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is symlink",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_is_dir!("/etc");
/// ```
#[macro_export]
macro_rules! assert_path_is_dir {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is directory",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_is_relative!("../");
/// ```
#[macro_export]
macro_rules! assert_path_is_relative {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is relative",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_is_absolute!("/etc");
/// ```
#[macro_export]
macro_rules! assert_path_is_absolute {
    ($path:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is absolute",
            $crate::assertions::filesystem_assertions::assert_path_exists_impl(&$path),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * `base` - The base path to be used as a prefix.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_starts_with!("/etc/localtime", "/etc");
/// ```
#[macro_export]
macro_rules! assert_path_starts_with {
    ($path:expr, $base:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is starts with base",
            $crate::assertions::filesystem_assertions::assert_path_starts_with_impl(&$path, &$base),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
                    .with_argument("base", stringify!($base), &$base)
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
/// # Arguments
///
/// * `path` - The path to check.
/// * `child` - The child path to be used as a suffix.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_path_starts_with!("/etc/localtime", "localtime");
/// ```
#[macro_export]
macro_rules! assert_path_ends_with {
    ($path:expr, $child:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "path is ends with child",
            $crate::assertions::filesystem_assertions::assert_path_starts_with_impl(&$path, &$child),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("path", stringify!($path), &$path)
                    .with_argument("child", stringify!($child), &$child)
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
/// # Arguments
///
/// * `path` - The path of the file to read.
/// * `on_text` - A closure that takes the file content string as an argument.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
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
/// # Arguments
///
/// * `path` - The path of the file to read.
/// * `on_text` - A closure that takes the file content byte array as an argument.
/// * Optional keyword arguments for assertions.
///
/// # Example
///
/// ```
/// assert_file_text_raw!(
///     "hello_world_file.txt",
///     on_text = |text| {
///         assert_eq!(text, "hello, world");
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
