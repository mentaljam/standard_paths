# Standard Paths

A Rust library providing methods for accessing standard paths on the local filesystem (config, cache, user directories and etc.).

It's a port of [QStandardPaths](https://doc.qt.io/qt-5/qstandardpaths.html) class of the Qt framework.

##### [Documentation](https://docs.rs/standard_paths)

### Currently implemented for Unix and Windows

- [writable_location](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.writable_location)
- [standard_locations](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.standard_locations)
- [find_executable](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.find_executable)
- [find_executable_in_paths](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.find_executable_in_paths)
- [locate](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.locate)
- [locate_all](https://docs.rs/standard_paths/~0/standard_paths/struct.StandardPaths.html#method.locate_all)

### Usage

#### Cargo.toml

```toml
[dependencies]
standard_paths = "^0.3"
```

#### main.rs

```rust
extern crate standard_paths;

use standard_paths::*;
use standard_paths::LocationType::*;

fn main() {
    let sp = StandardPaths::new_with_names("app", "org");
    println!("App data location: {:?}", sp.writable_location(AppLocalDataLocation));
}
```
