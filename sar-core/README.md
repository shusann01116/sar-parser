# sar-core

Core library for parsing and processing PSO2 SymbolArt (SAR) files.

## Features

- Parse SAR files and extract their contents
- Process and manipulate SymbolArt data
- Support for various SAR file formats
- Error handling with detailed error messages

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sar-core = "0.1.0"
```

Example usage:

```rust
use sar_core::SarParser;

// Parse a SAR file
let parser = SarParser::new();
let result = parser.parse_file("path/to/file.sar")?;

// Process the parsed data
for symbol in result.symbols {
    println!("Symbol: {:?}", symbol);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
