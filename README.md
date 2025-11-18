# gib2sgf

Converts Tygem GIB game records into SGF.

![Rust](https://github.com/komu/gib2sgf/workflows/Rust/badge.svg)

## Features

- Recursively scans a directory for .gib files and converts them to .sgf
- Normalizes quirky Tygem file names (e.g. trailing .dll/.gib/periods)
- Preserves original file modification time on the generated SGF
- Skips files that already have a corresponding .sgf next to them
- Library API and WebAssembly binding in addition to the CLI

## Installation

From crates.io:

```
cargo install gib2sgf
```

From source (this repository):

```
git clone https://github.com/komu/gib2sgf.git
cd gib2sgf
cargo install --path .
```

## Usage (CLI)

Run against a directory to convert all .gib files (recursively). If a matching .sgf already exists, it will be skipped.

```
gib2sgf /my/sgf/dir
Convert "/my/sgf/dir/tygem/komula(4D)_zangpung(4D)_202004012236.gib" -> "/my/sgf/dir/tygem/komula(4D)_zangpung(4D)_202004012236.sgf"
Convert "/my/sgf/dir/tygem/fingersid(2D)_komula(2D)_202003142229.gib" -> "/my/sgf/dir/tygem/fingersid(2D)_komula(2D)_202003142229.sgf"
Skip    "/my/sgf/dir/tygem/komula(4D)_ysoo5(4D)_202004142142.gib" -> "/my/sgf/dir/tygem/komula(4D)_ysoo5(4D)_202004142142.sgf"
```

- Without arguments, the current directory (.) is used.
- Filenames like foo.gib.dll... will be normalized to foo.gib before conversion, and the output will be foo.sgf next to it.
- When nothing needs conversion, the tool prints: `no unconverted files`.

## Library usage (Rust)

Add to Cargo.toml:

```
gib2sgf = "0.1"
```

Example:

```rust
use gib2sgf::gib_to_sgf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gib = std::fs::read_to_string("game.gib")?;
    let sgf = gib_to_sgf(&gib)?; // returns Result<String, _>
    std::fs::write("game.sgf", sgf)?;
    Ok(())
}
```

## WebAssembly

This crate exposes a small WASM-friendly wrapper function through wasm-bindgen:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_gib_to_sgf(input: &str) -> Option<String> {
    gib2sgf::gib_to_sgf(input).ok()
}
```

Build with wasm-pack (release profile uses wasm-opt -Oz by default):

```
wasm-pack build --release
```

## Development

Run tests:

```
cargo test
```

Run the converter against sample data (current dir by default):

```
cargo run -- /path/to/tygem/export
```

## License

MIT, see LICENSE.
