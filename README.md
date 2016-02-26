# rust-endianness
Rust lib for reading numbers in big-endian and little-endian.

## Usage
```rust
extern crate endianness;
use endianness::*;

let v = vec![0, 128, 128, 0];

assert_eq!(-32768, read_i16(&v[0..2], ByteOrder::LittleEndian).unwrap());
assert_eq!(-32768, read_i16(&v[2..4], ByteOrder::BigEndian).unwrap());

match read_i32(&v, ByteOrder::LittleEndian) {
    Ok(n) => println!("Read value {}", n),
    Err(err) => println!("Error: {}", err),
}
```
