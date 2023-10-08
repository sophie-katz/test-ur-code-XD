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

# Recommended crates

There are a lot of testing crates in Rust that provide functionality that test ur code XD does not. Here are some recommendations:

## Utilities

| Crate                                                           | Description                                                                              |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [Nextest](https://crates.io/crates/cargo-nextest)               | A new, faster test runner for Rust                                                       |
| [Static assertions](https://crates.io/crates/static_assertions) | Compile-time assertions for Rust                                                         |
| [Fail](https://crates.io/crates/fail)                           | Fail point implementation for Rust                                                       |
| [`partial-io`](https://crates.io/crates/partial-io)             | Helpers for testing I/O behavior with partial, interrupted and blocking reads and writes |
| [`assert_cmd`](https://crates.io/crates/assert_cmd)             | Easy command initialization and assertions                                               |

## Test cases

| Crate                                                           | Description                                                                              |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [`serial_test`](https://crates.io/crates/serial_test)           | Allows for the creation of serialised Rust tests                                         |
| [`rstest`](https://crates.io/crates/rstest)                     | Fixture-based test framework for Rust                                                    |
| [Datatest](https://crates.io/crates/datatest)                   | Crate for supporting data-driven tests                                                   |

## Fuzzing and property testing

| Crate                                                           | Description                                                                              |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [Arbitrary](https://crates.io/crates/arbitrary)                 | Trait for generating structured data from arbitrary, unstructured input                  |
| [Proptest](https://crates.io/crates/proptest)                   | Property testing framework inspired by the Hypothesis framework for Python               |
| [Kani](https://crates.io/crates/kani-verifier)                  | Bit-precise model checker for Rust                                                       |
| [Bolero](https://crates.io/crates/bolero)                       | Fuzz and property testing front-end for Rust                                             |

## Mocking

| Crate                                                           | Description                                                                              |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [Mockall](https://crates.io/crates/mockall)                     | Powerful mock object library for Rust                                                    |
