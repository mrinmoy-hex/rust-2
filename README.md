# Rust practice

A Cargo workspace of small Rust examples and projects I'm working through while learning the language. Each topic folder is an independent crate with its own `Cargo.toml` and `src/` entry point.

The folders cover topics like strings, vectors, generics, traits, lifetimes, and error handling.

Run a binary crate from the workspace root with:

```sh
cargo run -p vectors
```

Run an example target with:

```sh
cargo run -p vectors --example vec1
```

Check or test all workspace crates with:

```sh
cargo check --workspace
cargo test --workspace
```
