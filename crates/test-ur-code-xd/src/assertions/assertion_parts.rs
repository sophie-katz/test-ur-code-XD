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

//! Assertions are built in parts in test-ur-code-XD. This is to allow for assertion operations.
//!
//! # Negating assertions
//!
//! You can take any test-ur-code-XD assertion and negate it like this:
//!
//! ```ignore
//! !assert_str_contains!("hello, world", "hi");
//! ```
//!
//! This will pass because `"hello, world"` does not contain `"hi"`. The negation is done by the `!`
//! prefix.
//!
//! # Implementation
//!
//! This works because all assertion macros resolve to an expression that looks like this:
//!
//! ```plaintext
//! (assertion configuration) | (assertion executor)
//! ```
//!
//! The assertion configuration is a set of modifiers that can be applied to the assertion to change
//! its behavior. The assertion executor is the definition of what kind of assertion it is. It binds
//! the values being asserted and how they should be compared. The `|` operator is what actually
//! performs the assertion.
//!
//! So when an assertion is prefixed with `!`, the resolved expression turns into this:
//!
//! ```plaintext
//! !(assertion configuration) | (assertion executor)
//! ```
//!
//! Because `!` has higher precedence than `|`, it gets applied to the assertion configuration
//! before the assertion is executed. What the `!` operator is actually doing is modifying the
//! default configuration instance to set a negation flag.
//!
//! # Why choose `|`?
//!
//! The `|` operator is chosen because it is the
//! [lowest precedence operator that is not a comparison](https://doc.rust-lang.org/reference/expressions.html#expression-precedence).
//! This provides test-ur-code-XD with the greatest number of operators to potentially prefix
//! assertions with.
//!
//! It is important that a comparison operation not be used because rustc and Clippy both warn about
//! useless comparisons. For example, a comparison like this would issue a warning:
//!
//! ```
//! 5 == 5; // this comparison is useless
//! ```
//!
//! This warning is generated because the type of the expression is `bool`, but that boolean value
//! is never used.
//!
//! However, not-comparison operations can return the unit type `()`. If they do this, there is no
//! unused value and thus no warning. Here is a minimal example of this:
//!
//! ```
//! # use core::ops::Add;
//! #
//! struct MyType {
//!     x: i32
//! }
//!
//! impl Add<MyType> for MyType {
//!     type Output = ();
//!     
//!     fn add(self, _: Self) {}
//! }
//!
//! fn main() {
//!     let a = MyType { x: 5 };
//!     let b = MyType { x: 6 };
//!
//!     a + b; // the result of this operation is `()` so there is no unused value warning
//! }
//! ```
//!
//! # Avoiding Clippy warnings
//!
//! Even though the `|` operator returns the unit type `()`, Clippy will still warn about it having
//! no effect if it thinks that the statement has no side effects. Two struct initializations or
//! function calls as operands to the `|` operator will trigger this warning, so a workaround is
//! needed to make sure that every single assertion is not flagged.
//!
//! This part is a bit of a hack. If you look at the
//! [source code for the Clippy lint](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/no_effect.rs#L32),
//! you can see that the check is done by looking at the token tree types of the operands. One of
//! the token tree types that is excempt from this check is macros. Clippy will always assume that
//! macros have side effects to be on the safe side.
//!
//! As long as one of the operands to the `|` operator is a macro, Clippy will not warn about the
//! assertion. The executor was chosen because the `!` operator overrides the macro in Clippy. So
//! the actual resolved expression looks like this:
//!
//! ```ignore
//! make_assertion_part_config() | make_assertion_part_executor!(some_predicate, some_panic)
//! ```

use core::ops::BitOr;
use std::ops::Not;

/// The configuration for an assertion.
///
/// Contains modifiers that can be applied to the assertion to change its behavior. This is always
/// constructed through the [`make_assertion_part_config`] function.
#[derive(Default)]
pub struct AssertionConfig {
    pub negate: bool,
    pub assertion_description: &'static str,
    pub assertion_description_owned: String,
}

impl Not for AssertionConfig {
    type Output = AssertionConfig;

    fn not(self) -> Self::Output {
        AssertionConfig {
            negate: !self.negate,
            ..self
        }
    }
}

/// The executor for an assertion.
///
/// This is a binding of the predicate and the panic handler. This is always constructed through the
/// [`make_assertion_part_executor`] macro.
pub struct AssertionExecutor<PredicateType: Fn() -> bool, OnPanicType: Fn(AssertionConfig)> {
    predicate: PredicateType,
    on_panic: OnPanicType,
}

/// Constructs an assertion executor.
///
/// This function must always be wrapped in the [`make_assertion_part_executor`] macro to avoid
/// Clippy warnings.
pub fn make_assertion_part_executor_impl<
    PredicateType: Fn() -> bool,
    OnPanicType: Fn(AssertionConfig),
>(
    predicate: PredicateType,
    on_panic: OnPanicType,
) -> AssertionExecutor<PredicateType, OnPanicType> {
    AssertionExecutor {
        predicate,
        on_panic,
    }
}

/// Constructs an assertion executor.
///
/// # Arguments
///
/// * `predicate` - The predicate to execute. If the predicate returns `false` it will panic, unless
///                 the assertion is negated in which case the behavior is inverted.
/// * `on_panic` - This gets called if the predicate causes a panic.
#[macro_export]
macro_rules! make_assertion_part_executor {
    ($predicate: expr, $on_panic: expr) => {
        $crate::assertions::assertion_parts::make_assertion_part_executor_impl(
            $predicate, $on_panic,
        )
    };
}

impl<PredicateType: Fn() -> bool, OnPanicType: Fn(AssertionConfig)>
    BitOr<AssertionExecutor<PredicateType, OnPanicType>> for AssertionConfig
{
    type Output = ();

    fn bitor(self, executor: AssertionExecutor<PredicateType, OnPanicType>) {
        // Here is the truth table of whether or not to panic:
        //
        // | negate | predicate | panic |
        // |--------|-----------|-------|
        // | false  | false     | true  |
        // | false  | true      | false |
        // | true   | false     | false |
        // | true   | true      | true  |
        // |--------|-----------|-------|
        //
        // This truth table is the same as `negate == predicate`, which is used as the condition
        // below. It's hard to read, but efficient!
        if self.negate == (executor.predicate)() {
            (executor.on_panic)(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_panic() {
        AssertionConfig {
            ..Default::default()
        } | make_assertion_part_executor!(|| true, |config| panic!());
    }

    #[test]
    #[should_panic]
    fn does_panic() {
        AssertionConfig {
            ..Default::default()
        } | make_assertion_part_executor!(|| false, |config| panic!());
    }

    #[test]
    fn no_panic_negated_by_flag() {
        AssertionConfig {
            negate: true,
            ..Default::default()
        } | make_assertion_part_executor!(|| false, |config| panic!());
    }

    #[test]
    #[should_panic]
    fn does_panic_negated_by_flag() {
        AssertionConfig {
            negate: true,
            ..Default::default()
        } | make_assertion_part_executor!(|| true, |config| panic!());
    }

    #[test]
    fn no_panic_negated_by_operation() {
        !AssertionConfig {
            ..Default::default()
        } | make_assertion_part_executor!(|| false, |config| panic!());
    }

    #[test]
    #[should_panic]
    fn does_panic_negated_by_operation() {
        !AssertionConfig {
            ..Default::default()
        } | make_assertion_part_executor!(|| true, |config| panic!());
    }
}
