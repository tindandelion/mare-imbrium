//! Shared helpers for integration tests. Each `tests/*.rs` binary imports this via `mod common;`.
#![allow(dead_code)]
// Not every test binary uses every helper; suppress `dead_code` per submodule when building
// separate integration-test crates (`tests/*.rs`).

mod cube;

pub use cube::cube;
