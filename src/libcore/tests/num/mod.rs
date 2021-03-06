// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::convert::TryFrom;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::ops::{Add, Sub, Mul, Div, Rem};
use core::option::Option;
use core::option::Option::{Some, None};

#[macro_use]
mod int_macros;

mod i8;
mod i16;
mod i32;
mod i64;

#[macro_use]
mod uint_macros;

mod u8;
mod u16;
mod u32;
mod u64;

mod flt2dec;
mod dec2flt;
mod bignum;


/// Adds the attribute to all items in the block.
macro_rules! cfg_block {
    ($(#[$attr:meta]{$($it:item)*})*) => {$($(
        #[$attr]
        $it
    )*)*}
}

/// Groups items that assume the pointer width is either 16/32/64, and has to be altered if
/// support for larger/smaller pointer widths are added in the future.
macro_rules! assume_usize_width {
    {$($it:item)*} => {#[cfg(not(any(
        target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64")))]
           compile_error!("The current tests of try_from on usize/isize assume that \
                           the pointer width is either 16, 32, or 64");
                    $($it)*
    }
}

/// Helper function for testing numeric operations
pub fn test_num<T>(ten: T, two: T) where
    T: PartialEq
     + Add<Output=T> + Sub<Output=T>
     + Mul<Output=T> + Div<Output=T>
     + Rem<Output=T> + Debug
     + Copy
{
    assert_eq!(ten.add(two),  ten + two);
    assert_eq!(ten.sub(two),  ten - two);
    assert_eq!(ten.mul(two),  ten * two);
    assert_eq!(ten.div(two),  ten / two);
    assert_eq!(ten.rem(two),  ten % two);
}

#[test]
fn from_str_issue7588() {
    let u : Option<u8> = u8::from_str_radix("1000", 10).ok();
    assert_eq!(u, None);
    let s : Option<i16> = i16::from_str_radix("80000", 10).ok();
    assert_eq!(s, None);
}

#[test]
fn test_int_from_str_overflow() {
    let mut i8_val: i8 = 127;
    assert_eq!("127".parse::<i8>().ok(), Some(i8_val));
    assert_eq!("128".parse::<i8>().ok(), None);

    i8_val = i8_val.wrapping_add(1);
    assert_eq!("-128".parse::<i8>().ok(), Some(i8_val));
    assert_eq!("-129".parse::<i8>().ok(), None);

    let mut i16_val: i16 = 32_767;
    assert_eq!("32767".parse::<i16>().ok(), Some(i16_val));
    assert_eq!("32768".parse::<i16>().ok(), None);

    i16_val = i16_val.wrapping_add(1);
    assert_eq!("-32768".parse::<i16>().ok(), Some(i16_val));
    assert_eq!("-32769".parse::<i16>().ok(), None);

    let mut i32_val: i32 = 2_147_483_647;
    assert_eq!("2147483647".parse::<i32>().ok(), Some(i32_val));
    assert_eq!("2147483648".parse::<i32>().ok(), None);

    i32_val = i32_val.wrapping_add(1);
    assert_eq!("-2147483648".parse::<i32>().ok(), Some(i32_val));
    assert_eq!("-2147483649".parse::<i32>().ok(), None);

    let mut i64_val: i64 = 9_223_372_036_854_775_807;
    assert_eq!("9223372036854775807".parse::<i64>().ok(), Some(i64_val));
    assert_eq!("9223372036854775808".parse::<i64>().ok(), None);

    i64_val = i64_val.wrapping_add(1);
    assert_eq!("-9223372036854775808".parse::<i64>().ok(), Some(i64_val));
    assert_eq!("-9223372036854775809".parse::<i64>().ok(), None);
}

#[test]
fn test_leading_plus() {
    assert_eq!("+127".parse::<u8>().ok(), Some(127));
    assert_eq!("+9223372036854775807".parse::<i64>().ok(), Some(9223372036854775807));
}

#[test]
fn test_invalid() {
    assert_eq!("--129".parse::<i8>().ok(), None);
    assert_eq!("++129".parse::<i8>().ok(), None);
    assert_eq!("Съешь".parse::<u8>().ok(), None);
}

#[test]
fn test_empty() {
    assert_eq!("-".parse::<i8>().ok(), None);
    assert_eq!("+".parse::<i8>().ok(), None);
    assert_eq!("".parse::<u8>().ok(), None);
}

macro_rules! test_impl_from {
    ($fn_name: ident, $Small: ty, $Large: ty) => {
        #[test]
        fn $fn_name() {
            let small_max = <$Small>::max_value();
            let small_min = <$Small>::min_value();
            let large_max: $Large = small_max.into();
            let large_min: $Large = small_min.into();
            assert_eq!(large_max as $Small, small_max);
            assert_eq!(large_min as $Small, small_min);
        }
    }
}

// Unsigned -> Unsigned
test_impl_from! { test_u8u16, u8, u16 }
test_impl_from! { test_u8u32, u8, u32 }
test_impl_from! { test_u8u64, u8, u64 }
test_impl_from! { test_u8usize, u8, usize }
test_impl_from! { test_u16u32, u16, u32 }
test_impl_from! { test_u16u64, u16, u64 }
test_impl_from! { test_u32u64, u32, u64 }

// Signed -> Signed
test_impl_from! { test_i8i16, i8, i16 }
test_impl_from! { test_i8i32, i8, i32 }
test_impl_from! { test_i8i64, i8, i64 }
test_impl_from! { test_i8isize, i8, isize }
test_impl_from! { test_i16i32, i16, i32 }
test_impl_from! { test_i16i64, i16, i64 }
test_impl_from! { test_i32i64, i32, i64 }

// Unsigned -> Signed
test_impl_from! { test_u8i16, u8, i16 }
test_impl_from! { test_u8i32, u8, i32 }
test_impl_from! { test_u8i64, u8, i64 }
test_impl_from! { test_u16i32, u16, i32 }
test_impl_from! { test_u16i64, u16, i64 }
test_impl_from! { test_u32i64, u32, i64 }

// Signed -> Float
test_impl_from! { test_i8f32, i8, f32 }
test_impl_from! { test_i8f64, i8, f64 }
test_impl_from! { test_i16f32, i16, f32 }
test_impl_from! { test_i16f64, i16, f64 }
test_impl_from! { test_i32f64, i32, f64 }

// Unsigned -> Float
test_impl_from! { test_u8f32, u8, f32 }
test_impl_from! { test_u8f64, u8, f64 }
test_impl_from! { test_u16f32, u16, f32 }
test_impl_from! { test_u16f64, u16, f64 }
test_impl_from! { test_u32f64, u32, f64 }

// Float -> Float
#[cfg_attr(all(target_arch = "wasm32", target_os = "emscripten"), ignore)] // issue 42630
#[test]
fn test_f32f64() {
    use core::f32;

    let max: f64 = f32::MAX.into();
    assert_eq!(max as f32, f32::MAX);
    assert!(max.is_normal());

    let min: f64 = f32::MIN.into();
    assert_eq!(min as f32, f32::MIN);
    assert!(min.is_normal());

    let min_positive: f64 = f32::MIN_POSITIVE.into();
    assert_eq!(min_positive as f32, f32::MIN_POSITIVE);
    assert!(min_positive.is_normal());

    let epsilon: f64 = f32::EPSILON.into();
    assert_eq!(epsilon as f32, f32::EPSILON);
    assert!(epsilon.is_normal());

    let zero: f64 = (0.0f32).into();
    assert_eq!(zero as f32, 0.0f32);
    assert!(zero.is_sign_positive());

    let neg_zero: f64 = (-0.0f32).into();
    assert_eq!(neg_zero as f32, -0.0f32);
    assert!(neg_zero.is_sign_negative());

    let infinity: f64 = f32::INFINITY.into();
    assert_eq!(infinity as f32, f32::INFINITY);
    assert!(infinity.is_infinite());
    assert!(infinity.is_sign_positive());

    let neg_infinity: f64 = f32::NEG_INFINITY.into();
    assert_eq!(neg_infinity as f32, f32::NEG_INFINITY);
    assert!(neg_infinity.is_infinite());
    assert!(neg_infinity.is_sign_negative());

    let nan: f64 = f32::NAN.into();
    assert!(nan.is_nan());
}


/// Conversions where the full width of $source can be represented as $target
macro_rules! test_impl_try_from_always_ok {
    ($fn_name:ident, $source:ty, $target: ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::max_value();
            let min = <$source>::min_value();
            let zero: $source = 0;
            assert_eq!(<$target as TryFrom<$source>>::try_from(max).unwrap(),
                       max as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(min).unwrap(),
                       min as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(),
                       zero as $target);
        }
    }
}

test_impl_try_from_always_ok! { test_try_u8u8, u8, u8 }
test_impl_try_from_always_ok! { test_try_u8u16, u8, u16 }
test_impl_try_from_always_ok! { test_try_u8u32, u8, u32 }
test_impl_try_from_always_ok! { test_try_u8u64, u8, u64 }
test_impl_try_from_always_ok! { test_try_u8u128, u8, u128 }
test_impl_try_from_always_ok! { test_try_u8i16, u8, i16 }
test_impl_try_from_always_ok! { test_try_u8i32, u8, i32 }
test_impl_try_from_always_ok! { test_try_u8i64, u8, i64 }
test_impl_try_from_always_ok! { test_try_u8i128, u8, i128 }

test_impl_try_from_always_ok! { test_try_u16u16, u16, u16 }
test_impl_try_from_always_ok! { test_try_u16u32, u16, u32 }
test_impl_try_from_always_ok! { test_try_u16u64, u16, u64 }
test_impl_try_from_always_ok! { test_try_u16u128, u16, u128 }
test_impl_try_from_always_ok! { test_try_u16i32, u16, i32 }
test_impl_try_from_always_ok! { test_try_u16i64, u16, i64 }
test_impl_try_from_always_ok! { test_try_u16i128, u16, i128 }

test_impl_try_from_always_ok! { test_try_u32u32, u32, u32 }
test_impl_try_from_always_ok! { test_try_u32u64, u32, u64 }
test_impl_try_from_always_ok! { test_try_u32u128, u32, u128 }
test_impl_try_from_always_ok! { test_try_u32i64, u32, i64 }
test_impl_try_from_always_ok! { test_try_u32i128, u32, i128 }

test_impl_try_from_always_ok! { test_try_u64u64, u64, u64 }
test_impl_try_from_always_ok! { test_try_u64u128, u64, u128 }
test_impl_try_from_always_ok! { test_try_u64i128, u64, i128 }

test_impl_try_from_always_ok! { test_try_u128u128, u128, u128 }

test_impl_try_from_always_ok! { test_try_i8i8, i8, i8 }
test_impl_try_from_always_ok! { test_try_i8i16, i8, i16 }
test_impl_try_from_always_ok! { test_try_i8i32, i8, i32 }
test_impl_try_from_always_ok! { test_try_i8i64, i8, i64 }
test_impl_try_from_always_ok! { test_try_i8i128, i8, i128 }

test_impl_try_from_always_ok! { test_try_i16i16, i16, i16 }
test_impl_try_from_always_ok! { test_try_i16i32, i16, i32 }
test_impl_try_from_always_ok! { test_try_i16i64, i16, i64 }
test_impl_try_from_always_ok! { test_try_i16i128, i16, i128 }

test_impl_try_from_always_ok! { test_try_i32i32, i32, i32 }
test_impl_try_from_always_ok! { test_try_i32i64, i32, i64 }
test_impl_try_from_always_ok! { test_try_i32i128, i32, i128 }

test_impl_try_from_always_ok! { test_try_i64i64, i64, i64 }
test_impl_try_from_always_ok! { test_try_i64i128, i64, i128 }

test_impl_try_from_always_ok! { test_try_i128i128, i128, i128 }

test_impl_try_from_always_ok! { test_try_usizeusize, usize, usize }
test_impl_try_from_always_ok! { test_try_isizeisize, isize, isize }

assume_usize_width! {
    test_impl_try_from_always_ok! { test_try_u8usize, u8, usize }
    test_impl_try_from_always_ok! { test_try_u8isize, u8, isize }
    test_impl_try_from_always_ok! { test_try_i8isize, i8, isize }

    test_impl_try_from_always_ok! { test_try_u16usize, u16, usize }
    test_impl_try_from_always_ok! { test_try_i16isize, i16, isize }

    test_impl_try_from_always_ok! { test_try_usizeu64, usize, u64 }
    test_impl_try_from_always_ok! { test_try_usizeu128, usize, u128 }
    test_impl_try_from_always_ok! { test_try_usizei128, usize, i128 }

    test_impl_try_from_always_ok! { test_try_isizei64, isize, i64 }
    test_impl_try_from_always_ok! { test_try_isizei128, isize, i128 }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_always_ok! { test_try_usizeu16, usize, u16 }
            test_impl_try_from_always_ok! { test_try_isizei16, isize, i16 }
            test_impl_try_from_always_ok! { test_try_usizeu32, usize, u32 }
            test_impl_try_from_always_ok! { test_try_usizei32, usize, i32 }
            test_impl_try_from_always_ok! { test_try_isizei32, isize, i32 }
            test_impl_try_from_always_ok! { test_try_usizei64, usize, i64 }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_always_ok! { test_try_u16isize, u16, isize }
            test_impl_try_from_always_ok! { test_try_usizeu32, usize, u32 }
            test_impl_try_from_always_ok! { test_try_isizei32, isize, i32 }
            test_impl_try_from_always_ok! { test_try_u32usize, u32, usize }
            test_impl_try_from_always_ok! { test_try_i32isize, i32, isize }
            test_impl_try_from_always_ok! { test_try_usizei64, usize, i64 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_always_ok! { test_try_u16isize, u16, isize }
            test_impl_try_from_always_ok! { test_try_u32usize, u32, usize }
            test_impl_try_from_always_ok! { test_try_u32isize, u32, isize }
            test_impl_try_from_always_ok! { test_try_i32isize, i32, isize }
            test_impl_try_from_always_ok! { test_try_u64usize, u64, usize }
            test_impl_try_from_always_ok! { test_try_i64isize, i64, isize }
        }
    );
}

/// Conversions where max of $source can be represented as $target,
macro_rules! test_impl_try_from_signed_to_unsigned_upper_ok {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::max_value();
            let min = <$source>::min_value();
            let zero: $source = 0;
            let neg_one: $source = -1;
            assert_eq!(<$target as TryFrom<$source>>::try_from(max).unwrap(),
                       max as $target);
            assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(),
                       zero as $target);
            assert!(<$target as TryFrom<$source>>::try_from(neg_one).is_err());
        }
    }
}

test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u8, i8, u8 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u16, i8, u16 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u32, i8, u32 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u64, i8, u64 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u128, i8, u128 }

test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u16, i16, u16 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u32, i16, u32 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u64, i16, u64 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u128, i16, u128 }

test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u32, i32, u32 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u64, i32, u64 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u128, i32, u128 }

test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64u64, i64, u64 }
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64u128, i64, u128 }

test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i128u128, i128, u128 }

assume_usize_width! {
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8usize, i8, usize }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16usize, i16, usize }

    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu64, isize, u64 }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu128, isize, u128 }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeusize, isize, usize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu16, isize, u16 }
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu32, isize, u32 }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu32, isize, u32 }

            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32usize, i32, usize }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32usize, i32, usize }
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64usize, i64, usize }
        }
    );
}

/// Conversions where max of $source can not be represented as $target,
/// but min can.
macro_rules! test_impl_try_from_unsigned_to_signed_upper_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::max_value();
            let min = <$source>::min_value();
            let zero: $source = 0;
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(min).unwrap(),
                       min as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(),
                       zero as $target);
        }
    }
}

test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u8i8, u8, i8 }

test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16i8, u16, i8 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16i16, u16, i16 }

test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i8, u32, i8 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i16, u32, i16 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i32, u32, i32 }

test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i8, u64, i8 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i16, u64, i16 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i32, u64, i32 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i64, u64, i64 }

test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i8, u128, i8 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i16, u128, i16 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i32, u128, i32 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i64, u128, i64 }
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i128, u128, i128 }

assume_usize_width! {
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64isize, u64, isize }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128isize, u128, isize }

    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei8, usize, i8 }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei16, usize, i16 }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizeisize, usize, isize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16isize, u16, isize }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32isize, u32, isize }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32isize, u32, isize }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei32, usize, i32 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei32, usize, i32 }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei64, usize, i64 }
        }
    );
}

/// Conversions where min/max of $source can not be represented as $target.
macro_rules! test_impl_try_from_same_sign_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::max_value();
            let min = <$source>::min_value();
            let zero: $source = 0;
            let t_max = <$target>::max_value();
            let t_min = <$target>::min_value();
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            if min != 0 {
                assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            }
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(),
                       zero as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(t_max as $source)
                            .unwrap(),
                       t_max as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(t_min as $source)
                            .unwrap(),
                       t_min as $target);
        }
    }
}

test_impl_try_from_same_sign_err! { test_try_u16u8, u16, u8 }

test_impl_try_from_same_sign_err! { test_try_u32u8, u32, u8 }
test_impl_try_from_same_sign_err! { test_try_u32u16, u32, u16 }

test_impl_try_from_same_sign_err! { test_try_u64u8, u64, u8 }
test_impl_try_from_same_sign_err! { test_try_u64u16, u64, u16 }
test_impl_try_from_same_sign_err! { test_try_u64u32, u64, u32 }

test_impl_try_from_same_sign_err! { test_try_u128u8, u128, u8 }
test_impl_try_from_same_sign_err! { test_try_u128u16, u128, u16 }
test_impl_try_from_same_sign_err! { test_try_u128u32, u128, u32 }
test_impl_try_from_same_sign_err! { test_try_u128u64, u128, u64 }

test_impl_try_from_same_sign_err! { test_try_i16i8, i16, i8 }
test_impl_try_from_same_sign_err! { test_try_isizei8, isize, i8 }

test_impl_try_from_same_sign_err! { test_try_i32i8, i32, i8 }
test_impl_try_from_same_sign_err! { test_try_i32i16, i32, i16 }

test_impl_try_from_same_sign_err! { test_try_i64i8, i64, i8 }
test_impl_try_from_same_sign_err! { test_try_i64i16, i64, i16 }
test_impl_try_from_same_sign_err! { test_try_i64i32, i64, i32 }

test_impl_try_from_same_sign_err! { test_try_i128i8, i128, i8 }
test_impl_try_from_same_sign_err! { test_try_i128i16, i128, i16 }
test_impl_try_from_same_sign_err! { test_try_i128i32, i128, i32 }
test_impl_try_from_same_sign_err! { test_try_i128i64, i128, i64 }

assume_usize_width! {
    test_impl_try_from_same_sign_err! { test_try_usizeu8, usize, u8 }
    test_impl_try_from_same_sign_err! { test_try_u128usize, u128, usize }
    test_impl_try_from_same_sign_err! { test_try_i128isize, i128, isize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_same_sign_err! { test_try_u32usize, u32, usize }
            test_impl_try_from_same_sign_err! { test_try_u64usize, u64, usize }

            test_impl_try_from_same_sign_err! { test_try_i32isize, i32, isize }
            test_impl_try_from_same_sign_err! { test_try_i64isize, i64, isize }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_same_sign_err! { test_try_u64usize, u64, usize }
            test_impl_try_from_same_sign_err! { test_try_usizeu16, usize, u16 }

            test_impl_try_from_same_sign_err! { test_try_i64isize, i64, isize }
            test_impl_try_from_same_sign_err! { test_try_isizei16, isize, i16 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_same_sign_err! { test_try_usizeu16, usize, u16 }
            test_impl_try_from_same_sign_err! { test_try_usizeu32, usize, u32 }

            test_impl_try_from_same_sign_err! { test_try_isizei16, isize, i16 }
            test_impl_try_from_same_sign_err! { test_try_isizei32, isize, i32 }
        }
    );
}

/// Conversinos where neither the min nor the max of $source can be represented by
/// $target, but max/min of the target can be represented by the source.
macro_rules! test_impl_try_from_signed_to_unsigned_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::max_value();
            let min = <$source>::min_value();
            let zero: $source = 0;
            let t_max = <$target>::max_value();
            let t_min = <$target>::min_value();
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(),
                       zero as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(t_max as $source)
                            .unwrap(),
                       t_max as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(t_min as $source)
                            .unwrap(),
                       t_min as $target);
        }
    }
}

test_impl_try_from_signed_to_unsigned_err! { test_try_i16u8, i16, u8 }

test_impl_try_from_signed_to_unsigned_err! { test_try_i32u8, i32, u8 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i32u16, i32, u16 }

test_impl_try_from_signed_to_unsigned_err! { test_try_i64u8, i64, u8 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i64u16, i64, u16 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i64u32, i64, u32 }

test_impl_try_from_signed_to_unsigned_err! { test_try_i128u8, i128, u8 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u16, i128, u16 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u32, i128, u32 }
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u64, i128, u64 }

assume_usize_width! {
    test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu8, isize, u8 }
    test_impl_try_from_signed_to_unsigned_err! { test_try_i128usize, i128, usize }

    cfg_block! {
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_i32usize, i32, usize }
            test_impl_try_from_signed_to_unsigned_err! { test_try_i64usize, i64, usize }
        }
        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_i64usize, i64, usize }

            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu16, isize, u16 }
        }
        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu16, isize, u16 }
            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu32, isize, u32 }
        }
    }
}

macro_rules! test_float {
    ($modname: ident, $fty: ty, $inf: expr, $neginf: expr, $nan: expr) => { mod $modname {
        use core::num::Float;
        // FIXME(nagisa): these tests should test for sign of -0.0
        #[test]
        fn min() {
            assert_eq!(0.0.min(0.0), 0.0);
            assert_eq!((-0.0).min(-0.0), -0.0);
            assert_eq!(9.0.min(9.0), 9.0);
            assert_eq!((-9.0).min(0.0), -9.0);
            assert_eq!(0.0.min(9.0), 0.0);
            assert_eq!((-0.0).min(-9.0), -9.0);
            assert_eq!($inf.min(9.0), 9.0);
            assert_eq!(9.0.min($inf), 9.0);
            assert_eq!($inf.min(-9.0), -9.0);
            assert_eq!((-9.0).min($inf), -9.0);
            assert_eq!($neginf.min(9.0), $neginf);
            assert_eq!(9.0.min($neginf), $neginf);
            assert_eq!($neginf.min(-9.0), $neginf);
            assert_eq!((-9.0).min($neginf), $neginf);
            assert_eq!($nan.min(9.0), 9.0);
            assert_eq!($nan.min(-9.0), -9.0);
            assert_eq!(9.0.min($nan), 9.0);
            assert_eq!((-9.0).min($nan), -9.0);
            assert!($nan.min($nan).is_nan());
        }
        #[test]
        fn max() {
            assert_eq!(0.0.max(0.0), 0.0);
            assert_eq!((-0.0).max(-0.0), -0.0);
            assert_eq!(9.0.max(9.0), 9.0);
            assert_eq!((-9.0).max(0.0), 0.0);
            assert_eq!(0.0.max(9.0), 9.0);
            assert_eq!((-0.0).max(-9.0), -0.0);
            assert_eq!($inf.max(9.0), $inf);
            assert_eq!(9.0.max($inf), $inf);
            assert_eq!($inf.max(-9.0), $inf);
            assert_eq!((-9.0).max($inf), $inf);
            assert_eq!($neginf.max(9.0), 9.0);
            assert_eq!(9.0.max($neginf), 9.0);
            assert_eq!($neginf.max(-9.0), -9.0);
            assert_eq!((-9.0).max($neginf), -9.0);
            assert_eq!($nan.max(9.0), 9.0);
            assert_eq!($nan.max(-9.0), -9.0);
            assert_eq!(9.0.max($nan), 9.0);
            assert_eq!((-9.0).max($nan), -9.0);
            assert!($nan.max($nan).is_nan());
        }
    } }
}

test_float!(f32, f32, ::core::f32::INFINITY, ::core::f32::NEG_INFINITY, ::core::f32::NAN);
test_float!(f64, f64, ::core::f64::INFINITY, ::core::f64::NEG_INFINITY, ::core::f64::NAN);
