# `flatten`

## Overview
`flatten` is a Rust-based command-line tool that consolidates all files within a specified directory into a single output file. The tool supports options for including/excluding specific files or directories, processing hidden files, and managing output formats for various file types. With its flexible CLI support, `flatten` allows for quick aggregation of files for analysis.

## Features
- **Directory Flattening**: Recursively includes all files in a directory into one output file.
- **Include/Exclude Filtering**: Specify which files or directories to include or exclude.
- **Hidden Files Support**: Toggle whether hidden files are included in the output.
- **.gitignore Respected**: Automatically excludes files listed in `.gitignore`.
- **.ignore File Support**: Excludes files listed in `.ignore` files.

## Installation
To install `flatten`, ensure you have Rust and Cargo installed, then clone the repository and run:

```bash
git clone https://github.com/username/flatten.git
cd flatten
cargo build --release
```
This will produce an executable in the `target/release` directory.

## Ignore Files and Directories

`flatten` respects `.gitignore` and `.ignore` files in the directory being flattened. If these files are present, `flatten` will exclude files and directories listed in them. This feature is useful for excluding build artifacts, configuration files, temporary files, and other unwanted content.

## CLI Usage Instructions

### Overview

```bash
flatten [FLAGS] [OPTIONS]
```
### Description
The `flatten` CLI tool provides a way to flatten directories into a single file, applying include and exclude filters as well as options for handling hidden files. This section describes each command-line option and provides usage examples.

### Command-Line Options

| Flag                   | Description                                                                             | Example                       |
|------------------------|-----------------------------------------------------------------------------------------|-------------------------------|
| `-- <directory>`          | Specifies the directory to flatten. Defaults to the current directory (`.`) if omitted. | `-- ./src    |
| `-o`,`--output`        | Defines the output file where the flattened content will be saved.                      | `--output ./flattened.txt`    |
| `-e`,`--exclude`       | Specifies files or directories to exclude during flattening. Can be used multiple times.| `--exclude ./file1.txt`       |
| `-i`, `--include`      | Specifies files or directories to include, overriding excludes. Can be used multiple times. | `--include ./file2.txt`   |
| `-h`, `--allow_hidden` | Allows hidden files to be included in the output. Without this flag, hidden files are skipped. | `--allow_hidden`       |

### Usage Examples

- **Basic Flattening of a Directory**:
  ```bash
  flatten -- ./src
  ```
  This command flattens all files in `./src` and saves them in `./flattened_output.txt`.

- **Exclude Specific Files or Directories**:
  ```bash
  flatten --output ./output.txt --exclude ./my_directory/ignore_me -- ./my_directory
  ```
  This command excludes `ignore_me` from `./my_directory` while flattening and saves the result in `output.txt`.

- **Include Only Specific Files**:
  ```bash
  flatten --output ./output.txt --include ./my_directory/important.txt
  ```
  This will process only `important.txt` and write it to `output.txt`.

- **Flatten with Hidden Files**:
  ```bash
  flatten --output ./output.txt --allow_hidden -- ./my_directory
  ```
  This will include hidden files in the output.

### Default Output File
If no output file is specified, the program generates one with the format `flatten-[directory]-YYYY-MM-DD_HH-MM-SS.txt`.

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
```plaintext
MIT License
...
```

## Development

This project uses **Rust’s structopt** for CLI parsing and **syntect** for syntax highlighting. To set up the project:

1. Clone the repository.
2. Run `cargo build` to compile.
3. For testing, use `cargo test`.

## Contributing

We welcome contributions to `flatten`! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -am 'Add my feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a pull request.
