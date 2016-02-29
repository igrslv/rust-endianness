# rust-endianness
Library provides functions to read numbers from a stream of bytes either in big-endian or little-endian. 
Functions return Result type.

[![Build Status](https://travis-ci.org/igrslv/rust-endianness.svg?branch=master)](https://travis-ci.org/igrslv/rust-endianness)
[![](http://meritbadge.herokuapp.com/endianness)](https://crates.io/crates/endianness)

## Installation
Add the package to your Cargo.toml:

```toml
[dependencies]
endianness = "0.1"
```
and then import it:

```rust
extern crate endianness;
use endianness::*;
```

## Usage

Read signed 16-bit integers:

```rust
let v = vec![0, 128, 128, 0];

assert_eq!(-32768, read_i16(&v[0..2], ByteOrder::LittleEndian).unwrap());
assert_eq!(-32768, read_i16(&v[2..4], ByteOrder::BigEndian).unwrap());
```

Read a signed 32-bit integer:

```rust
let v = vec![0, 128, 128, 0];

match read_i32(&v, ByteOrder::LittleEndian) {
    Ok(n) => println!("Read value {}", n), // 8421376
    Err(err) => println!("Error: {}", err),
}
```

Read a single-precision floating point number:

```rust
let v = vec![194, 255, 0, 0];
assert_eq!(-127.5, read_f32(&v[0..4], ByteOrder::BigEndian).unwrap());
```
