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

use std::{fs, panic::Location, path::Path};

use crate::utilities::panic_message_builder::PanicMessageBuilder;

pub fn assert_path_exists_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

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

pub fn assert_path_is_file_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

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

pub fn assert_path_is_symlink_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_symlink()
}

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

pub fn assert_path_is_dir_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

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

pub fn assert_path_is_relative_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_relative()
}

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

pub fn assert_path_is_absolute_impl(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_absolute()
}

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

pub fn assert_path_starts_with_impl(path: impl AsRef<Path>, base: impl AsRef<Path>) -> bool {
    path.as_ref().starts_with(base.as_ref())
}

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

pub fn assert_path_ends_with_impl(path: impl AsRef<Path>, child: impl AsRef<Path>) -> bool {
    path.as_ref().ends_with(child.as_ref())
}

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

pub fn assert_file_text<OnTextType: FnOnce(String)>(path: impl AsRef<Path>, on_text: OnTextType) {
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

#[macro_export]
macro_rules! assert_file_text {
    ($path:expr, on_text = $on_text:expr) => {
        $crate::assertions::filesystem_assertions::assert_file_text($path, $on_text)
    };
}
