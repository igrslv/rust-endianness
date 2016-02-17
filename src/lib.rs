use std::fmt;
use std::error;
use std::result;

#[derive(Debug)]
pub enum ByteOrder {
    /// Intel byte order
    LittleEndian,
    /// Motorola byte order
    BigEndian,
}

#[derive(Debug,PartialEq)]
pub enum Error {
    ShortSlice,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ShortSlice => write!(f, "The slice length is too short to read."),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ShortSlice => "The slice length is too short to read.",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ShortSlice => None,
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
                Ok((data[0] as u16) * 256 + (data[1] as u16))
            }
            ByteOrder::LittleEndian => {
                Ok((data[1] as u16) * 256 + (data[0] as u16))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_u16_should_return_a_correct_u16_integer_for_the_first_two_elements_in_slice() {
        let big_endian_result = read_u16(&[1, 2], ByteOrder::BigEndian).unwrap();
        assert_eq!(big_endian_result, 258);

        let little_endian_result = read_u16(&[1, 2], ByteOrder::LittleEndian).unwrap();
        assert_eq!(little_endian_result, 513);
    }

    #[test]
    fn read_u16_should_return_an_error_type_if_there_are_less_than_two_elements_in_slice() {
        let result = read_u16(&[1], ByteOrder::BigEndian).unwrap_err();
        assert_eq!(result, Error::ShortSlice);
    }
}
