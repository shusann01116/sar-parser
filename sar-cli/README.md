# sar-cli

Command-line interface for PSO2 SymbolArt (SAR) file processing.

## Installation

### From Source

```bash
cargo install --path sar-cli
```

### From Crates.io

```bash
cargo install sar-cli
```

## Usage

```shell
$ sar-cli --help
Usage: sar-cli [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Path to the SAR file or directory
  -o, --output <OUTPUT>  Path to the output directory
      --raise-error      Raise errors instead of ignoring them
      --overwrite        Overwrite existing files
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

Process a single SAR file:

```bash
sar-cli -i input.sar -o output/
```

Process all SAR files in a directory:

```bash
sar-cli -i input_directory/ -o output/
```

## Features

- Process single SAR files or entire directories
- Configurable error handling
- Overwrite protection for existing files
- Fast and efficient processing using parallel execution

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
