//! # Hosts
//!
//! `hosts` is a collection of tools to parse [hosts](https://en.wikipedia.org/wiki/Hosts_(file)) files.

pub mod host;
pub mod parser;

pub use host::Host;
pub use parser::read;
pub use parser::write;
