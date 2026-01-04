# Fig Language (Rust)

Fig is a small, pragmatic scripting language implemented in Rust. This repository contains the full pipeline — lexer, parser, and evaluator — plus a simple CLI to run `.fg` files or scaffold new Fig projects.

## Overview

- File extension: `.fg`
- Default entry resolution: prefers `src/main.fg`, falls back to `./main.fg`
- Zero external dependencies; built on Rust std only
- Friendly CLI: `fig run [path]` and `fig new <name>`

### Quick Example

```text
# Fig starter
var x = 0;

for i in range(3) {
    print(i);
}

fn add(a, b) {
    return a + b;
}

print(add(3, 4));
```

## Features

- Language basics: `var` declarations, `fn` functions, `return`
- Control flow: `for` loops with `range(...)`
- Built-ins: `print(...)` for console output
- CLI commands:
  - `fig run [path]` — run a file or resolve an entry inside a directory
  - `fig new <name>` — scaffold a new Fig project with `src/main.fg`
- Entry discovery: `src/main.fg` (preferred) → `./main.fg` (fallback)
- Clean module layout: lexer, parser, evaluator, environment, tokens, types
- No third-party crates; fast builds and easy maintenance

## Tech Stack

- Language: Rust (Edition 2024)
- Build tool: Cargo
- Runtime: Single binary (`fig`) produced by Cargo
- Dependencies: none (uses Rust standard library)

## Getting Started

### Prerequisites

- Rust toolchain via rustup
- macOS, Linux, or Windows (Rust offers cross-platform builds)

### Build

```bash
cargo build
```

### Run (default entry)

Runs `src/main.fg` if present, otherwise `./main.fg`:

```bash
cargo run
```

### Run a specific target

- Run a directory (resolves its entry):

```bash
cargo run -- run .
```

- Run an explicit file:

```bash
cargo run -- run path/to/script.fg
```

### Scaffold a new project

Creates `<name>/src/main.fg` with a starter program:

```bash
cargo run -- new hello
```

### Direct binary usage

After a build, you can call the binary directly:

```bash
./target/debug/fig run path/to/script.fg
./target/debug/fig new myproject
```

## Project Structure

- [`src/main.rs`](src/main.rs) — CLI entry, entry resolution, runner, scaffolder
- [`src/lexer.rs`](src/lexer.rs) — tokenization of Fig source
- [`src/parser.rs`](src/parser.rs) — AST construction from tokens
- [`src/evalulator.rs`](src/evalulator.rs) — program evaluation (interpreter)
- [`src/enviorment.rs`](src/enviorment.rs) — runtime environment & scope handling
- [`src/token.rs`](src/token.rs) — token kinds used by the lexer/parser
- [`src/types.rs`](src/types.rs) — core value and type representations
- [`main.fg`](main.fg) — example entry at repository root
- [`example.fg`](example.fg) — additional sample script

## Development

- Format code:

```bash
cargo fmt
```

- Lint (optional, if `clippy` is installed):

```bash
cargo clippy --all-targets --all-features
```

- Run in debug:

```bash
cargo run -- run path/to/script.fg
```

- Build optimized:

```bash
cargo build --release
```

## Roadmap (Ideas)

- Expand the standard library (I/O, collections, math)
- Richer type system and error messages
- REPL mode for interactive exploration
- Packaging and module imports
- More control-flow constructs and pattern matching

## Contributing

Issues and pull requests are welcome. Please:

- Keep changes focused and well-tested
- Follow Rust style (`cargo fmt`) and run lints (`cargo clippy`)
- Discuss larger ideas in issues before implementation

## License

No license file is currently present. If you plan to publish or share widely, consider adding a LICENSE (e.g., MIT or Apache-2.0) in the repository root.
