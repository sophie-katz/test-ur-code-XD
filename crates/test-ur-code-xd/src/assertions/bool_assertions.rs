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

pub fn assert_impl(value: bool) -> bool {
    value
}

#[macro_export]
macro_rules! assert {
    ($value:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value is true",
            $crate::assertions::bool_assertions::assert_impl($value),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
            }
            $(, $keys = $values)*
        )
    };
}

pub fn assert_not_impl(value: bool) -> bool {
    !value
}

#[macro_export]
macro_rules! assert_not {
    ($value:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        $crate::assert_custom!(
            "value is false",
            $crate::assertions::bool_assertions::assert_not_impl($value),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
            }
            $(, $keys = $values)*
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::utilities::capture_output::capture_output;

    #[test]
    fn assert_passing() {
        let captured_outputs = capture_output(|| {
            assert!(true);
            assert!(false, negate = true);
        })
        .unwrap();

        std::assert!(captured_outputs.stdout.is_empty());
        std::assert!(captured_outputs.stderr.is_empty());
    }

    #[test]
    #[should_panic]
    fn assert_fail() {
        assert!(false);
    }

    #[test]
    #[should_panic]
    fn assert_fail_negated() {
        assert!(true, negate = true);
    }

    #[test]
    fn assert_not_passing() {
        let captured_outputs = capture_output(|| {
            assert_not!(false);
            assert_not!(true, negate = true);
        })
        .unwrap();

        std::assert!(captured_outputs.stdout.is_empty());
        std::assert!(captured_outputs.stderr.is_empty());
    }

    #[test]
    #[should_panic]
    fn assert_not_fail() {
        assert_not!(true);
    }

    #[test]
    #[should_panic]
    fn assert_not_fail_negated() {
        assert_not!(false, negate = true);
    }
}
