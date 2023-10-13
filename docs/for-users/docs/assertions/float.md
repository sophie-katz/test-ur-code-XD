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

# Float assertions

Testing with floats is deceptively complicated. [Arithmetic assertions](assertions/arithmetic.md) can work with floats just fine, but they do not account for floating-point error in calculations. For example, this assertion will *fail*:

```rust
assert_eq!(
    0.15 + 0.15 + 0.15,
    0.1 + 0.1 + 0.25
);
```

To explain why, we need to understand a bit of background about floating-point comparison. This [excellent article](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/) by Bruce Dawson goes into much more detail.

!!! warning

    There's a lot of explanation before this document gets to the actual assertions. *This is intentional.*
    
    test ur code XD handles floating-point testing in a much more detailed manner than many other testing frameworks. It requires a bit of background to understand why.

The example above taken from the excellent [`float_cmp` crate](https://docs.rs/float-cmp/latest/float_cmp/#the-problem). test ur code XD builds a bit more functionality on top of it.

## Floating-point comparison is complicated

Floating-point numbers take up 32 or 64 bits on most architectures. This provides a lot of precision, but not infinite precision. For example, 0.1 cannot be represented exactly using 32 bits. Instead, it is represented as [0.10000000149](https://www.h-schmidt.net/FloatConverter/IEEE754.html). The next smallest floating-point number possible in 32-bits is 0.0999999940395.

**ULP** is short for Unit in Last Place. It is the distance between a given floating-point value and the next one possible. See [this article](https://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/) for more detail. For example, there is 1 ULP between 0.0999999940395 and 0.10000000149 because they are neighbors.

### Epsilons

Systems provide floating point **epsilons** to help with floating-point comparisons. In rust they are defined as `f32::EPSILON` and `f64::EPSILON`. They are the difference between `1.0` and the next highest floating-point value.

## Most floating-point testing is bad

Two very common methods of testing with floating-point values have major issues.

### Epsilons alone are not enough

A lot of floating-point testing is done using floating-point epsilons. It looks like this:

```
abs(x - y) <= epsilon
```

But there's a problem. When `x` and `y` are between 1.0 and 2.0, an `f32::EPSILON` comparison means that the two values are within 1 ULP. However 1 ULP at 1,000,000 is 0.0625. This epsilon is way too small to be useful at that range.

### ULPs alone are not enough

ULPs can also be used directly for comparison, which is what the [Google Test Framework](https://github.com/google/googletest) does in C++. It says that if two floating-point values are within 4 ULPs, then they are close enough to equal. The problem is that in every floating-point operation there is a possible error. This error can accumulate over multiple operations, and easily get over 4 ULPs.

Moreover, near zero one ULP is an incredibly small value. The next highest floating-point value after 0.0 is 1.40129846432e-45. This is way smaller than `f32::EPSILON` and implies much more precision than most floating-point calculations need. Near zero, 1 ULP is much too precise.

## Testing that takes this into account

There are two potential ways to compare floating-point numbers while taking this into account. 

### Relative epsilons

`f32::EPSILON` and `f64::EPSILON` are 1 ULP between 1.0 and 2.0, but they can be extended to be applicable to numbers outside that range. Multiply them by the larger magnitude of the two numbers to get a relative epsilon. This relative epsilon will have approximately the same precision at different magnitudes.

This still breaks down when comparing numbers near zero, though. Near zero, an absolute epsilon can be used. This is a fixed value which represents the real precision needed for the calculation. If your calculation needs to be accurate to within 1e-6, then use 1e-6 as the absolute epsilon.

### ULPs with epsilon near zero

ULPs by definition have precision that is relative to the magnitude of the numbers being compared. This is good, but it still breaks down near zero. The same absolute epsilon can be used near zero as for relative epsilon comparisons.

### When to use relative epsilon vs ULPs

Unfortunately, there is no right answer. Relative epsilons tend to make a bit more sense when you care more about real numbers, while ULPs tend to make more sense mean dealing with the details of floating-point operations.

### Special floating-point values

The special floating point values we care about are:

* Positive infinity
* Negative infinity
* NaN
* Negative zero

These values can be compared using separate logic to the rest of the numbers:

* Positive infinity is only equal to positive infinity
* Negative infinity is only equal to negative infinity
* NaN is only equal to NaN
* Negative zero is equivalent to positive zero for the purposes of comparison

## Assertions

test ur code XD provides assertions that follow the above guidelines. Let us take the assertion `assert_f32_eq!(...)` as an initial example for relative epsilon comparison:

```rust
assert_f32_eq!(
    x,
    y,
    relative_epsilon = f32::EPSILON,
    epsilon_near_zero = 1e-6
);
```

The assertion has these keyword arguments:

* `relative_epsilon = f32::EPSILON` means that the relative epsilon used for comparison is `f32::EPSILON`.
* `epsilon_near_zero = 1e-6` is the fixed epsilon that is used when `x` and `y` are near zero. This means that the comparison is guaranteed to within `1e-6` precision. You can set it to 0.0 to disable near zero comparison handling.

!!! info

    These arguments apply to every floating-point assertion in test ur code XD.

There is no default value for either argument because these values will have to change for different comparisons depending on the calculation and the needs of the testing.

* `relative_epsilon` will commonly be something like `4 * f32::EPSILON` to account for more error in the calculation.
* `epsilon_near_zero` is the absolute guaranteed precision and will need to change depending on the needs of the testing.

### With ULPs

You can also write this assertion in terms of ULPs:

```rust
assert_f32_eq!(x, y, ulps = 1, epsilon_near_zero = 1e-6);
```

The argument `ulps = 1` means that the calculation is guaranteed to be correct within 1 ULP.

### Different types of comparisons

test ur code XD has these assertion macros for doing different comparisons with floats:

```rust
// Ensure that x and y are equal accounting for floating-point error
assert_f32_eq!(x, y, ...);

// Ensure that x and y are unequal accounting for floating-point error
assert_f32_ne!(x, y, ...);

// Ensure that x is less than or equal to y accounting for
// floating-point error
assert_f32_le!(x, y, ...);

// Ensure that x is greater than or equal to y accounting for
// floating-point error
assert_f32_ge!(x, y, ...);
```

There are also equivalent macros for dealing with `f64` values:

```rust
assert_f64_eq!(x, y, ...);
assert_f64_ne!(x, y, ...);
assert_f64_le!(x, y, ...);
assert_f64_ge!(x, y, ...);
```

They all use the same arguments.
