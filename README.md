# bincake

um

[![Crates.io](https://img.shields.io/crates/v/bincake.svg)](https://crates.io/crates/bincake)

## Documentation

[https://docs.rs/bincake](https://docs.rs/bincake)

## Overview

`Tape` wraps a byte slice with a position cursor, providing methods for
scanning, backtracking, and consuming bytes without allocating.

- Optimized character and string search via [`memchr`](https://github.com/BurntSushi/memchr)
- Simple lookahead and lookbehind of whitespace characters
- Indentation counting and paragraph/line awareness

Originally developed as the byte reader for [`bincake`](https://github.com/aeckar/bincake),
extracted as a standalone primitive after the same pattern appeared
across multiple projects.

## Example

```rust
use taped::Tape;

let data = b"hello world";
let mut tape = Tape::new(data);

tape.seek(|&b| b == b' '); // advance to space
let word = tape.consume(|&b| b != b' '); // consume "hello"
assert_eq!(word, b"hello");
```

## When to use this

- Writing a parser for a binary or text format
- Scanning byte sequences without regex or [`nom`](https://github.com/rust-bakery/nom)
- Anywhere you need backtracking via cheap position snapshots (`tape.clone()`)

## When not to use this

- Full parser combinator framework → use [`nom`](https://github.com/rust-bakery/nom) or [`winnow`](https://github.com/winnow-rs/winnow)
- Lexer generation → use [`logos`](https://github.com/maciejhirsz/logos)
- Async byte streams → use [`tokio::io::AsyncBufRead`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufRead.html)
