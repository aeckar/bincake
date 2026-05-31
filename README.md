# bincake

Serde-free deterministic binary serialization.

[![Rust](https://github.com/aeckar/bincake/actions/workflows/rust.yml/badge.svg)](https://github.com/aeckar/bincake/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/bincake.svg)](https://crates.io/crates/bincake)

## Documentation

[https://docs.rs/bincake](https://docs.rs/bincake)

## Overview

`bincake` serializes and deserializes Rust types to and from compact little-endian binary,
with no dependency on `serde`.

- Deterministic output — identical data always produces identical bytes
- Derive macros for automatic implementation on custom types
    - Controlled by `derive` feature flag
- Built on [`taped`](https://crates.io/crates/taped) for zero-allocation byte reading
- Numeric types, strings, vecs, and tuples supported out of the box

Originally developed as the bytecode serialization format for [`rvm`](https://github.com/aeckar/rvm),
extracted as a standalone library after proving stable under real usage.

## Example

```rust
use bincake::*;

#[derive(Serialize)]
struct Instruction {
    opcode: u8,
    operand: u32,
}

// Serialize
let instr = Instruction { opcode: 0x01, operand: 42 };
let mut dest = vec![];
dest.write(instr);

// Deserialize
let mut src = bytes.to_tape();
let instr = src.read::<Instruction>();
```

## When to use this

- Serializing bytecode, binary protocols, or other compact binary formats
- Anywhere deterministic output is required (content hashing, signing)
- Projects where `serde` compile times are a concern
- `no_std` environments

## When not to use this

- Human-readable formats → use [`serde`](https://serde.rs) with `serde_json` or `toml`
- Schema evolution and forward compatibility → use [`prost`](https://github.com/tokio-rs/prost) (protobuf)
- Maximum encode/decode performance → use [`rkyv`](https://github.com/rkyv/rkyv)
- Interoperability with other languages or systems → use [`bincode`](https://github.com/bincode-org/bincode)
