# Byte array literals

This crate provides the `hb!` macro for creating fixed size byte arrays
at compile time from hexadecimal byte literals.

```rust,no-test
#![feature(proc_macro)]

extern crate hexbytes;
use hexbytes::hb;

fn main() {
    let bytes = hb!("0bad abba cd");
}
```
