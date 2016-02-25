use std::fmt;
use std::error;
use std::result;

#[derive(Debug)]
pub enum ByteOrder {
    // Intel byte order
    LittleEndian,
    // Motorola byte order
    BigEndian,
}

#[derive(Debug,PartialEq)]
pub enum Error {
    ShortSlice,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ShortSlice => write!(f, "The slice length is too short."),
            Error::Other(ref err_string) => write!(f, "{}", err_string),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ShortSlice => "The slice length is too short.",
            Error::Other(ref err_string) => err_string,
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ShortSlice => None,
            Error::Other(_) => None,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn read_u16(data: &[u8], endianness: ByteOrder) -> Result<u16> {
    if data.len() < 2 {
        Err(Error::ShortSlice)
    } else {
        match endianness {
            ByteOrder::BigEndian => {
                Ok( ((data[0] as u16) << 8) + (data[1] as u16) )
            }
            ByteOrder::LittleEndian => {
                Ok( ((data[1] as u16) << 8) + (data[0] as u16) )
            }
        }
    }
}

pub fn read_i16(data: &[u8], endianness: ByteOrder) -> Result<i16> {
    Ok( try!(read_u16(data, endianness)) as i16 )
}

pub fn read_u32(data: &[u8], endianness: ByteOrder) -> Result<u32> {
    if data.len() < 4 {
        Err(Error::ShortSlice)
    } else {
        match endianness {
            ByteOrder::BigEndian => {
                Ok(
                    ( (data[0] as u32) << 24 ) + ( (data[1] as u32) << 16 ) +
                    ( (data[2] as u32) << 8 ) + (data[3] as u32)
                )
            }
            ByteOrder::LittleEndian => {
                Ok(
                    ( (data[3] as u32) << 24 ) + ( (data[2] as u32) << 16 ) +
                    ( (data[1] as u32) << 8 ) + (data[0] as u32)
                )
            }
        }
    }
}

pub fn read_i32(data: &[u8], endianness: ByteOrder) -> Result<i32> {
    Ok( try!(read_u32(data, endianness)) as i32 )
}

#[cfg(test)]
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
        )
    }

    short_slice!(short_u16, read_u16);
    short_slice!(short_i16, read_i16);
    short_slice!(short_u32, read_u32);
    short_slice!(short_i32, read_u32);

    // A macro to perform generative testing using the following invariant:
    // for any integer N that was transmuted to a stream of bytes read functions must return N.
    macro_rules! read_correctness {
        ($name:ident, $ty_int:ty, $bytes: expr, $read:ident, $max:expr) => {
            mod $name {
                use std::mem;
                use {ByteOrder, $read};

                extern crate quickcheck;
                extern crate rand;
                use self::quickcheck::{QuickCheck, StdGen, Testable};

                #[test]
                fn read_big_endian() {
                    #[cfg(target_endian = "little")]
                    fn prop(n: $ty_int) -> bool {
                        let mut data = unsafe { mem::transmute::<_, [u8; $bytes]>(n as $ty_int) };
                        data.reverse();
                        n == $read(&data, ByteOrder::BigEndian).unwrap()
                    }

                    #[cfg(target_endian = "big")]
                    fn prop(n: $ty_int) -> bool {
                        let data = unsafe { mem::transmute::<_, [u8; $bytes]>(n as $ty_int) };
                        n == $read(&data, ByteOrder::BigEndian).unwrap()
                    }

                    self::quick_check(prop as fn($ty_int) -> bool);
                }

                #[test]
                fn read_little_endian() {
                    #[cfg(target_endian = "little")]
                    fn prop(n: $ty_int) -> bool {
                        let data = unsafe { mem::transmute::<_, [u8; $bytes]>(n as $ty_int) };
                        n == $read(&data, ByteOrder::LittleEndian).unwrap()
                    }

                    #[cfg(target_endian = "big")]
                    fn prop(n: $ty_int) -> bool {
                        let mut data = unsafe { mem::transmute::<_, [u8; $bytes]>(n as $ty_int) };
                        data.reverse();
                        n == $read(&data, ByteOrder::LittleEndian).unwrap()
                    }

                    self::quick_check(prop as fn($ty_int) -> bool);
                }

                fn quick_check<T: Testable>(prop: T) {
                    QuickCheck::new()
                        .gen(StdGen::new(rand::thread_rng(), $max as usize))
                        .quickcheck(prop);
                }
            }
        }
    }

    read_correctness!(test_u16, u16, 2, read_u16, ::std::u16::MAX);
    read_correctness!(test_i16, i16, 2, read_i16, ::std::i16::MAX);
    read_correctness!(test_u32, u32, 4, read_u32, ::std::u32::MAX);
    read_correctness!(test_i32, i32, 4, read_i32, ::std::i32::MAX);
}
