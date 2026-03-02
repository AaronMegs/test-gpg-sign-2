# depot-gbk

Character encoding detection tools for GBK and other encodings.

This repository contains both Go and Rust implementations for character encoding detection.

## Go Implementation

The Go version uses the `chardet` library to detect character encodings.

```bash
go run main.go
```

## Rust Implementation

The Rust version provides both a library and a command-line tool for character encoding detection.

### Building

```bash
cargo build --release
```

### Usage

#### Command Line Tool

```bash
# Detect encoding from text
./target/release/depot-gbk-rust --text "这是一段测试文本。"

# Detect encoding from file
./target/release/depot-gbk-rust --file test.java

# Detect encoding from stdin
echo "Hello World" | ./target/release/depot-gbk-rust

# Show help
./target/release/depot-gbk-rust --help
```

#### Library Usage

```rust
use depot_gbk_rust::detect_encoding_detailed;

let text = "这是一段测试文本。";
let result = detect_encoding_detailed(text.as_bytes());
println!("Charset: {}, Confidence: {:.2}%", result.charset, result.confidence * 100.0);
```

### Features

- Character encoding detection using chardet
- UTF-8 validation
- GBK encoding detection heuristics
- Command-line interface with flexible input options
- Library API for programmatic use
- Comprehensive test suite
