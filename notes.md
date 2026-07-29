# Notes

## Objective

This project is part of a systems programming series.

The purpose is not to replace GNU/BSD `ls`, but to understand how the command
is implemented using Unix APIs and Rust's standard library.

---

# What I Learned

## 1. `read_dir()` only returns directory entries

```rust
fs::read_dir(path)
```

returns an iterator of `DirEntry`.

It does **not** provide metadata.

To obtain

- permissions
- owner
- timestamps
- file size

I must call

```rust
entry.metadata()
```

which performs an additional `stat()`-like operation.

---

## 2. Directory entries vs Metadata

`DirEntry`

contains

- filename
- path

`Metadata`

contains

- permissions
- timestamps
- owner
- inode
- size

This separation mirrors the Unix API.

---

## 3. Unix permissions

Permissions are stored inside

```
st_mode
```

Example

```
0755
```

becomes

```
rwxr-xr-x
```

The first character is **not** a permission.

It represents the file type.

Examples

```
-
regular file

d
directory

l
symbolic link
```

---

## 4. Owner and Group

Rust's standard library exposes

```
uid()
gid()
```

but not the corresponding names.

To obtain usernames I used

```
libc::getpwuid()
```

Groups use

```
libc::getgrgid()
```

These functions return C structures, requiring

```
unsafe
```

and conversion from

```
char *
```

to Rust strings.

---

## 5. MetadataExt

Several useful Unix fields are only available through

```rust
std::os::unix::fs::MetadataExt
```

Examples

- uid()
- gid()
- nlink()
- size()
- ino()

These correspond directly to fields inside

```
struct stat
```

---

## 6. Time

Rust returns

```rust
SystemTime
```

instead of formatted dates.

Instead of using `chrono`, I implemented calendar conversion manually from Unix
seconds.

This helped understand that timestamps are simply seconds since

```
1970-01-01 00:00:00 UTC
```

---

## 7. Hidden files

Unix has no hidden-file attribute.

A file is hidden simply because its name begins with

```
.
```

---

## 8. Error handling

Most filesystem operations return

```
io::Result<T>
```

Using

```rust
?
```

greatly simplified error propagation.

---

## 9. Current Architecture

```
CLI
 │
 ▼
Argument parsing
 │
 ▼
Directory traversal
 │
 ▼
Filtering
 │
 ▼
Metadata retrieval
 │
 ▼
Formatting
 │
 ▼
Printing
```

---

# Future Improvements

## Features

- recursive listing (-R)
- multiple paths
- sorting
- colors
- symbolic link support
- human-readable sizes
- total block count
- inode display
- terminal width detection

---

## Performance

Benchmark against

```
/bin/ls
```

using

```
hyperfine
```

Measure

- execution time
- memory usage
- syscall count

---

## Unix APIs to Explore

While implementing future versions, investigate

- `lstat()`
- `fstat()`
- `getdents()` (Linux)
- `readdir()`
- `opendir()`
- `scandir()`
- extended attributes
- ACLs

---

## Interesting Observation

Most of the work performed by `ls -l` is **not printing text**.

For every directory entry, the program must retrieve metadata (effectively a `stat()` operation), and those filesystem lookups dominate the runtime on large directories. This means optimizing string formatting has little effect compared to reducing metadata accesses.
