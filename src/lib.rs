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
                Ok( (data[0] as u16) * 256 + (data[1] as u16) )
            }
            ByteOrder::LittleEndian => {
                Ok( (data[1] as u16) * 256 + (data[0] as u16) )
            }
        }
    }
}

pub fn read_i16(data: &[u8], endianness: ByteOrder) -> Result<i16> {
    Ok( try!(read_u16(data, endianness)) as i16 )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u16() {
        assert_eq!(read_u16(&[1, 2], ByteOrder::BigEndian).unwrap(), read_u16(&[2, 1], ByteOrder::LittleEndian).unwrap());

        assert_eq!(Error::ShortSlice, read_u16(&[], ByteOrder::BigEndian).unwrap_err());
        assert_eq!(Error::ShortSlice, read_u16(&[], ByteOrder::LittleEndian).unwrap_err());
    }

    #[test]
    fn test_read_i16() {
        assert_eq!(read_i16(&[1, 255], ByteOrder::BigEndian).unwrap(), read_i16(&[255, 1], ByteOrder::LittleEndian).unwrap());
        assert_eq!(read_i16(&[255, 1], ByteOrder::BigEndian).unwrap(), read_i16(&[1, 255], ByteOrder::LittleEndian).unwrap());
        assert_eq!(read_i16(&[255, 255], ByteOrder::BigEndian).unwrap(), read_i16(&[255, 255], ByteOrder::LittleEndian).unwrap());
        assert_eq!(read_i16(&[128, 0], ByteOrder::BigEndian).unwrap(), read_i16(&[0, 128], ByteOrder::LittleEndian).unwrap());
        assert_eq!(read_i16(&[0, 128], ByteOrder::BigEndian).unwrap(), read_i16(&[128, 0], ByteOrder::LittleEndian).unwrap());

        // it should return an error type if the size of slice is less than two
        assert_eq!(Error::ShortSlice, read_i16(&[], ByteOrder::BigEndian).unwrap_err());
        assert_eq!(Error::ShortSlice, read_i16(&[], ByteOrder::LittleEndian).unwrap_err());
    }
}
