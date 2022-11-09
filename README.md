# Leo's Music Shop in Rust

Another cool API for the Music Shop, written in Rust.

## Motivation

- Learning Rust
- Integrate Segment using pure HTTP, but somebody already [wrote a library](https://crates.io/crates/segment).

## Architecture

- [Diesel](http://diesel.rs/): ORM
- [R2D2](https://github.com/sfackler/r2d2): Database connection pool
- [Actix](https://actix.rs/): Web Framework

## Building

```sh
cargo build
```

## Running

```sh
cargo run
```

## Troubleshooting

### Problems while building/linking

```sh
cargo clean
cargo build
```