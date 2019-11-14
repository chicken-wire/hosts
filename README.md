# Hosts [![Build Status](https://travis-ci.com/haydenhughes/hosts-rs.svg?branch=master)](https://travis-ci.com/haydenhughes/hosts-rs)

Rust [hosts](https://en.wikipedia.org/wiki/Hosts_(file)) parsing.

## Example
```rust
use hosts::{Host, HostsFile};

// Create a host
let host = Host::new("127.0.0.1", "localhost", None);

// Create a hosts file
let hosts_file = HostsFile::from(vec!(host));

// Write the hosts file
hosts_file.write("hosts");
```
