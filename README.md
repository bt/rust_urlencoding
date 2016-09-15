# urlencoding

[![Latest Version](https://img.shields.io/crates/v/urlencoding.svg)](https://crates.io/crates/urlencoding)

A Rust library for doing URL percentage encoding.

Installation
============

This crate can be downloaded through Cargo. To do so, add the following line to your `Cargo.toml` file, under `dependencies`:

```toml
urlencoding = "0.1"
```

Usage
=====

To encode a string, do the following:

```rust
extern crate urlencoding;

use urlencoding::encode;

fn main() {
  let encoded = encode("This string will be URL encoded.");
  // This%20string%20will%20be%20URL%20encoded.
}
```

Decoding is pending implementation.
