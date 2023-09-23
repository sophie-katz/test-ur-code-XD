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

//! The assertions included within the crate as well as extendability for user-defined assertions.

pub mod config;

use float_cmp::approx_eq;
use panic_message::panic_message;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    fs,
    panic::{self, Location, UnwindSafe},
    path::Path,
};

use crate::utilities::{
    capture_output::capture_output, panic_message_builder::PanicMessageBuilder,
};

use self::config::Config;

pub fn assert_impl_predicate(value: bool) -> bool {
    value
}

// pub fn assert_impl_on_panic(config: AssertionConfig, value: bool, description: impl Display) -> ! {
//     PanicMessageBuilder::new("value is true", Location::caller())
//         .with_argument("value", description, value)
//         .with_assertion_description(config.assertion_description)
//         .with_assertion_description(config.assertion_description_owned)
//         .panic();
// }

#[macro_export]
macro_rules! assert {
    ($value:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        // // $crate::assertions::assertion_parts::make_assertion_part_config()
        // $crate::assertions::config::Config {
        //     $($keys: $values ,)*
        //     ..::std::default::Default::default()
        // }.execute_assertion(
        //     "value is true",
        //     $value,
        //     ::std::panic::Location::caller(),
        //     |panic_message_builder| {
        //         panic_message_builder
        //             .with_argument("value", stringify!($value), $value)
        //     },
        // )
        $crate::execute_assertion!("value is true", $value, |panic_message_builder| {
            panic_message_builder.with_argument("value", stringify!($value), $value)
        } $(, $keys = $values)*)
        // } | $crate::make_assertion_part_executor!(
        //     || $crate::assertions::assert_impl_predicate($value),
        //     |config| $crate::assertions::assert_impl_on_panic(
        //         config,
        //         $value,
        //         stringify!($value)
        //     )
        // )
    }; // ($value:expr, $failure_description:expr $(,)?) => {
       //     $crate::assertions::assertion_parts::make_assertion_part_config()
       //         | $crate::make_assertion_part_executor!(|| $value, || {
       //             $crate::utilities::panic_message_builder::PanicMessageBuilder::new(
       //                 "value is true",
       //                 ::std::panic::Location::caller(),
       //             )
       //             .with_argument("value", stringify!($value), $value)
       //             .with_failure_description(::std::option::Option::<&str>::Some($failure_description))
       //             .panic()
       //         })
       // };
}

// pub fn assert_not_impl(
//     value: bool,
//     description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if value {
//         PanicMessageBuilder::new("value is false", Location::caller())
//             .with_argument("value", description, value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_eq_impl<LHSType: Debug + PartialEq<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.eq(rhs_value) {
//         PanicMessageBuilder::new("lhs == rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_ne_impl<LHSType: Debug + PartialEq<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if lhs_value.eq(rhs_value) {
//         PanicMessageBuilder::new("lhs != rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_lt_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.lt(rhs_value) {
//         PanicMessageBuilder::new("lhs < rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_le_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.le(rhs_value) {
//         PanicMessageBuilder::new("lhs <= rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// #[track_caller]
// pub fn assert_gt_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.gt(rhs_value) {
//         PanicMessageBuilder::new("lhs > rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_ge_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
//     lhs_value: &LHSType,
//     lhs_description: impl Display,
//     rhs_value: &RHSType,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.ge(rhs_value) {
//         PanicMessageBuilder::new("lhs >= rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value)
//             .with_argument("rhs", rhs_description, rhs_value)
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_f32_eq_impl(
//     lhs_value: f32,
//     lhs_description: impl Display,
//     rhs_value: f32,
//     rhs_description: impl Display,
//     ulps: i32,
//     failure_description: Option<impl Display>,
// ) {
//     if !approx_eq!(f32, lhs_value, rhs_value, ulps = ulps) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs == rhs (within {} 32-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f32_ne_impl(
//     lhs_value: f32,
//     lhs_description: impl Display,
//     rhs_value: f32,
//     rhs_description: impl Display,
//     ulps: i32,
//     failure_description: Option<impl Display>,
// ) {
//     if approx_eq!(f32, lhs_value, rhs_value, ulps = ulps) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs != rhs (within {} 32-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f32_le_impl(
//     lhs_value: f32,
//     lhs_description: impl Display,
//     rhs_value: f32,
//     rhs_description: impl Display,
//     ulps: i32,
//     failure_description: Option<impl Display>,
// ) {
//     if !(lhs_value < rhs_value || approx_eq!(f32, lhs_value, rhs_value, ulps = ulps)) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs <= rhs (within {} 32-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f32_ge_impl(
//     lhs_value: f32,
//     lhs_description: impl Display,
//     rhs_value: f32,
//     rhs_description: impl Display,
//     ulps: i32,
//     failure_description: Option<impl Display>,
// ) {
//     if !(lhs_value > rhs_value || approx_eq!(f32, lhs_value, rhs_value, ulps = ulps)) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs >= rhs (within {} 32-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f64_eq_impl(
//     lhs_value: f64,
//     lhs_description: impl Display,
//     rhs_value: f64,
//     rhs_description: impl Display,
//     ulps: i64,
//     failure_description: Option<impl Display>,
// ) {
//     if !approx_eq!(f64, lhs_value, rhs_value, ulps = ulps) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs == rhs (within {} 64-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f64_ne_impl(
//     lhs_value: f64,
//     lhs_description: impl Display,
//     rhs_value: f64,
//     rhs_description: impl Display,
//     ulps: i64,
//     failure_description: Option<impl Display>,
// ) {
//     if approx_eq!(f64, lhs_value, rhs_value, ulps = ulps) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs != rhs (within {} 64-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f64_le_impl(
//     lhs_value: f64,
//     lhs_description: impl Display,
//     rhs_value: f64,
//     rhs_description: impl Display,
//     ulps: i64,
//     failure_description: Option<impl Display>,
// ) {
//     if !(lhs_value < rhs_value || approx_eq!(f64, lhs_value, rhs_value, ulps = ulps)) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs <= rhs (within {} 64-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_f64_ge_impl(
//     lhs_value: f64,
//     lhs_description: impl Display,
//     rhs_value: f64,
//     rhs_description: impl Display,
//     ulps: i64,
//     failure_description: Option<impl Display>,
// ) {
//     if !(lhs_value > rhs_value || approx_eq!(f64, lhs_value, rhs_value, ulps = ulps)) {
//         PanicMessageBuilder::new(
//             format!(
//                 "lhs >= rhs (within {} 64-bit float ulp{})",
//                 ulps,
//                 if ulps == 1 { "" } else { "s" }
//             ),
//             Location::caller(),
//         )
//         .with_argument("lhs", lhs_description, lhs_value)
//         .with_argument("rhs", rhs_description, rhs_value)
//         .with_failure_description(failure_description)
//         .panic();
//     }
// }

// pub fn assert_str_contains_impl(
//     lhs_value: impl AsRef<str>,
//     lhs_description: impl Display,
//     rhs_value: impl AsRef<str>,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.as_ref().contains(rhs_value.as_ref()) {
//         PanicMessageBuilder::new("lhs contains rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value.as_ref())
//             .with_argument("rhs", rhs_description, rhs_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_str_starts_with_impl(
//     lhs_value: impl AsRef<str>,
//     lhs_description: impl Display,
//     rhs_value: impl AsRef<str>,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.as_ref().starts_with(rhs_value.as_ref()) {
//         PanicMessageBuilder::new("lhs starts with rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value.as_ref())
//             .with_argument("rhs", rhs_description, rhs_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_str_ends_with_impl(
//     lhs_value: impl AsRef<str>,
//     lhs_description: impl Display,
//     rhs_value: impl AsRef<str>,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !lhs_value.as_ref().ends_with(rhs_value.as_ref()) {
//         PanicMessageBuilder::new("lhs ends with rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value.as_ref())
//             .with_argument("rhs", rhs_description, rhs_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_str_matches_impl(
//     lhs_value: impl AsRef<str>,
//     lhs_description: impl Display,
//     rhs_value: impl AsRef<str>,
//     rhs_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     let rhs_value_regex = Regex::new(rhs_value.as_ref()).unwrap();

//     if !rhs_value_regex.is_match(lhs_value.as_ref()) {
//         PanicMessageBuilder::new("lhs matches regex rhs", Location::caller())
//             .with_argument("lhs", lhs_description, lhs_value.as_ref())
//             .with_argument("rhs", rhs_description, rhs_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_exists_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().exists() {
//         PanicMessageBuilder::new("path exists", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_is_file_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().is_file() {
//         PanicMessageBuilder::new("path is file", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_is_symlink_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().is_symlink() {
//         PanicMessageBuilder::new("path is symlink", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_is_dir_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().is_dir() {
//         PanicMessageBuilder::new("path is directory", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_is_relative_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().is_relative() {
//         PanicMessageBuilder::new("path is relative", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_is_absolute_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().is_absolute() {
//         PanicMessageBuilder::new("path is absolute", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_starts_with_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     base_value: impl AsRef<Path>,
//     base_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().starts_with(base_value.as_ref()) {
//         PanicMessageBuilder::new("path starts with base", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_argument("base", base_description, base_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// pub fn assert_path_ends_with_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     base_value: impl AsRef<Path>,
//     base_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     if !path_value.as_ref().ends_with(base_value.as_ref()) {
//         PanicMessageBuilder::new("path ends with base", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_argument("base", base_description, base_value.as_ref())
//             .with_failure_description(failure_description)
//             .panic();
//     }
// }

// fn file_text_helper(path_value: impl AsRef<Path>, path_description: impl Display) -> String {
//     if !path_value.as_ref().is_file() {
//         PanicMessageBuilder::new("path is file", Location::caller())
//             .with_argument("path", &path_description, path_value.as_ref())
//             .panic();
//     }

//     match fs::read_to_string(path_value.as_ref()) {
//         Ok(file_text) => file_text,
//         Err(error) => {
//             PanicMessageBuilder::new(format!("error reading file: {}", error), Location::caller())
//                 .with_argument("path", path_description, path_value.as_ref())
//                 .panic()
//         }
//     }
// }

// pub fn assert_file_text_eq_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     text_value: impl AsRef<str>,
//     text_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     let file_text = file_text_helper(&path_value, &path_description);

//     if !file_text.eq(text_value.as_ref()) {
//         PanicMessageBuilder::new("read file text equals expected text", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_argument("read file text", "--", file_text)
//             .with_argument("expected text", text_description, text_value.as_ref())
//             .with_failure_description(failure_description.as_ref())
//             .panic();
//     }
// }

// pub fn assert_file_text_matches_impl(
//     path_value: impl AsRef<Path>,
//     path_description: impl Display,
//     pattern_value: impl AsRef<str>,
//     pattern_description: impl Display,
//     failure_description: Option<impl Display>,
// ) {
//     let file_text = file_text_helper(&path_value, &path_description);
//     let pattern_value_regex = Regex::new(pattern_value.as_ref()).unwrap();

//     if !pattern_value_regex.is_match(file_text.as_ref()) {
//         PanicMessageBuilder::new("read file text matches pattern", Location::caller())
//             .with_argument("path", path_description, path_value.as_ref())
//             .with_argument("read file text", "--", file_text)
//             .with_argument("pattern", pattern_description, pattern_value.as_ref())
//             .with_failure_description(failure_description.as_ref())
//             .panic();
//     }
// }

// pub fn assert_outputs_impl<
//     ActionType: FnOnce(),
//     StdoutCallbackType: FnOnce(String),
//     StderrCallbackType: FnOnce(String),
// >(
//     action: ActionType,
//     on_stdout: Option<StdoutCallbackType>,
//     on_stderr: Option<StderrCallbackType>,
// ) {
//     let captured_outputs = capture_output(action).unwrap();

//     if let Some(on_stdout) = on_stdout {
//         on_stdout(captured_outputs.stdout);
//     }

//     if let Some(on_stderr) = on_stderr {
//         on_stderr(captured_outputs.stderr);
//     }
// }

// pub fn assert_panics_impl<
//     ActionType: FnOnce() + UnwindSafe,
//     MessageCallbackType: FnOnce(String),
// >(
//     action: ActionType,
//     on_message: Option<MessageCallbackType>,
// ) {
//     if let Err(error) = panic::catch_unwind(action) {
//         if let Some(on_message) = on_message {
//             on_message(panic_message(&error).to_owned());
//         }
//     } else {
//         PanicMessageBuilder::new("action panics", Location::caller()).panic();
//     }
// }

// #[macro_export]
// macro_rules! assert {
//     ($value:expr $(,)?) => {
//         // #[allow(unused_comparisons)]
//         // $crate::assertions::AssertionConfig::default()
//         //     == $crate::assertions::AssertionExecutor {
//         //         predicate: || $value,
//         //         on_panic: || {
//         //             PanicMessageBuilder::new("value is true", ::std::panic::Location::caller())
//         //                 .with_argument("value", stringify!($value), $value)
//         //                 .panic();
//         //         },
//         //     };
//         $crate::assertions::assert_impl(
//             $value,
//             stringify!($value),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($value:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_impl(
//             $value,
//             stringify!($value),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_not {
//     ($value:expr $(,)?) => {
//         $crate::assertions::assert_not_impl(
//             $value,
//             stringify!($value),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($value:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_not_impl(
//             $value,
//             stringify!($value),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_eq {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_eq_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_eq_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_ne {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_ne_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_ne_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_lt {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_lt_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_lt_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_le {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_le_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_le_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_gt {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_gt_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_gt_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_ge {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_ge_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_ge_impl(
//             &$lhs,
//             stringify!($lhs),
//             &$rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f32_eq {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f32_eq_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f32_eq_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f32_ne {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f32_ne_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f32_ne_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f32_le {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f32_le_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f32_le_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f32_ge {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f32_ge_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f32_ge_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f64_eq {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f64_eq_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f64_eq_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f64_ne {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f64_ne_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f64_ne_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f64_le {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f64_le_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f64_le_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_f64_ge {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_f64_ge_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_f64_ge_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             2,
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_str_contains {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_str_contains_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_str_contains_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_str_starts_with {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_str_starts_with_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_str_starts_with_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_str_ends_with {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_str_ends_with_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_str_ends_with_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_str_matches {
//     ($lhs:expr, $rhs:expr $(,)?) => {
//         $crate::assertions::assert_str_matches_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_str_matches_impl(
//             $lhs,
//             stringify!($lhs),
//             $rhs,
//             stringify!($rhs),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_exists {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_exists_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_exists_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_is_file {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_is_file_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_is_file_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_is_symlink {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_is_symlink_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_is_symlink_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_is_dir {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_is_dir_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_is_dir_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_is_relative {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_is_relative_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_is_relative_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_is_absolute {
//     ($path:expr $(,)?) => {
//         $crate::assertions::assert_path_is_absolute_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_is_absolute_impl(
//             $path,
//             stringify!($path),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_starts_with {
//     ($path:expr, $base:expr $(,)?) => {
//         $crate::assertions::assert_path_starts_with_impl(
//             $path,
//             stringify!($path),
//             $base,
//             stringify!($base),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $base:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_starts_with_impl(
//             $path,
//             stringify!($path),
//             $base,
//             stringify!($base),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_path_ends_with {
//     ($path:expr, $base:expr $(,)?) => {
//         $crate::assertions::assert_path_ends_with_impl(
//             $path,
//             stringify!($path),
//             $base,
//             stringify!($base),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $base:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_path_ends_with_impl(
//             $path,
//             stringify!($path),
//             $base,
//             stringify!($base),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_file_text_eq {
//     ($path:expr, $text:expr $(,)?) => {
//         $crate::assertions::assert_file_text_eq_impl(
//             $path,
//             stringify!($path),
//             $text,
//             stringify!($text),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $text:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_file_text_eq_impl(
//             $path,
//             stringify!($path),
//             $text,
//             stringify!($text),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_file_text_matches {
//     ($path:expr, $pattern:expr $(,)?) => {
//         $crate::assertions::assert_file_text_matches_impl(
//             $path,
//             stringify!($path),
//             $pattern,
//             stringify!($pattern),
//             ::std::option::Option::<&str>::None,
//         );
//     };
//     ($path:expr, $pattern:expr, $failure_description:expr $(,)?) => {
//         $crate::assertions::assert_file_text_matches_impl(
//             $path,
//             stringify!($path),
//             $pattern,
//             stringify!($pattern),
//             ::std::option::Option::Some($failure_description),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_outputs {
//     ($action:expr, on_stdout = $on_stdout:expr $(,)?) => {
//         $crate::assertions::assert_outputs_impl(
//             $action,
//             ::std::option::Option::Some($on_stdout),
//             ::std::option::Option::<FnOnce(String)>::None,
//         );
//     };
//     ($action:expr, on_stderr = $on_stderr:expr $(,)?) => {
//         $crate::assertions::assert_outputs_impl(
//             $action,
//             ::std::option::Option::<FnOnce(String)>::None,
//             ::std::option::Option::Some($on_stderr),
//         );
//     };
//     ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr $(,)?) => {
//         $crate::assertions::assert_outputs_impl(
//             $action,
//             ::std::option::Option::Some($on_stdout),
//             ::std::option::Option::Some($on_stderr),
//         );
//     };
// }

// #[macro_export]
// macro_rules! assert_panics {
//     ($action:expr, on_message = $on_message:expr $(,)?) => {
//         $crate::assertions::assert_panics_impl($action, ::std::option::Option::Some($on_message));
//     };
//     ($action:expr $(,)?) => {
//         $crate::assertions::assert_panics_impl(
//             $action,
//             ::std::option::Option::<Box<dyn FnOnce(String)>>::None,
//         );
//     };
// }

#[cfg(test)]
mod tests {
    use crate::utilities::capture_output::capture_output;

    #[test]
    fn test_passing_no_output() {
        let captured_outputs = capture_output(|| {
            crate::assert!(true);
            crate::assert!(true, assertion_description = "assertion description");
            crate::assert!(
                true,
                assertion_description_owned = "assertion description".to_owned()
            );
            crate::assert!(false, negate = true);
            crate::assert!(
                false,
                negate = true,
                assertion_description = "assertion description"
            );
            crate::assert!(
                false,
                negate = true,
                assertion_description_owned = "assertion description".to_owned()
            );
            // !crate::assert!(false);
            // !crate::assert!(false, assertion_description = "assertion description");
            // !crate::assert!(
            //     false,
            //     assertion_description_owned = "assertion description".to_owned()
            // );
            // crate::assert_not!(false);
            // crate::assert_not!(false, "failure message");
            // crate::assert_eq!(5, 5);
            // crate::assert_eq!(5, 5, "failure message");
            // crate::assert_eq!(5, 5u32);
            // crate::assert_eq!(5, 5u32, "failure message");
            // crate::assert_ne!(5, 6);
            // crate::assert_ne!(5, 6, "failure message");
            // crate::assert_ne!(5, 6u32);
            // crate::assert_ne!(5, 6u32, "failure message");
            // crate::assert_lt!(5, 6);
            // crate::assert_lt!(5, 6, "failure message");
            // crate::assert_lt!(5, 6u32);
            // crate::assert_lt!(5, 6u32, "failure message");
            // crate::assert_le!(5, 5);
            // crate::assert_le!(5, 5, "failure message");
            // crate::assert_le!(5, 5u32);
            // crate::assert_le!(5, 5u32, "failure message");
            // crate::assert_le!(5, 6);
            // crate::assert_le!(5, 6, "failure message");
            // crate::assert_le!(5, 6u32);
            // crate::assert_le!(5, 6u32, "failure message");
            // crate::assert_gt!(6, 5);
            // crate::assert_gt!(6, 5, "failure message");
            // crate::assert_gt!(6, 5u32);
            // crate::assert_gt!(6, 5u32, "failure message");
            // crate::assert_ge!(5, 5);
            // crate::assert_ge!(5, 5, "failure message");
            // crate::assert_ge!(5, 5u32);
            // crate::assert_ge!(5, 5u32, "failure message");
            // crate::assert_ge!(6, 5);
            // crate::assert_ge!(6, 5, "failure message");
            // crate::assert_ge!(6, 5u32);
            // crate::assert_ge!(6, 5u32, "failure message");
            // crate::assert_f32_eq!(5.0, 5.0);
            // crate::assert_f32_eq!(5.0, 5.0, "failure message");
            // crate::assert_f32_eq!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            // crate::assert_f32_eq!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            // crate::assert_f32_ne!(5.0, 6.0);
            // crate::assert_f32_ne!(5.0, 6.0, "failure message");
            // crate::assert_f32_le!(5.0, 5.0);
            // crate::assert_f32_le!(5.0, 5.0, "failure message");
            // crate::assert_f32_le!(5.0, 6.0);
            // crate::assert_f32_le!(5.0, 6.0, "failure message");
            // crate::assert_f32_le!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            // crate::assert_f32_le!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            // crate::assert_f32_ge!(5.0, 5.0);
            // crate::assert_f32_ge!(5.0, 5.0, "failure message");
            // crate::assert_f32_ge!(6.0, 5.0);
            // crate::assert_f32_ge!(6.0, 5.0, "failure message");
            // crate::assert_f32_ge!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            // crate::assert_f32_ge!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            // crate::assert_str_contains!("abc", "b");
            // crate::assert_str_contains!("abc", "b", "failure message");
            // crate::assert_str_starts_with!("abc", "a");
            // crate::assert_str_starts_with!("abc", "a", "failure message");
            // crate::assert_str_ends_with!("abc", "c");
            // crate::assert_str_ends_with!("abc", "c", "failure message");
        })
        .unwrap();

        std::assert!(
            captured_outputs.stdout.is_empty(),
            "stdout:\n<<<{}>>>",
            captured_outputs.stdout
        );
        std::assert!(
            captured_outputs.stderr.is_empty(),
            "stderr:\n<<<{}>>>",
            captured_outputs.stderr
        );
    }

    #[test]
    #[should_panic]
    fn test_assert_fail_no_assertion_description() {
        crate::assert!(false);
    }

    #[test]
    #[should_panic]
    fn test_assert_fail_with_assertion_description() {
        crate::assert!(false, assertion_description = "assertion description");
    }

    #[test]
    #[should_panic]
    fn test_assert_fail_with_assertion_description_owned() {
        crate::assert!(
            false,
            assertion_description_owned = "assertion description".to_owned()
        );
    }

    // #[test]
    // #[should_panic]
    // fn test_assert_not_fail_no_failure_description() {
    //     crate::assert_not!(true);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_not_fail_with_failure_description() {
    //     crate::assert_not!(true, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_eq_fail_no_failure_description() {
    //     crate::assert_eq!(5, 6);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_eq_fail_with_failure_description() {
    //     crate::assert_eq!(5, 6, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_ne_fail_no_failure_description() {
    //     crate::assert_ne!(5, 5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_ne_fail_with_failure_description() {
    //     crate::assert_ne!(5, 5, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_lt_fail_eq_no_failure_description() {
    //     crate::assert_lt!(5, 5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_lt_fail_eq_with_failure_description() {
    //     crate::assert_lt!(5, 5, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_lt_fail_gt_no_failure_description() {
    //     crate::assert_lt!(6, 5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_lt_fail_gt_with_failure_description() {
    //     crate::assert_lt!(6, 5, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_le_fail_gt_no_failure_description() {
    //     crate::assert_le!(6, 5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_le_fail_gt_with_failure_description() {
    //     crate::assert_le!(6, 5, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_gt_fail_eq_no_failure_description() {
    //     crate::assert_gt!(5, 5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_gt_fail_eq_with_failure_description() {
    //     crate::assert_gt!(5, 5, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_gt_fail_lt_no_failure_description() {
    //     crate::assert_gt!(5, 6);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_gt_fail_lt_with_failure_description() {
    //     crate::assert_gt!(5, 6, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_ge_fail_lt_no_failure_description() {
    //     crate::assert_ge!(5, 6);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_ge_fail_lt_with_failure_description() {
    //     crate::assert_ge!(5, 6, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_eq_fail_no_failure_description() {
    //     crate::assert_f32_eq!(5.0, 5.01);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_eq_fail_with_failure_description() {
    //     crate::assert_f32_eq!(5.0, 5.01, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_ne_fail_no_failure_description() {
    //     crate::assert_f32_ne!(5.0, 5.0);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_ne_fail_with_failure_description() {
    //     crate::assert_f32_ne!(5.0, 5.0, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_le_fail_no_failure_description() {
    //     crate::assert_f32_le!(5.01, 5.0);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_le_fail_with_failure_description() {
    //     crate::assert_f32_le!(5.01, 5.0, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_ge_fail_no_failure_description() {
    //     crate::assert_f32_ge!(5.0, 5.01);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f32_ge_fail_with_failure_description() {
    //     crate::assert_f32_ge!(5.0, 5.01, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_eq_fail_no_failure_description() {
    //     crate::assert_f64_eq!(5.0, 5.01);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_eq_fail_with_failure_description() {
    //     crate::assert_f64_eq!(5.0, 5.01, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_ne_fail_no_failure_description() {
    //     crate::assert_f64_ne!(5.0, 5.0);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_ne_fail_with_failure_description() {
    //     crate::assert_f64_ne!(5.0, 5.0, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_le_fail_no_failure_description() {
    //     crate::assert_f64_le!(5.01, 5.0);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_le_fail_with_failure_description() {
    //     crate::assert_f64_le!(5.01, 5.0, "failure description");
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_ge_fail_no_failure_description() {
    //     crate::assert_f64_ge!(5.0, 5.01);
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_f64_ge_fail_with_failure_description() {
    //     crate::assert_f64_ge!(5.0, 5.01, "failure description");
    // }

    // #[test]
    // fn test_assert_outputs_pass() {
    //     crate::assert_outputs!(
    //         || {
    //             println!("Hello, world!");
    //             eprintln!("Hello, world!");
    //         },
    //         on_stdout = |stdout: String| {
    //             crate::assert_eq!(stdout, "Hello, world!\n");
    //         },
    //         on_stderr = |stderr: String| {
    //             crate::assert_eq!(stderr, "Hello, world!\n");
    //         }
    //     );
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_outputs_fail_stdout() {
    //     crate::assert_outputs!(
    //         || {
    //             println!("Hello, world!");
    //             eprintln!("Hello, world!");
    //         },
    //         on_stdout = |stdout: String| {
    //             crate::assert_eq!(stdout, "Not hello, world!\n");
    //         },
    //         on_stderr = |stderr: String| {
    //             crate::assert_eq!(stderr, "Hello, world!\n");
    //         }
    //     );
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_outputs_fail_stderr() {
    //     crate::assert_outputs!(
    //         || {
    //             println!("Hello, world!");
    //             eprintln!("Hello, world!");
    //         },
    //         on_stdout = |stdout: String| {
    //             crate::assert_eq!(stdout, "Hello, world!\n");
    //         },
    //         on_stderr = |stderr: String| {
    //             crate::assert_eq!(stderr, "Not hello, world!\n");
    //         }
    //     );
    // }

    // #[test]
    // fn test_assert_panics_pass_no_message() {
    //     crate::assert_panics!(|| panic!("asdf"));
    // }

    // #[test]
    // fn test_assert_panics_pass_with_message() {
    //     crate::assert_panics!(
    //         || panic!("asdf"),
    //         on_message = |message| {
    //             crate::assert_eq!(message, "asdf");
    //         }
    //     );
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_panics_fail_no_message() {
    //     crate::assert_panics!(|| ());
    // }

    // #[test]
    // #[should_panic]
    // fn test_assert_panics_fail_with_message() {
    //     crate::assert_panics!(
    //         || (),
    //         on_message = |message| {
    //             crate::assert_eq!(message, "asdf");
    //         }
    //     );
    // }

    // #[derive(Default)]
    // struct Config {
    //     negate: bool,
    // }

    // struct Executor;

    // impl Executor {
    //     pub fn execute(&self, config: &Config) -> bool {
    //         config.negate
    //     }
    // }

    // #[allow(dead_code)]
    // enum AssertionPart<PredicateType: Fn() -> bool, OnPanicType: Fn()> {
    //     Config {
    //         negate: bool,
    //     },
    //     Executor {
    //         predicate: PredicateType,
    //         on_panic: OnPanicType,
    //     },
    // }

    // fn assertion_part_config(negate: bool) -> AssertionPart<fn() -> bool, fn()> {
    //     AssertionPart::Config { negate }
    // }

    // fn assertion_part_executor<PredicateType: Fn() -> bool, OnPanicType: Fn()>(
    //     predicate: PredicateType,
    //     on_panic: OnPanicType,
    // ) -> AssertionPart<PredicateType, OnPanicType> {
    //     AssertionPart::Executor {
    //         predicate,
    //         on_panic,
    //     }
    // }

    // macro_rules! assertion_part_executor_wrapper {
    //     ($predicate:expr, $on_panic:expr) => {
    //         assertion_part_executor($predicate, $on_panic)
    //     };
    // }

    // impl<PredicateType: Fn() -> bool, OnPanicType: Fn()> core::ops::Not
    //     for AssertionPart<PredicateType, OnPanicType>
    // {
    //     type Output = Self;

    //     fn not(self) -> Self::Output {
    //         match self {
    //             Self::Config { negate } => Self::Config { negate: !negate },
    //             _ => panic!(),
    //         }
    //     }
    // }

    // impl<PredicateType: Fn() -> bool, OnPanicType: Fn()> core::ops::BitOr
    //     for AssertionPart<PredicateType, OnPanicType>
    // {
    //     type Output = ();

    //     fn bitor(self, rhs: Self) -> Self::Output {
    //         if let Self::Config { negate } = self {
    //             if let Self::Executor { .. } = rhs {
    //                 println!("negate = {}", negate);
    //                 return;
    //             }
    //         }

    //         panic!();
    //     }
    //     // fn rem_assign(&mut self, rhs: Self) {
    //     //     if let Self::Config { negate } = self {
    //     //         if let Self::Executor = rhs {
    //     //             println!("negate = {}", negate);
    //     //         }
    //     //     }

    //     //     panic!();
    //     // }
    // }

    // // impl PartialEq<Executor> for Config {
    // //     fn eq(&self, other: &Executor) -> bool {
    // //         other.execute(self)
    // //     }
    // // }

    // // macro_rules! configurable_assert {
    // //     () => {
    // //         AssertionPart::Config { negate: false } | AssertionPart::Executor
    // //     };
    // // }

    // #[test]
    // fn test_config_ops() {
    //     // configurable_assert!();

    //     // !configurable_assert!();

    //     // AssertionPart::Config { negate: false } | AssertionPart::Executor;

    //     // !AssertionPart::Config { negate: false } | AssertionPart::Executor;

    //     assertion_part_config(false) | assertion_part_executor_wrapper!(|| true, || panic!());

    //     !assertion_part_config(false) | assertion_part_executor_wrapper!(|| false, || panic!());
    // }

    // #[test]
    // fn test_old_assertion_performance() {
    //     for _ in 0..100_000_000 {
    //         assert!(true);
    //     }
    // }

    // // #[test]
    // // #[allow(unused_must_use)]
    // // #[allow(clippy::unnecessary_operation)]
    // // fn test_new_assertion_performance() {
    // //     for _ in 0..100_000_000 {
    // //         super::AssertionConfig::default()
    // //             == super::AssertionExecutor {
    // //                 predicate: || true,
    // //                 on_panic: || {
    // //                     PanicMessageBuilder::new("value is true", std::panic::Location::caller())
    // //                         .with_argument("value", "true", true)
    // //                         // .with_failure_description(None)
    // //                         .panic();
    // //                 },
    // //             };
    // //     }
    // // }
}
