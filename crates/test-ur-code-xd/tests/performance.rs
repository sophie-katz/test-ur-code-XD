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

#![allow(clippy::print_stdout)]

use std::time::Instant;

use test_ur_code_xd::assertions::config::Config;

const SAMPLE_SIZE: u32 = 50_000_000;

#[inline(never)]
fn id<T>(value: T) -> T {
    value
}

#[test]
fn benchmark_empty_loop() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {}

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_empty_create_config() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        let _: Config = Config {
            ..Default::default()
        };
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_test_ur_code_xd() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        test_ur_code_xd::assert!(id(true));
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_std() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        std::assert!(id(true));
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_if() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        // Ignore warnings because we are trying to simulate an assertion with an if statement
        // intentionally.
        #[allow(clippy::manual_assert, clippy::panic)]
        if !id(true) {
            panic!();
        }
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_eq_test_ur_code_xd() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        test_ur_code_xd::assert_eq!(id(3), id(3));
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_eq_std() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        std::assert_eq!(id(3), id(3));
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}

#[test]
fn benchmark_assert_eq_if() {
    let start = Instant::now();

    for _ in 0..SAMPLE_SIZE {
        // Ignore warnings because we are trying to simulate an assertion with an if statement
        // intentionally.
        #[allow(clippy::manual_assert, clippy::panic)]
        if !id(3).eq(&id(3)) {
            panic!();
        }
    }

    println!("{:?}ms elapsed", start.elapsed().as_millis());
}
