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

use console::{style, Color};
use float_cmp::approx_eq;
use panic_message::panic_message;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    panic::{self, UnwindSafe},
};

use crate::capture_output::capture_output;

struct PanicMessageBuilder {
    buffer: String,
}

impl PanicMessageBuilder {
    pub fn new(predicate_description: impl Display) -> Self {
        Self {
            buffer: format!(
                "{} assertion failed: {}",
                style("\u{26CC}").fg(Color::Red).bright().bold(),
                style(predicate_description)
                    .fg(Color::White)
                    .bright()
                    .bold()
            ),
        }
    }

    pub fn with_argument(
        mut self,
        argument_description: impl Display,
        value_description: impl Display,
        value: impl Debug,
    ) -> Self {
        let value_description_string = format!("{}", value_description);
        let value_string = format!("{:?}", value);

        self.buffer.push_str(
            format!(
                "\n  {} {}",
                style(format!("{}:", argument_description)).dim().bold(),
                style(value_description).fg(if value_description_string == value_string {
                    Color::Cyan
                } else {
                    Color::White
                }),
            )
            .as_str(),
        );

        if value_description_string != value_string {
            self.buffer.push_str(
                format!(
                    "\n       {} {:?}",
                    style("==").dim(),
                    style(value).fg(Color::Cyan)
                )
                .as_str(),
            );
        }

        self
    }

    pub fn with_failure_description(mut self, failure_description: Option<impl Display>) -> Self {
        if let Some(failure_description) = failure_description {
            self.buffer
                .push_str(format!("\n  note: {}", failure_description).as_str());
        }

        self
    }

    pub fn panic(self) -> String {
        panic!("{}", self.buffer);
    }
}

#[doc(hidden)]
pub fn assert_impl(
    value: bool,
    description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !value {
        PanicMessageBuilder::new("value is true")
            .with_argument("value", description, value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_not_impl(
    value: bool,
    description: impl Display,
    failure_description: Option<impl Display>,
) {
    if value {
        PanicMessageBuilder::new("value is false")
            .with_argument("value", description, value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_eq_impl<LHSType: Debug + PartialEq<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.eq(rhs_value) {
        PanicMessageBuilder::new("lhs == rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_ne_impl<LHSType: Debug + PartialEq<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if lhs_value.eq(rhs_value) {
        PanicMessageBuilder::new("lhs != rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_lt_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.lt(rhs_value) {
        PanicMessageBuilder::new("lhs < rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_le_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.le(rhs_value) {
        PanicMessageBuilder::new("lhs <= rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_gt_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.gt(rhs_value) {
        PanicMessageBuilder::new("lhs > rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_ge_impl<LHSType: Debug + PartialOrd<RHSType>, RHSType: Debug>(
    lhs_value: &LHSType,
    lhs_description: impl Display,
    rhs_value: &RHSType,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.ge(rhs_value) {
        PanicMessageBuilder::new("lhs >= rhs")
            .with_argument("lhs", lhs_description, lhs_value)
            .with_argument("rhs", rhs_description, rhs_value)
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_f32_eq_impl(
    lhs_value: f32,
    lhs_description: impl Display,
    rhs_value: f32,
    rhs_description: impl Display,
    ulps: i32,
    failure_description: Option<impl Display>,
) {
    if !approx_eq!(f32, lhs_value, rhs_value, ulps = ulps) {
        PanicMessageBuilder::new(format!(
            "lhs == rhs (within {} 32-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f32_ne_impl(
    lhs_value: f32,
    lhs_description: impl Display,
    rhs_value: f32,
    rhs_description: impl Display,
    ulps: i32,
    failure_description: Option<impl Display>,
) {
    if approx_eq!(f32, lhs_value, rhs_value, ulps = ulps) {
        PanicMessageBuilder::new(format!(
            "lhs != rhs (within {} 32-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f32_le_impl(
    lhs_value: f32,
    lhs_description: impl Display,
    rhs_value: f32,
    rhs_description: impl Display,
    ulps: i32,
    failure_description: Option<impl Display>,
) {
    if !(lhs_value < rhs_value || approx_eq!(f32, lhs_value, rhs_value, ulps = ulps)) {
        PanicMessageBuilder::new(format!(
            "lhs <= rhs (within {} 32-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f32_ge_impl(
    lhs_value: f32,
    lhs_description: impl Display,
    rhs_value: f32,
    rhs_description: impl Display,
    ulps: i32,
    failure_description: Option<impl Display>,
) {
    if !(lhs_value > rhs_value || approx_eq!(f32, lhs_value, rhs_value, ulps = ulps)) {
        PanicMessageBuilder::new(format!(
            "lhs >= rhs (within {} 32-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f64_eq_impl(
    lhs_value: f64,
    lhs_description: impl Display,
    rhs_value: f64,
    rhs_description: impl Display,
    ulps: i64,
    failure_description: Option<impl Display>,
) {
    if !approx_eq!(f64, lhs_value, rhs_value, ulps = ulps) {
        PanicMessageBuilder::new(format!(
            "lhs == rhs (within {} 64-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f64_ne_impl(
    lhs_value: f64,
    lhs_description: impl Display,
    rhs_value: f64,
    rhs_description: impl Display,
    ulps: i64,
    failure_description: Option<impl Display>,
) {
    if approx_eq!(f64, lhs_value, rhs_value, ulps = ulps) {
        PanicMessageBuilder::new(format!(
            "lhs != rhs (within {} 64-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f64_le_impl(
    lhs_value: f64,
    lhs_description: impl Display,
    rhs_value: f64,
    rhs_description: impl Display,
    ulps: i64,
    failure_description: Option<impl Display>,
) {
    if !(lhs_value < rhs_value || approx_eq!(f64, lhs_value, rhs_value, ulps = ulps)) {
        PanicMessageBuilder::new(format!(
            "lhs <= rhs (within {} 64-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_f64_ge_impl(
    lhs_value: f64,
    lhs_description: impl Display,
    rhs_value: f64,
    rhs_description: impl Display,
    ulps: i64,
    failure_description: Option<impl Display>,
) {
    if !(lhs_value > rhs_value || approx_eq!(f64, lhs_value, rhs_value, ulps = ulps)) {
        PanicMessageBuilder::new(format!(
            "lhs >= rhs (within {} 64-bit float ulp{})",
            ulps,
            if ulps == 1 { "" } else { "s" }
        ))
        .with_argument("lhs", lhs_description, lhs_value)
        .with_argument("rhs", rhs_description, rhs_value)
        .with_failure_description(failure_description)
        .panic();
    }
}

#[doc(hidden)]
pub fn assert_str_contains_impl(
    lhs_value: impl AsRef<str>,
    lhs_description: impl Display,
    rhs_value: impl AsRef<str>,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.as_ref().contains(rhs_value.as_ref()) {
        PanicMessageBuilder::new("lhs contains rhs")
            .with_argument("lhs", lhs_description, lhs_value.as_ref())
            .with_argument("rhs", rhs_description, rhs_value.as_ref())
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_str_starts_with_impl(
    lhs_value: impl AsRef<str>,
    lhs_description: impl Display,
    rhs_value: impl AsRef<str>,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.as_ref().starts_with(rhs_value.as_ref()) {
        PanicMessageBuilder::new("lhs starts with rhs")
            .with_argument("lhs", lhs_description, lhs_value.as_ref())
            .with_argument("rhs", rhs_description, rhs_value.as_ref())
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_str_ends_with_impl(
    lhs_value: impl AsRef<str>,
    lhs_description: impl Display,
    rhs_value: impl AsRef<str>,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    if !lhs_value.as_ref().ends_with(rhs_value.as_ref()) {
        PanicMessageBuilder::new("lhs ends with rhs")
            .with_argument("lhs", lhs_description, lhs_value.as_ref())
            .with_argument("rhs", rhs_description, rhs_value.as_ref())
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_str_matches_impl(
    lhs_value: impl AsRef<str>,
    lhs_description: impl Display,
    rhs_value: impl AsRef<str>,
    rhs_description: impl Display,
    failure_description: Option<impl Display>,
) {
    let rhs_value_regex = Regex::new(rhs_value.as_ref()).unwrap();

    if !rhs_value_regex.is_match(lhs_value.as_ref()) {
        PanicMessageBuilder::new("lhs matches regex rhs")
            .with_argument("lhs", lhs_description, lhs_value.as_ref())
            .with_argument("rhs", rhs_description, rhs_value.as_ref())
            .with_failure_description(failure_description)
            .panic();
    }
}

#[doc(hidden)]
pub fn assert_outputs_impl<
    ActionType: FnOnce(),
    StdoutCallbackType: FnOnce(String),
    StderrCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_stdout: Option<StdoutCallbackType>,
    on_stderr: Option<StderrCallbackType>,
) {
    let captured_outputs = capture_output(action);

    if let Some(on_stdout) = on_stdout {
        on_stdout(captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(captured_outputs.stderr);
    }
}

#[doc(hidden)]
pub fn assert_panics_impl<
    ActionType: FnOnce() + UnwindSafe,
    MessageCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_message: Option<MessageCallbackType>,
) {
    if let Err(error) = panic::catch_unwind(action) {
        if let Some(on_message) = on_message {
            on_message(panic_message(&error).to_owned());
        }
    } else {
        PanicMessageBuilder::new("action panics").panic();
    }
}

#[macro_export]
macro_rules! assert {
    ($value:expr $(,)?) => {
        $crate::assertions::assert_impl(
            $value,
            stringify!($value),
            ::std::option::Option::<&str>::None,
        );
    };
    ($value:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_impl(
            $value,
            stringify!($value),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_not {
    ($value:expr $(,)?) => {
        $crate::assertions::assert_not_impl(
            $value,
            stringify!($value),
            ::std::option::Option::<&str>::None,
        );
    };
    ($value:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_not_impl(
            $value,
            stringify!($value),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_eq_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_eq_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_ne_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_ne_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_lt {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_lt_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_lt_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_le {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_le_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_le_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_gt {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_gt_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_gt_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_ge {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_ge_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_ge_impl(
            &$lhs,
            stringify!($lhs),
            &$rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f32_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f32_eq_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f32_eq_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f32_ne {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f32_ne_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f32_ne_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f32_le {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f32_le_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f32_le_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f32_ge {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f32_ge_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f32_ge_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f64_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f64_eq_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f64_eq_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f64_ne {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f64_ne_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f64_ne_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f64_le {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f64_le_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f64_le_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_f64_ge {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_f64_ge_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_f64_ge_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            2,
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_str_contains {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_str_contains_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_str_contains_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_str_starts_with {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_str_starts_with_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_str_starts_with_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_str_ends_with {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_str_ends_with_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_str_ends_with_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_str_matches {
    ($lhs:expr, $rhs:expr $(,)?) => {
        $crate::assertions::assert_str_matches_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::<&str>::None,
        );
    };
    ($lhs:expr, $rhs:expr, $failure_description:expr $(,)?) => {
        $crate::assertions::assert_str_matches_impl(
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            ::std::option::Option::Some($failure_description),
        );
    };
}

#[macro_export]
macro_rules! assert_outputs {
    ($action:expr, on_stdout = $on_stdout:expr $(,)?) => {
        $crate::assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some($on_stdout),
            ::std::option::Option::<FnOnce(String)>::None,
        );
    };
    ($action:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::<FnOnce(String)>::None,
            ::std::option::Option::Some($on_stderr),
        );
    };
    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr $(,)?) => {
        $crate::assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some($on_stdout),
            ::std::option::Option::Some($on_stderr),
        );
    };
}

#[macro_export]
macro_rules! assert_panics {
    ($action:expr, on_message = $on_message:expr $(,)?) => {
        $crate::assertions::assert_panics_impl($action, ::std::option::Option::Some($on_message));
    };
    ($action:expr $(,)?) => {
        $crate::assertions::assert_panics_impl(
            $action,
            ::std::option::Option::<Box<dyn FnOnce(String)>>::None,
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::capture_output::capture_output;

    #[test]
    fn test_passing_no_output() {
        let captured_outputs = capture_output(|| {
            crate::assert!(true);
            crate::assert!(true, "failure message");
            crate::assert_not!(false);
            crate::assert_not!(false, "failure message");
            crate::assert_eq!(5, 5);
            crate::assert_eq!(5, 5, "failure message");
            crate::assert_eq!(5, 5u32);
            crate::assert_eq!(5, 5u32, "failure message");
            crate::assert_ne!(5, 6);
            crate::assert_ne!(5, 6, "failure message");
            crate::assert_ne!(5, 6u32);
            crate::assert_ne!(5, 6u32, "failure message");
            crate::assert_lt!(5, 6);
            crate::assert_lt!(5, 6, "failure message");
            crate::assert_lt!(5, 6u32);
            crate::assert_lt!(5, 6u32, "failure message");
            crate::assert_le!(5, 5);
            crate::assert_le!(5, 5, "failure message");
            crate::assert_le!(5, 5u32);
            crate::assert_le!(5, 5u32, "failure message");
            crate::assert_le!(5, 6);
            crate::assert_le!(5, 6, "failure message");
            crate::assert_le!(5, 6u32);
            crate::assert_le!(5, 6u32, "failure message");
            crate::assert_gt!(6, 5);
            crate::assert_gt!(6, 5, "failure message");
            crate::assert_gt!(6, 5u32);
            crate::assert_gt!(6, 5u32, "failure message");
            crate::assert_ge!(5, 5);
            crate::assert_ge!(5, 5, "failure message");
            crate::assert_ge!(5, 5u32);
            crate::assert_ge!(5, 5u32, "failure message");
            crate::assert_ge!(6, 5);
            crate::assert_ge!(6, 5, "failure message");
            crate::assert_ge!(6, 5u32);
            crate::assert_ge!(6, 5u32, "failure message");
            crate::assert_f32_eq!(5.0, 5.0);
            crate::assert_f32_eq!(5.0, 5.0, "failure message");
            crate::assert_f32_eq!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            crate::assert_f32_eq!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            crate::assert_f32_ne!(5.0, 6.0);
            crate::assert_f32_ne!(5.0, 6.0, "failure message");
            crate::assert_f32_le!(5.0, 5.0);
            crate::assert_f32_le!(5.0, 5.0, "failure message");
            crate::assert_f32_le!(5.0, 6.0);
            crate::assert_f32_le!(5.0, 6.0, "failure message");
            crate::assert_f32_le!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            crate::assert_f32_le!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            crate::assert_f32_ge!(5.0, 5.0);
            crate::assert_f32_ge!(5.0, 5.0, "failure message");
            crate::assert_f32_ge!(6.0, 5.0);
            crate::assert_f32_ge!(6.0, 5.0, "failure message");
            crate::assert_f32_ge!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            crate::assert_f32_ge!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25, "failure message");
            crate::assert_str_contains!("abc", "b");
            crate::assert_str_contains!("abc", "b", "failure message");
            crate::assert_str_starts_with!("abc", "a");
            crate::assert_str_starts_with!("abc", "a", "failure message");
            crate::assert_str_ends_with!("abc", "c");
            crate::assert_str_ends_with!("abc", "c", "failure message");
        });

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
    fn test_assert_fail_no_failure_description() {
        crate::assert!(false);
    }

    #[test]
    #[should_panic]
    fn test_assert_fail_with_failure_description() {
        crate::assert!(false, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_not_fail_no_failure_description() {
        crate::assert_not!(true);
    }

    #[test]
    #[should_panic]
    fn test_assert_not_fail_with_failure_description() {
        crate::assert_not!(true, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_fail_no_failure_description() {
        crate::assert_eq!(5, 6);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_fail_with_failure_description() {
        crate::assert_eq!(5, 6, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_fail_no_failure_description() {
        crate::assert_ne!(5, 5);
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_fail_with_failure_description() {
        crate::assert_ne!(5, 5, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail_eq_no_failure_description() {
        crate::assert_lt!(5, 5);
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail_eq_with_failure_description() {
        crate::assert_lt!(5, 5, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail_gt_no_failure_description() {
        crate::assert_lt!(6, 5);
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail_gt_with_failure_description() {
        crate::assert_lt!(6, 5, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_le_fail_gt_no_failure_description() {
        crate::assert_le!(6, 5);
    }

    #[test]
    #[should_panic]
    fn test_assert_le_fail_gt_with_failure_description() {
        crate::assert_le!(6, 5, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail_eq_no_failure_description() {
        crate::assert_gt!(5, 5);
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail_eq_with_failure_description() {
        crate::assert_gt!(5, 5, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail_lt_no_failure_description() {
        crate::assert_gt!(5, 6);
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail_lt_with_failure_description() {
        crate::assert_gt!(5, 6, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_ge_fail_lt_no_failure_description() {
        crate::assert_ge!(5, 6);
    }

    #[test]
    #[should_panic]
    fn test_assert_ge_fail_lt_with_failure_description() {
        crate::assert_ge!(5, 6, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_eq_fail_no_failure_description() {
        crate::assert_f32_eq!(5.0, 5.01);
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_eq_fail_with_failure_description() {
        crate::assert_f32_eq!(5.0, 5.01, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_ne_fail_no_failure_description() {
        crate::assert_f32_ne!(5.0, 5.0);
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_ne_fail_with_failure_description() {
        crate::assert_f32_ne!(5.0, 5.0, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_le_fail_no_failure_description() {
        crate::assert_f32_le!(5.01, 5.0);
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_le_fail_with_failure_description() {
        crate::assert_f32_le!(5.01, 5.0, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_ge_fail_no_failure_description() {
        crate::assert_f32_ge!(5.0, 5.01);
    }

    #[test]
    #[should_panic]
    fn test_assert_f32_ge_fail_with_failure_description() {
        crate::assert_f32_ge!(5.0, 5.01, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_eq_fail_no_failure_description() {
        crate::assert_f64_eq!(5.0, 5.01);
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_eq_fail_with_failure_description() {
        crate::assert_f64_eq!(5.0, 5.01, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_ne_fail_no_failure_description() {
        crate::assert_f64_ne!(5.0, 5.0);
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_ne_fail_with_failure_description() {
        crate::assert_f64_ne!(5.0, 5.0, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_le_fail_no_failure_description() {
        crate::assert_f64_le!(5.01, 5.0);
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_le_fail_with_failure_description() {
        crate::assert_f64_le!(5.01, 5.0, "failure description");
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_ge_fail_no_failure_description() {
        crate::assert_f64_ge!(5.0, 5.01);
    }

    #[test]
    #[should_panic]
    fn test_assert_f64_ge_fail_with_failure_description() {
        crate::assert_f64_ge!(5.0, 5.01, "failure description");
    }

    #[test]
    fn test_assert_outputs_pass() {
        crate::assert_outputs!(
            || {
                println!("Hello, world!");
                eprintln!("Hello, world!");
            },
            on_stdout = |stdout: String| {
                crate::assert_eq!(stdout, "Hello, world!\n");
            },
            on_stderr = |stderr: String| {
                crate::assert_eq!(stderr, "Hello, world!\n");
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_assert_outputs_fail_stdout() {
        crate::assert_outputs!(
            || {
                println!("Hello, world!");
                eprintln!("Hello, world!");
            },
            on_stdout = |stdout: String| {
                crate::assert_eq!(stdout, "Not hello, world!\n");
            },
            on_stderr = |stderr: String| {
                crate::assert_eq!(stderr, "Hello, world!\n");
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_assert_outputs_fail_stderr() {
        crate::assert_outputs!(
            || {
                println!("Hello, world!");
                eprintln!("Hello, world!");
            },
            on_stdout = |stdout: String| {
                crate::assert_eq!(stdout, "Hello, world!\n");
            },
            on_stderr = |stderr: String| {
                crate::assert_eq!(stderr, "Not hello, world!\n");
            }
        );
    }

    #[test]
    fn test_assert_panics_pass_no_message() {
        crate::assert_panics!(|| panic!("asdf"));
    }

    #[test]
    fn test_assert_panics_pass_with_message() {
        crate::assert_panics!(
            || panic!("asdf"),
            on_message = |message| {
                crate::assert_eq!(message, "asdf");
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_assert_panics_fail_no_message() {
        crate::assert_panics!(|| ());
    }

    #[test]
    #[should_panic]
    fn test_assert_panics_fail_with_message() {
        crate::assert_panics!(
            || (),
            on_message = |message| {
                crate::assert_eq!(message, "asdf");
            }
        );
    }
}
