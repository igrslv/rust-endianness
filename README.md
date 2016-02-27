# rust-endianness
Library provides functions to read numbers from a stream of bytes either in big-endian or little-endian. 

Functions return Result type instead of panic!().

[![Build Status](https://travis-ci.org/igorsolovyov/rust-endianness.svg?branch=master)](https://travis-ci.org/igorsolovyov/rust-endianness)

## Usage

Read signed 16-bit integers:

```rust
extern crate endianness;
use endianness::*;

let v = vec![0, 128, 128, 0];

assert_eq!(-32768, read_i16(&v[0..2], ByteOrder::LittleEndian).unwrap());
assert_eq!(-32768, read_i16(&v[2..4], ByteOrder::BigEndian).unwrap());
```

Read a signed 32-bit integer:

```rust
extern crate endianness;
use endianness::*;

let v = vec![0, 128, 128, 0];

match read_i32(&v, ByteOrder::LittleEndian) {
    Ok(n) => println!("Read value {}", n), // 8421376
    Err(err) => println!("Error: {}", err),
}
```
