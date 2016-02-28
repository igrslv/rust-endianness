//! This crate provides functions to read numbers from a stream of bytes
//! either in big-endian or little-endian. Functions return Result type
//! instead of panic!.
//!
//! # Examples
//!
//! Read signed 16-bit integers:
//! ```rust
//! use endianness::*;
//!
//! let v = vec![0, 128, 128, 0];
//! assert_eq!(-32768, read_i16(&v[0..2], ByteOrder::LittleEndian).unwrap());
//! assert_eq!(-32768, read_i16(&v[2..4], ByteOrder::BigEndian).unwrap());
//! ```
//!
//! Read a signed 32-bit integer:
//! ```rust
//! use endianness::*;
//!
//! let v = vec![0, 128, 128, 0];
//! match read_i32(&v, ByteOrder::LittleEndian) {
//! Ok(n) => println!("Read value {}", n), // 8421376
//! Err(err) => println!("Error: {}", err),
//! }
//! ```
//!
//! Read a single-precision floating point number:
//! ```rust
//! use endianness::*;
//!
//! let v = vec![194, 255, 0, 0];
//! assert_eq!(-127.5, read_f32(&v[0..4], ByteOrder::BigEndian).unwrap());
//! ```
//!

#![crate_name = "endianness"]

#![deny(missing_docs, missing_debug_implementations,
        missing_copy_implementations, trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications)]

use std::mem;
use std::fmt;
use std::error;
use std::result;

/// The 'ByteOrder' type. It represents the order of bytes in a stream we read from.
#[derive(Debug, Copy, Clone)]
pub enum ByteOrder {
    /// Intel byte order
    LittleEndian,
    /// Motorola byte order
    BigEndian,
}

/// The error type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    /// The stream is too small to read the requested type.
    ShortSlice,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ShortSlice => write!(f, "The slice length is too short."),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ShortSlice => "The slice length is too short.",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ShortSlice => None,
        }
    }
}

/// Result type alias that fixes Error parameter.
pub type Result<T> = result::Result<T, Error>;

/// Reads unsigned 16-bit integer from a stream of bytes.
pub fn read_u16(data: &[u8], endianness: ByteOrder) -> Result<u16> {
    if data.len() < 2 {
        Err(Error::ShortSlice)
    } else {
        match endianness {
            ByteOrder::BigEndian => Ok(((data[0] as u16) << 8) + (data[1] as u16)),
            ByteOrder::LittleEndian => Ok(((data[1] as u16) << 8) + (data[0] as u16)),
        }
    }
}

/// Reads signed 16-bit integer from a stream of bytes.
pub fn read_i16(data: &[u8], endianness: ByteOrder) -> Result<i16> {
    Ok(try!(read_u16(data, endianness)) as i16)
}

/// Reads unsigned 32-bit integer from a stream of bytes.
pub fn read_u32(data: &[u8], endianness: ByteOrder) -> Result<u32> {
    if data.len() < 4 {
        Err(Error::ShortSlice)
    } else {
        match endianness {
            ByteOrder::BigEndian => {
                Ok(((data[0] as u32) << 24) + ((data[1] as u32) << 16) + ((data[2] as u32) << 8) +
                   (data[3] as u32))
            }
            ByteOrder::LittleEndian => {
                Ok(((data[3] as u32) << 24) + ((data[2] as u32) << 16) + ((data[1] as u32) << 8) +
                   (data[0] as u32))
            }
        }
    }
}

/// Reads signed 32-bit integer from a stream of bytes.
pub fn read_i32(data: &[u8], endianness: ByteOrder) -> Result<i32> {
    Ok(try!(read_u32(data, endianness)) as i32)
}

/// Reads unsigned 64-bit integer from a stream of bytes.
pub fn read_u64(data: &[u8], endianness: ByteOrder) -> Result<u64> {
    if data.len() < 8 {
        Err(Error::ShortSlice)
    } else {
        match endianness {
            ByteOrder::BigEndian => {
                Ok(((data[0] as u64) << 56) + ((data[1] as u64) << 48) +
                   ((data[2] as u64) << 40) + ((data[3] as u64) << 32) +
                   ((data[4] as u64) << 24) + ((data[5] as u64) << 16) +
                   ((data[6] as u64) << 8) + (data[7] as u64))
            }
            ByteOrder::LittleEndian => {
                Ok(((data[7] as u64) << 56) + ((data[6] as u64) << 48) +
                   ((data[5] as u64) << 40) + ((data[4] as u64) << 32) +
                   ((data[3] as u64) << 24) + ((data[2] as u64) << 16) +
                   ((data[1] as u64) << 8) + (data[0] as u64))
            }
        }
    }
}

/// Reads signed 64-bit integer from a stream of bytes.
pub fn read_i64(data: &[u8], endianness: ByteOrder) -> Result<i64> {
    Ok(try!(read_u64(data, endianness)) as i64)
}

/// Reads a single-precision floating point number.
pub fn read_f32(data: &[u8], endianness: ByteOrder) -> Result<f32> {
    let u = try!(read_u32(data, endianness));
    Ok(unsafe { mem::transmute(u) })
}

/// Reads a double-precision floating point number.
pub fn read_f64(data: &[u8], endianness: ByteOrder) -> Result<f64> {
    let u = try!(read_u64(data, endianness));
    Ok(unsafe { mem::transmute(u) })
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    // Macro to test that all of the functions return an error type
    // when given a slice that is too short for them.
    macro_rules! short_slice {
        ($name:ident, $read:ident) => (
            mod $name {
                use {ByteOrder, Error, $read};

                #[test]
                fn read_big_endian() {
                    assert_eq!(Error::ShortSlice, $read(&[], ByteOrder::BigEndian).unwrap_err());
                }

                #[test]
                fn read_little_endian() {
                    assert_eq!(Error::ShortSlice, $read(&[], ByteOrder::LittleEndian).unwrap_err());
                }
            }
        );
    }

    short_slice!(short_slice_u16, read_u16);
    short_slice!(short_slice_i16, read_i16);
    short_slice!(short_slice_u32, read_u32);
    short_slice!(short_slice_i32, read_i32);
    short_slice!(short_slice_u64, read_u64);
    short_slice!(short_slice_i64, read_i64);
    short_slice!(short_slice_f32, read_f32);
    short_slice!(short_slice_f64, read_f64);

    // A macro to perform generative testing using the following invariant:
    // for any integer N that was transmuted to a stream of bytes read functions must return N.
    macro_rules! read_correctness {
        ($name:ident, $ty:ty, $size: expr, $read:ident, $max:expr) => (
            mod $name {
                use std::mem;
                use {ByteOrder, $read};

                extern crate quickcheck;
                extern crate rand;
                use self::quickcheck::{QuickCheck, StdGen, Testable};

                #[test]
                fn read_big_endian() {
                    #[cfg(target_endian = "little")]
                    fn prop(n: $ty) -> bool {
                        let mut data = unsafe { mem::transmute::<_, [u8; $size]>(n) };
                        data.reverse();
                        n == $read(&data, ByteOrder::BigEndian).unwrap()
                    }

                    #[cfg(target_endian = "big")]
                    fn prop(n: $ty) -> bool {
                        let data = unsafe { mem::transmute::<_, [u8; $size]>(n) };
                        n == $read(&data, ByteOrder::BigEndian).unwrap()
                    }

                    // cast function explicitly to get rid off warns about trivial cast (why?)
                    let f: fn($ty) -> bool = prop;
                    quick_check(f);
                }

                #[test]
                fn read_little_endian() {
                    #[cfg(target_endian = "little")]
                    fn prop(n: $ty) -> bool {
                        let data = unsafe { mem::transmute::<_, [u8; $size]>(n) };
                        n == $read(&data, ByteOrder::LittleEndian).unwrap()
                    }

                    #[cfg(target_endian = "big")]
                    fn prop(n: $ty) -> bool {
                        let mut data = unsafe { mem::transmute::<_, [u8; $size]>(n) };
                        data.reverse();
                        n == $read(&data, ByteOrder::LittleEndian).unwrap()
                    }

                    // cast function explicitly to get rid off warns about trivial cast (why?)
                    let f: fn($ty) -> bool = prop;
                    quick_check(f);
                }

                fn quick_check<T: Testable>(prop: T) {
                    QuickCheck::new()
                        .gen(StdGen::new(rand::thread_rng(), $max as usize))
                        .quickcheck(prop);
                }
            }
        );
    }

    read_correctness!(test_u16, u16, 2, read_u16, ::std::u16::MAX);
    read_correctness!(test_i16, i16, 2, read_i16, ::std::i16::MAX);
    read_correctness!(test_u32, u32, 4, read_u32, ::std::u32::MAX);
    read_correctness!(test_i32, i32, 4, read_i32, ::std::i32::MAX);
    read_correctness!(test_u64, u64, 8, read_u64, ::std::u64::MAX);
    read_correctness!(test_i64, i64, 8, read_i64, ::std::i64::MAX);
    read_correctness!(test_f32, f32, 4, read_f32, ::std::u32::MAX);
    read_correctness!(test_f64, f64, 8, read_f64, ::std::u64::MAX);
}
