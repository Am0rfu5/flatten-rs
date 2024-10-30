# `flattenrs`

## Overview
`flattenrs` is a Rust-based command-line tool that consolidates all files within a specified directory into a single output file. The tool supports options for including/excluding specific files or directories, processing hidden files, and managing output formats for various file types. With its flexible CLI and syntax highlighting support, `flattenrs` is ideal for quick aggregation of files for documentation, backup, or analysis.

## Features
- **Directory Flattening**: Recursively includes all files in a directory into one output file.
- **Include/Exclude Filtering**: Specify which files or directories to include or exclude.
- **Hidden Files Support**: Toggle whether hidden files are included in the output.
- **Syntax Highlighting**: Applies syntax formatting based on file extensions.

## Installation
To install `flattenrs`, ensure you have Rust and Cargo installed, then clone the repository and run:

```bash
git clone https://github.com/username/flattenrs.git
cd flattenrs
cargo build --release
```

This will produce an executable in the `target/release` directory.

## Usage
The CLI offers a variety of options for custom usage:

```bash
flattenrs [FLAGS] [OPTIONS]
```

### Arguments & Options

| Flag/Option      | Description                                    | Example Usage                |
|------------------|------------------------------------------------|------------------------------|
| `--directory`    | The directory to flatten. Defaults to `.`      | `--directory ./src`          |
| `-o`, `--output` | Specify the output file path                   | `-o flattened_output.txt`    |
| `-e`, `--exclude`| Exclude specific files or directories          | `-e path/to/exclude`         |
| `-i`, `--include`| Include specific files or directories only     | `-i path/to/include`         |
| `-h`, `--allow_hidden`| Allow hidden files to be processed       | `-h`                         |

**Example Command**
To flatten files in a specific directory, excluding certain files and including hidden files, use:

```bash
flattenrs --directory ./my_project -o output.txt -e .git -h
```

### Default Output File
If no output file is specified, the program generates one with the format `flattenrs-[directory]-YYYY-MM-DD_HH-MM-SS.txt`.

## Example Output Format
The tool aggregates files using syntax-aware markdown-style comments for better readability:

```
## src/main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```

## LICENSE
``plaintext
MIT License
...
```

## Development

This project uses **Rust’s structopt** for CLI parsing and **syntect** for syntax highlighting. To set up the project:

1. Clone the repository.
2. Run `cargo build` to compile.
3. For testing, use `cargo test`.

## Contributing

We welcome contributions to `flattenrs`! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -am 'Add my feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a pull request.
