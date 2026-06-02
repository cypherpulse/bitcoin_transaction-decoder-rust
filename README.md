# Transaction Decoder — Rust For Bitcoiners

[![Build](https://img.shields.io/badge/build-manual-lightgrey)](https://github.com/)
[![Rust](https://img.shields.io/badge/rust-1.72+-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/badge/crate-transaction--decoder-blue)](https://crates.io/)
[![License](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

A compact, educational Bitcoin transaction decoder implemented in Rust. This project is a hands-on learning journey aimed at Rust learners who want to understand Bitcoin transaction serialization, parsing, and the Rust concepts commonly used when implementing low-level protocol parsers.

## Table of Contents

- [About](#about)
- [Badges](#badges)
- [Quickstart](#quickstart)
- [How it works](#how-it-works)
  - [Decode Flow (Mermaid)](#decode-flow-mermaid)
- [Project structure](#project-structure)
- [Key Rust concepts used](#key-rust-concepts-used)
- [Examples](#examples)
- [Development notes](#development-notes)
- [Contributing](#contributing)
- [Further reading](#further-reading)

## About

This repository contains a small command-line utility, `transaction-decoder`, that accepts raw Bitcoin transaction hex and prints a human-readable breakdown of the transaction: version, inputs, outputs, locktime, and script information. The purpose is educational — to show how binary parsing and protocol decoding are implemented idiomatically in Rust.

## Badges

- Build: placeholder — update when CI is added.
- Rust: indicates the targeted Rust toolchain.
- Crates.io: placeholder (not published by default).

## Quickstart

Prerequisites: Rust toolchain (stable or the edition specified in Cargo.toml).

Build and run locally:

```bash
cargo build --release
cargo run --release -- <raw-transaction-hex>
```

Example (replace the hex with a real transaction hex):

```bash
cargo run --release -- 0100000001...deadbeef
```

## How it works

At a high level the program:

1. Accepts hex input from the command line.
2. Converts hex to bytes using the `hex` crate.
3. Walks the byte slice, parsing fields according to Bitcoin's transaction serialization format.
4. Prints a readable representation.

### Decode Flow (Mermaid)

```mermaid
flowchart TD
  A[Start: raw hex input] --> B[hex::decode -> Vec<u8>]
  B --> C[Create byte slice & cursor]
  C --> D[Parse version (4 bytes)]
  D --> E[Parse varint -> input count]
  E --> F[Loop parse inputs]
  F --> G[Parse outputs count]
  G --> H[Loop parse outputs]
  H --> I[Parse locktime (4 bytes)]
  I --> J[Format & print result]

  subgraph Input parsing
    F --> F1[prev_txid (32 bytes)]
    F1 --> F2[vout (4 bytes)]
    F2 --> F3[script length (varint)]
    F3 --> F4[script bytes]
    F4 --> F5[sequence (4 bytes)]
  end

  subgraph Output parsing
    H --> O1[value (8 bytes)]
    O1 --> O2[script length (varint)]
    O2 --> O3[script bytes]
  end
```

## Project structure

- `src/main.rs` — CLI entrypoint and top-level parsing orchestration.
- `Cargo.toml` — dependency manifest (this project uses the `hex` crate for hex decoding).

If you split the parser into modules, typical modules would include:

- `parser` — the binary parsing helpers (read_u32_le, read_u64_le, read_varint, etc.).
- `types` — transaction, input, and output data structures.
- `fmt` — pretty-printing helpers for scripts, addresses, and values.

## Key Rust concepts used

This project intentionally demonstrates several Rust language and ecosystem concepts useful for systems programming and protocol parsing:

- Ownership and borrowing: parsing operates on a `&[u8]` slice and advances a cursor; functions borrow slices rather than copying, avoiding allocations.
- Slices and indexing: safely access subranges of a byte slice using checked operations and `get()`/`try_from` to avoid panics.
- Iterators and loops: use iterator adapters and explicit `while`/`for` loops when parsing repeated structures.
- Pattern matching: `match` is used heavily to decode varints and script opcodes.
- Result and error handling: parser functions return `Result<T, E>` to propagate recoverable parsing errors cleanly.
- Little-endian conversions: parse integers from byte slices using `u32::from_le_bytes` and `u64::from_le_bytes`.
- External crates: the `hex` crate is used for converting hex strings to bytes — a great example of using small, focused crates from crates.io.

Code snippets illustrating ideas:

```rust
// read 4 bytes as little-endian u32
fn read_u32_le(cursor: &mut usize, buf: &[u8]) -> Result<u32, ParseError> {
    if *cursor + 4 > buf.len() { return Err(ParseError::UnexpectedEof); }
    let mut tmp = [0u8; 4];
    tmp.copy_from_slice(&buf[*cursor..*cursor+4]);
    *cursor += 4;
    Ok(u32::from_le_bytes(tmp))
}
```

## Examples

Run the binary with a sample transaction hex and inspect output. Add your own known transactions to test script parsing, segwit flag handling, and unusual script types.

## Development notes

- The parser is intentionally straightforward (no macro magic) to keep the logic readable for learners.
- Consider adding unit tests that decode known test vectors from Bitcoin Core.
- Add CI that runs `cargo test` and `cargo clippy`.

## Contributing

Contributions are welcome. If you'd like to add features (bech32 address derivation, script analysis, segwit support), open an issue or PR with an explanation and test vectors.

## Further reading

- Bitcoin Developer Guide — Transactions: https://developer.bitcoin.org/devguide/transactions.html
- Rust book: https://doc.rust-lang.org/book/
- Bitcoin serialization spec (BIPs): https://en.bitcoin.it/wiki/Protocol_documentation

---

If you'd like, I can:

- Add example test vectors and run local tests.
- Split `main.rs` into modules and add unit tests for parsing primitives.

Happy learning! — the transaction-decoder learning journey
