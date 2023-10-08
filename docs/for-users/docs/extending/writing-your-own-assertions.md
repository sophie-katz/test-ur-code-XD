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

# Writing your own assertions

This applies both when adding new assertions to this crate or when extending the library in
another crate.

## Implement the predicate

First write a function that represents the assertion predicate. It will take the inputs as
arguments and then return a boolean value. If it returns `true`, the assertion will pass. If
it returns a `false`, the assertion will fail and cause a panic. It will look something like
this:

=== "Non-copyable types"

    Unless the inputs implement the `Copy` trait, make sure to pass them in by reference:

    ```rust
    #[doc(hidden)]
    pub fn my_assertion_impl(
        input0: &InputType0,
        input1: &InputType1,
        /* ... */
    ) -> bool {
        /* ... */
    }
    ```

=== "Copyable types"

    If the inputs implement the `Copy` trait, you can pass them in by value:

    ```rust hl_lines="3 4"
    #[doc(hidden)]
    pub fn my_assertion_impl(
        input0: InputType0,
        input1: InputType1,
        /* ... */
    ) -> bool {
        /* ... */
    }
    ```

It has to be public so that the macro can access it, but since it is only for internal use we
mark it `#[doc(hidden)]`.

### Example

Let's reimplement the `assert_str_contains!(...)` macro from this crate as an example. It will be
a bit simplified from the actual implementation:

```rust
#[doc(hidden)]
pub fn assert_str_contains_impl(value: &str, substring: &str) -> bool {
    value.contains(substring)
}
```

Now that we have the perdicate, we can put together the macro itself.

## Declare the macro

Once we have the predicate function, we can wrap it in a macro. This will essentially be a
wrapper around `assert_custom!(...)` macro. It will look something like this:

```rust
#[macro_export]
macro_rules! my_assertion {
    ($input0:expr, $input1:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        ::test_ur_code_xd::assert_custom!(
            "a description of my assertion's predicate",
            $crate::my_assertion_impl($input0, $input1),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("input0", stringify!($input0), &$input0)
                    .with_argument("input1", stringify!($input1), &$input1)
            }
            $(, $keys = $values)*
        )
    }
}
```

This is a lot! Let's break it down.

### Macro arguments

The macro will take the inputs as `expr` arguments. This means that any Rust expression can be
used for the assertion inputs. Then we have some additional argument code:

```rust
$(, $keys:ident = $values:expr)* $(,)?
```

This is to accept any number of `<key> = <value>` arguments which are used to configure the
macro. When you pass `negate = true` into a macro as an additional argument, it goes through
this code.

### Use `assert_custom!(...)`

Then we have the call to `assert_custom!(...)`. We pass in all of our custom logic in as the
arguments to this macro. This is also the only place where the macro is different between
assertions written internally to test ur code XD and outside.

!!! note

    Macros written inside this crate will call this macro like `$crate::assert_custom(...)` while macros outside will write this `::test_ur_code_xd::assert_custom(...)`.

#### Predicate description

The first parameter is a description of the predicate:

```rust
"a description of my assertion's predicate"
```

This is used in the first line of the panic message. For example, the description for
`assert_str_contains!(...)` is `"value contains substring"`. It is important that the inputs are named
here so that the panic message's inputs can easily be understood.

#### Call the predicate function

The second parameter is a call to our predicate function:

=== "Non-copyable types"

    Unless the inputs implement the `Copy` trait, make sure to pass them in by reference:

    ```rust
    $crate::my_assertion_impl(&$input0, &$input1)
    ```

=== "Copyable types"

    If the inputs implement the `Copy` trait, you can pass them in by value:

    ```rust
    $crate::my_assertion_impl($input0, $input1)
    ```

#### Build the panic message

The third parameter is a closure which takes a `PanicMessageBuilder` and returns the same
instance. This is used to configure the panic message, usually to add debug information about
the inputs:

```rust
|panic_message_builder| {
    panic_message_builder
        .with_argument("input0", stringify!($input0), &$input0)
        .with_argument("input1", stringify!($input1), &$input1)
}
```

#### Forward keyword arguments

After the three arguments, we need to pass in any `<key> = <value>` arguments that we want to
forward from our macro invocation:

```rust
$(, $keys = $values)*
```

!!! warning

    Make sure not to put a comma before this! It will cause hard to debug compile-time errors.
    Make sure it's written like this:

    ```rust hl_lines="6"
    // ...
        |panic_message_builder| {
            panic_message_builder
                .with_argument("input0", stringify!($input0), &$input0)
                .with_argument("input1", stringify!($input1), &$input1)
        } // ← no comma here
        $(, $keys = $values)*
    // ...
    ```

Now your assertion should be functional!

### Example

Here's our simplified implementation of the `assert_str_contains!(...)` macro to use as an example:

```rust
#[doc(hidden)]
pub fn assert_str_contains_impl(value: &str, substring: &str) -> bool {
    value.contains(substring)
}

#[macro_export]
macro_rules! assert_str_contains {
    ($value:expr, $substring:expr $(, $keys:ident = $values:expr)* $(,)?) => {
        ::test_ur_code_xd::assert_custom!(
            "value contains substring",
            ::test_ur_code_xd::assertions::string_assertions::assert_str_contains_impl(
                $value,
                $substring
            ),
            |panic_message_builder| {
                panic_message_builder
                    .with_argument("value", stringify!($value), &$value)
                    .with_argument("substring", stringify!($substring), &$substring)
            }
            $(, $keys = $values)*
        )
    };
}
```

Then you can invoke it like this:

```rust
assert_str_contains!("hello, world", "world");
assert_str_contains!("hello, world", "asdf", negate = true);
```

## Writing your own assertion wrapper macros (advanced)

Some assertion macros do not explicitly make an assertion based on a predicate, but instead
accept a closure of other assertions. For example `assert_outputs!(...)` captures output but then
relies on closures to make assertions about the captured output:

```rust
assert_outputs!(
    || {
        println!("some text");
    },
    on_stdout = |stdout| {
        assert_eq!(stdout, "some text\n");
    }
);
```

The first step towards writing a macro like this is to, again, write an implementation function.
It will look something like this:

```rust
#[doc(hidden)]
pub fn my_assertion_wrapper_impl<
    ActionType: FnOnce(),
    ResultCallbackType: FnOnce(ResultType0),
>(
    action: ActionType,
    result_callback: ResultCallbackType,
) {
    // ...

    action();

    // ...

    result_callback(result);
}
```

Then write a macro to wrap this:

```rust
#[macro_export]
macro_rules! my_assertion_wrapper {
    (
        $action:expr,
        on_result = $on_result:expr
    ) => {
        $crate::my_assertion_wrapper_impl(
            $action,
            $on_result,
        )
    };
}
```

Ironically, this is simpler than implementing a new assertion with a predicate.

### Example

For a real world example of this, look at how a simplified version of `assert_panics!(...)` is implemented. Here is the predicate function:

```rust
#[doc(hidden)]
pub fn assert_panics_impl<
    ActionType: FnOnce() + UnwindSafe,
    MessageCallbackType: FnOnce(String),
>(
    action: ActionType,
    on_message: MessageCallbackType,
) {
    // If the action panics:
    if let Err(error) = panic::catch_unwind(AssertUnwindSafe(action)) {
        // Check the message
        on_message(panic_message::panic_message(&error).to_owned());
    } else {
        // Otherwise, fail the assertion
        PanicMessageBuilder::new("action panics", Location::caller()).panic();
    }
}
```

And then here is a simplified version of the macro:

```rust
#[macro_export]
macro_rules! assert_panics {
    ($action:expr, on_message = $on_message:expr) => {
        $crate::assert_panics_impl(
            $action,
            $on_message,
        )
    };
}
```
