# Standard Paths

A Rust library providing methods for accessing standard paths on the local filesystem (config, cache, user directories and etc.).

It's a port of [QStandardPaths](https://doc.qt.io/qt-5/qstandardpaths.html) class of the Qt framework.

##### [Documentation](https://docs.rs/standard_paths)

### Currently implemented

#### Unix
- writable_location
- standard_locations
- find_executable
- find_executable_in_paths
- locate
- locate_all

#### Windows
- writable_location
- standard_locations
- find_executable
- find_executable_in_paths
- locate
- locate_all

### Usage

#### Cargo.toml

```toml
[dependencies]
standard_paths = "0.3.1"
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
