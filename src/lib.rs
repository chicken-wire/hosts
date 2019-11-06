//! # Hosts
//!
//! `hosts` is a collection of tools to parse [hosts](https://en.wikipedia.org/wiki/Hosts_(file)) files.
//!
//! # Example
//! ```
//! use hosts::{Host, HostsFile};
//!
//! // Create a host
//! let host = Host::new("127.0.0.1", "localhost", None);
//!
//! // Create a hosts file
//! let hosts_file = HostsFile::from(vec!(host));
//!
//! // Write the hosts file
//! hosts_file.write("hosts");
//! # std::fs::remove_file("hosts");
//! ```

pub mod host;
pub mod parser;

pub use host::Host;
pub use parser::HostsFile;
