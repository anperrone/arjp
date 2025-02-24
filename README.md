# ARJP - Another Rust JSON Parser

[![CI Status](https://github.com/anperrone/arjp/actions/workflows/ci.yml/badge.svg)](https://github.com/anperrone/arjp/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/anperrone/arjp/branch/main/graph/badge.svg)](https://codecov.io/gh/anperrone/arjp)

A simple and efficient JSON parsing library written in Rust, designed to conform to the [JSON specification](https://www.json.org/). This library supports all JSON data types (objects, arrays, strings, numbers, booleans, and null) with a straightforward implementation and performance optimizations.

## Features

- **Full JSON Compliance**: Supports all JSON data types and escape sequences, including Unicode (`\uXXXX`).
- **Simplicity**: Minimal dependencies and a clear, easy-to-understand codebase.
- **Performance**: Optimized with pre-allocated data structures, efficient character handling, and minimal allocations.
- **Error Handling**: Detailed error messages for invalid JSON input.
- **Cross-Platform**: Tested on Linux, Windows, and macOS via GitHub Actions.
- **Extensible**: Modular design for easy additions or modifications.
- **Comprehensive Testing**: Includes both unit tests and integration tests for robust verification.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arjp = { git = "https://github.com/anperrone/arjp.git" }
```

Alternatively, if you want to use a local copy:

1. Clone the repository:

```bash
git clone https://github.com/anperrone/arjp.git
cd arjp
```

2. Add it as a local dependency in your Cargo.toml:

```toml
[dependencies]
arjp = { path = "../arjp" }
```

## Usage

Here's a simple example of how to use the library:

```rust
use arjp::{JsonParser, JsonValue};

fn main() -> Result<(), String> {
let json_str = r#"{"name": "John Doe", "age": 30, "scores": [1.23, 4.56]}"#;
let mut parser = JsonParser::new(json_str);
let value = parser.parse()?;

    match value {
        JsonValue::Object(map) => {
            println!("Name: {:?}", map.get("name"));
            println!("Age: {:?}", map.get("age"));
            println!("Scores: {:?}", map.get("scores"));
        }
        _ => println!("Expected an object"),
    }
    Ok(())

}
```

You can also use the convenience function `parse_json`:

```rust
use arjp::parse_json;

fn main() -> Result<(), String> {
    let json_str = r#""Hello \"world\"""#;
    let value = parse_json(json_str)?;
    println!("Parsed value: {:?}", value);
    Ok(())
}
```

## Building and Testing

### Prerequisites

- Rust stable (installed via rustup)

### Build

To build the library:

```bash
cargo build --release
```

### Run Tests

To run both unit and integration tests:

```bash
cargo test
```

- Unit Tests: Located within each source file (`src/value.rs`, `src/error.rs`, `src/parser.rs`) to verify internal functionality.
- Integration Tests: Located in `tests/integration.rs` to verify the public API.
- Formatting and Linting

### Ensure code style and quality:

```bash
cargo fmt # Format the code
cargo clippy # Run lint checks
```

### Code Coverage

To generate a coverage report locally:

1. Install `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

2. Run the coverage tool:

```bash
cargo tarpaulin --out Html
```

This generates an HTML report in `tarpaulin-report.html`.

Coverage is also tracked via Codecov in CI (see badges above).

### Benchmarks

Benchmarks are included using the `criterion` crate. To run them:

1. Install dependencies:

```bash
cargo bench --no-run # Downloads dependencies
```

2. Execute benchmarks:

```bash
cargo bench
```

Benchmarks are defined in `benches/arjp_bench.rs` and measure parsing performance processing a 32MB JSON file (`data/github.json`)
