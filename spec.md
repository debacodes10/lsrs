# lsrs Specification

## Goal

Implement a simplified version of the Unix `ls` utility in Rust to understand
filesystem traversal, Unix metadata, and POSIX APIs without relying on high-level
crates.

The objective is learning rather than feature parity with GNU/BSD `ls`.

---

## Supported Features

### Basic Listing

```bash
lsrs
lsrs <path>
```

Lists the contents of a directory.

Default path:

```
.
```

---

### Hidden Files

```bash
lsrs -a
```

Displays files beginning with `.`.

Without `-a`, hidden files are omitted.

---

### Long Listing

```bash
lsrs -l
```

Displays

- permissions
- hard link count
- owner
- group
- size
- last modification time
- filename

Example

```
drwxr-xr-x  16 debm staff 512 27 Jul 12:04 src
```

---

### Combined Flags

```bash
lsrs -la
lsrs -al
```

Equivalent to

```
-a -l
```

---

## Unsupported Features

The following are intentionally omitted.

- recursive listing (-R)
- sorting
- colors
- inode numbers
- block size
- symbolic link targets
- human-readable sizes (-h)
- time selection (-u, -c)
- column formatting
- ACL / extended attributes
- multiple directory arguments

---

## Internal Design

Program flow

```
parse arguments
        │
        ▼
read directory
        │
        ▼
filter hidden entries
        │
        ▼
collect metadata (if -l)
        │
        ▼
format output
```

---

## Modules / Functions

| Function | Responsibility |
|----------|----------------|
| parse_args | Parse CLI flags |
| read_entries | Read directory contents |
| filter_entries | Remove hidden files |
| print_entries | Simple output |
| print_long | Long listing |
| format_permissions | Decode Unix permission bits |
| username_from_uid | UID → username |
| groupname_from_gid | GID → group |
| format_modified | Format modification timestamp |

---

## Current Limitations

- assumes valid UTF-8 filenames
- modification time shown in UTC
- manual date conversion
- only directories are accepted as input
- minimal error reporting
