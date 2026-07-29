# lsrs

A simplified implementation of the Unix `ls` command written in Rust.

The goal of this project is **not** to replace GNU/BSD `ls`, but to understand how directory listing utilities work by building one from scratch and interacting with Unix filesystem metadata directly.

This project is part of a larger systems programming series where classic Unix utilities are reimplemented to learn operating system concepts, Rust, and performance engineering.

---

## Features

- List directory contents
- Specify a target directory
- Long listing (`-l`)
- Show hidden files (`-a`)
- Combined flags (`-la`, `-al`)
- Manual Unix permission formatting
- Username and group lookup
- Manual Unix timestamp formatting (without `chrono`)

---

## Current Output

```text
drwxr-xr-x 16 debm staff 512 29 Jul 15:11 src
-rw-r--r--  1 debm staff 2637 29 Jul 15:09 Cargo.toml
```

---

## Project Structure

```text
.
├── src
│   ├── cli.rs
│   ├── fs_ops.rs
│   ├── output.rs
│   ├── main.rs
│   └── format
│       ├── mod.rs
│       ├── ownership.rs
│       ├── permissions.rs
│       └── time.rs
├── benchmark/
├── notes.md
├── spec.md
├── Cargo.toml
└── README.md
```

---

## Building

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

---

## Usage

List current directory:

```bash
cargo run
```

List another directory:

```bash
cargo run -- ~/dev
```

Long listing:

```bash
cargo run -- -l ~/dev
```

Show hidden files:

```bash
cargo run -- -a ~/dev
```

Long listing with hidden files:

```bash
cargo run -- -la ~/dev
```

Or run the optimized binary:

```bash
./target/release/lsrs -la ~/dev
```

---

## Implementation Notes

This project intentionally avoids high-level abstractions where practical in order to better understand the underlying Unix APIs.

It currently demonstrates:

- directory traversal
- filesystem metadata
- Unix permission bits
- UID/GID lookup
- file ownership
- modification timestamps
- Rust module organization

---

## Planned Features

- Recursive listing (`-R`)
- Human-readable sizes (`-h`)
- Sorting
- Colored output
- Symbolic link support
- Multiple directory arguments
- Better error reporting
- Benchmarking against the system `ls`

---

## Learning Objectives

While building this project, the following Unix concepts were explored:

- `std::fs::read_dir`
- `DirEntry`
- `Metadata`
- Unix permission bits
- `MetadataExt`
- `getpwuid()`
- `getgrgid()`
- Module organization in Rust
- Error propagation using `Result`

---

## Benchmarking

A `benchmark/` directory is included for comparing `lsrs` against the system `ls`.

Future benchmarks will measure:

- execution time
- syscall overhead
- memory usage
- scalability on large directories

---

## License

MIT
