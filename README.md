# Standard Paths

A Rust library providing methods for accessing standard paths on the local filesystem (config, cache, user directories and etc.).

It's a port of [QStandardPaths](https://doc.qt.io/qt-5/qstandardpaths.html) class of the Qt framework.

[![Crates Version](https://img.shields.io/crates/v/standard_paths.svg)](https://crates.io/crates/standard_paths)
[![Crates Downloads](https://img.shields.io/crates/d/standard_paths.svg)](https://crates.io/crates/standard_paths)
[![Documentation](https://docs.rs/standard_paths/badge.svg)](https://docs.rs/standard_paths)

### Currently implemented for Linux and Windows

- [writable_location](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.writable_location)
- [standard_locations](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.standard_locations)
- [find_executable](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.find_executable)
- [find_executable_in_paths](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.find_executable_in_paths)
- [locate](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.locate)
- [locate_all](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.locate_all)

### macOS support

macOS is currently unsupported. If you want to help with macOS feel free to contribute!

### Usage

#### Cargo.toml

```toml
[dependencies]
standard_paths = "^1.0"
```

#### main.rs

```rust
use standard_paths::{LocationType, StandardPaths};

fn main() {
    let sp = StandardPaths::new("app", "org");
    println!("{:?}", sp.writable_location(LocationType::AppLocalDataLocation));
}
```
