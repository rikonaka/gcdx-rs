# gcdx-rs

Calculate the greatest common divisor of multiple values.

[![Rust](https://github.com/rikonaka/gcdx-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/gcdx-rs/actions/workflows/rust.yml)

## Example

```rust
use gcdx::gcdx;

fn main() {
    let v = vec![10, 9, 8, 7];
    let g = gcdx_euclidean(&v).unwrap();
    println!("{}", g);
    assert_eq!(g, 1);
}
```
