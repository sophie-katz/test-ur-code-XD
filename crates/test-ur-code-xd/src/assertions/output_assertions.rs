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

use crate::utilities::capture_output::capture_output;

pub fn assert_outputs_impl<
    ActionType: FnOnce(),
    StdoutCallbackType: FnOnce(String),
    StderrCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_stdout: Option<StdoutCallbackType>,
    on_stderr: Option<StderrCallbackType>,
) {
    let captured_outputs = capture_output(action).unwrap();

    if let Some(on_stdout) = on_stdout {
        on_stdout(captured_outputs.stdout);
    }

    if let Some(on_stderr) = on_stderr {
        on_stderr(captured_outputs.stderr);
    }
}

#[macro_export]
macro_rules! assert_outputs {
    ($action:expr, on_stdout = $on_stdout:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some($on_stdout),
            ::std::option::Option::<FnOnce(String)>::None,
        )
    };

    ($action:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::<FnOnce(String)>::None,
            ::std::option::Option::Some($on_stderr),
        )
    };

    ($action:expr, on_stdout = $on_stdout:expr, on_stderr = $on_stderr:expr) => {
        $crate::assertions::output_assertions::assert_outputs_impl(
            $action,
            ::std::option::Option::Some($on_stdout),
            ::std::option::Option::Some($on_stderr),
        )
    };
}
