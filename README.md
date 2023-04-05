Kahuna3D
======

Fork of https://github.com/OutOfTheVoid/kahuna which makes sets more useable and provides a 3D implementation.
- Allows for run-time determination of all_state
- Use bevy_utils hash set implementation
- Add Cube Grid

Removed bitset implementation - use the original repo if you want to use bitsets.

[![Crates.io](https://img.shields.io/crates/v/kahuna.svg?label=Kahuna)](https://crates.io/crates/kahuna) [![docs.rs](https://docs.rs/kahuna/badge.svg)](https://docs.rs/kahuna/)

This crate is a basic extensible implementation of [Wave Function Collapse](https://www.youtube.com/watch?v=2SuvO4Gi7uY)

## Features

- Support for custom grids of arbitrary dimension and topology, as long as there is an upper bound to cell neighbors
- Basic square grid implementation provided

## Examples


## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
