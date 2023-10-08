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

#[cfg(test)]
#[macro_use]
extern crate test_ur_code_xd;

mod home {
    #[test_with_parameter_values(
        x = [5, 6, 7],
        y = [1, 2]
    )]
    fn example(x: i32, y: i32) {
        // This will permute the values and automatically run all of these cases:
        //   x == 5, y == 1
        //   x == 5, y == 2
        //   x == 6, y == 1
        //   x == 6, y == 2
        //   x == 7, y == 1
        //   x == 7, y == 2
        assert!(x + y > 0);
    }
}

mod getting_started {
    #[test]
    fn example() {
        let hello_world = "hello, world";
        assert_str_eq!(hello_world, "hello, world");
    }
}

mod assertions {
    mod boolean {
        #[test]
        fn example() {
            let value = true;

            // Ensure that value is true
            assert!(value);

            let value = false;

            // Ensure that value is false
            assert_not!(value);
        }
    }

    mod arithemtic {
        #[test]
        fn example_equality() {
            let x = 5;
            let y = 5;

            // Ensure that the values are equal
            assert_eq!(x, y);

            let y = 6;

            // Ensure that the values are inequal
            assert_ne!(x, y);
        }

        #[test]
        fn example_ordering() {
            let x = 4;
            let y = 5;

            // Ensure that x is less than y
            assert_lt!(x, y);

            // Ensure that x is less than or equal to y
            assert_le!(x, y);

            let x = 5;
            let y = 4;

            // Ensure that x is greater than y
            assert_gt!(x, y);

            // Ensure that x is greater than or equal to y
            assert_ge!(x, y);
        }
    }

    mod string {
        #[test]
        fn example() {
            let hello_world = "hello, world";

            // Compare two strings and diff the results
            assert_str_eq!(hello_world, "hello, world");

            // Ensure that the second string is contained within the first
            assert_str_contains!("hello, world", "hello");

            // Ensure that the first string starts with the second
            assert_str_starts_with!("hello, world", "hello");

            // Ensure that the first string ends with the second
            assert_str_ends_with!("hello, world", "world");

            // Ensure that the first string matches the second regex
            assert_str_matches!("hello, world", "[a-z, ]+");
        }
    }

    mod float {
        #[test]
        #[should_panic]
        fn example_failing() {
            assert_eq!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
        }

        #[test]
        fn example_relative() {
            let x = 5.0;
            let y = 5.0;

            assert_f32_eq!(
                x,
                y,
                relative_epsilon = f32::EPSILON,
                epsilon_near_zero = 1e-6
            );
        }

        #[test]
        fn example_ulps() {
            let x = 5.0;
            let y = 5.0;

            assert_f32_eq!(x, y, ulps = 1, epsilon_near_zero = 1e-6);
        }
    }

    mod filesystem {
        use std::{env, fs, io::Write};
        use tempfile::tempdir;

        #[cfg(target_family = "unix")]
        use std::os::unix::fs::symlink;

        #[cfg(target_family = "windows")]
        use std::os::windows::fs::symlink_file as symlink;

        #[test]
        fn example() {
            let temp_dir = tempdir().unwrap();
            env::set_current_dir(temp_dir.path()).unwrap();
            fs::File::create("some_path").unwrap();
            fs::File::create("some_file").unwrap();
            symlink("some_file", "some_symlink").unwrap();
            fs::create_dir("some_dir").unwrap();
            fs::create_dir("a").unwrap();
            fs::create_dir("a/b").unwrap();
            fs::File::create("a/b/c").unwrap();

            // Ensure that the path exists
            assert_path_exists!("some_path");

            // Ensure that the path exists and is a file
            assert_path_is_file!("some_file");

            // Ensure that the path exists and is a symlink
            assert_path_is_symlink!("some_symlink");

            // Ensure that the path exists and is a directory
            assert_path_is_dir!("some_dir");

            // Ensure that the path is relative
            assert_path_is_relative!("some_path");

            // Ensure that the path is absolute
            #[cfg(target_family = "unix")]
            assert_path_is_absolute!("/etc");

            // Ensure that the first path is prefixed by the second
            assert_path_starts_with!("a/b/c", "a");

            // Ensure that the first path is suffixed by the second
            assert_path_ends_with!("a/b/c", "b/c");
        }

        #[test]
        fn example_file_text() {
            let temp_dir = tempdir().unwrap();
            env::set_current_dir(temp_dir.path()).unwrap();
            let mut file = fs::File::create("hello_world.txt").unwrap();
            file.write_all(b"hello, world").unwrap();

            assert_file_text!(
                "hello_world.txt",
                max_len = 1024,
                on_text = |text| {
                    assert_eq!(text, "hello, world");
                }
            );
        }

        #[test]
        fn example_file_text_raw() {
            let temp_dir = tempdir().unwrap();
            env::set_current_dir(temp_dir.path()).unwrap();
            let mut file = fs::File::create("hello_world.txt").unwrap();
            file.write_all(b"hello, world").unwrap();

            assert_file_text_raw!(
                "hello_world.txt",
                max_len = 1024,
                on_text = |text| {
                    assert_eq!(text, b"hello, world");
                    //               ↑
                }
            );
        }
    }

    mod panic {
        #[test]
        fn example() {
            // Ensure that the code panics
            assert_panics!(|| {
                panic!();
            });

            // Ensure that the code panics with a specific message
            assert_panics!(
                || {
                    panic!("hello, world");
                },
                on_message = |message| {
                    assert_eq!(message, "hello, world");
                }
            );
        }

        #[test]
        #[should_panic]
        fn example_should_panic() {
            panic!();

            // Code here will not run
        }

        #[test]
        fn example_panic_assertion() {
            assert_panics!(|| {
                panic!();
            });

            // This code will still run
        }
    }

    mod output {
        #[test]
        fn example() {
            assert_outputs!(
                || {
                    println!("hello, world");
                },
                on_stdout = |stdout| {
                    assert_eq!(stdout, "hello, world\n");
                }
            );
        }

        #[test]
        fn example_raw() {
            assert_outputs_raw!(
                || {
                    println!("hello, world");
                },
                on_stdout = |stdout| {
                    assert_eq!(stdout, b"hello, world\n");
                    //                 ↑
                }
            );
        }
    }

    mod custom {
        #[test]
        fn example() {
            // Instead of this:
            let x = 3 + 5;
            let y = 8;
            assert_eq!(x, y);

            // We can write this:
            assert_custom!("lhs == rhs", x == y, |panic_message_builder| {
                panic_message_builder
                    .with_argument("lhs", "x", &x)
                    .with_argument("rhs", "y", &y)
            })
        }
    }

    mod configuring_assertions {
        use num_traits::Float;

        #[test]
        fn example_negation() {
            assert_str_contains!("hello, world", "asdf", negate = true);
        }

        #[test]
        fn example_description() {
            let x = 1.0;
            let y = 1.05;

            assert_le!(
                (x - y).abs(),
                0.1,
                description = "x should be within 0.1 of y"
            );
        }

        #[test]
        fn example_description_owned() {
            const THRESHOLD: f32 = 0.1;

            let x = 1.0;
            let y = 1.05;

            assert_le!(
                (x - y).abs(),
                THRESHOLD,
                description_owned = format!("x should be within {} of y", THRESHOLD)
            );
        }
    }
}

mod tests {
    mod parameterized_tests {
        #[test_with_parameter_values(
            x = [5, 6, 7],
            y = [1, 2])
        ]
        fn example(x: i32, y: i32) {
            assert!(x + y > 0);
        }

        #[test_with_parameter_values(
            x = [5, 6, 7],
            y = [1, 2])
        ]
        #[doc(hidden)]
        fn example_attributes(x: i32, y: i32) {
            assert!(x + y > 0);
        }
    }
}
