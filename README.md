# dirlines

A fast, lightweight recursive code statistics tool written in Rust. It scans directories, categorizes files by extension, and breaks down code lines, comment lines, and blank/void lines into a clean terminal table.

## Features

- **Recursive Scanning**: Efficiently walks through directory trees.
- **Multi-Language Awareness**: Custom comment-parsing rules for C/C++, Python, JavaScript/TypeScript, Rust, HTML/Markdown, SQL, Shell scripts, JSON, and more.
- **Clean Table Output**: Automatically formats columns and truncates long file extensions (e.g., `.gitmodules` $\rightarrow$ `.gitmod...`).
- **Zero Dependencies for Modularity**: Built with a flat, clean module structure.

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone [https://github.com/cvrspark/dirlines.git](https://github.com/cvrspark/dirlines.git)
cd dirlines
cargo build --release
```

## Usage

Run `dirlines` in your current directory:

```bash
./target/release/dirlines
```

Or target a specific directory path:

```bash
./target/release/dirlines /path/to/project
```

### CLI Options

* `[PATH]`: Target directory to analyze (defaults to the current directory `.`)
* `--help`, `-h`: Print the help message
* `--version`, `-V`: Print version information
