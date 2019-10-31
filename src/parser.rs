use crate::host::Host;
use std::fs;

/// Parses a hosts file.
///
/// # Example
/// ```
/// // tests/hosts:
/// // 127.0.0.1	localhost
/// // ::1		localhost
/// // 127.0.1.1	test.localdomain test
///
/// assert_eq!(hosts::read("tests/hosts").len(), 3)
/// ```
pub fn read(path: &str) -> Vec<Host> {
    let file = fs::read_to_string(path).expect("File not found");
    let mut hosts: Vec<Host> = vec![];

    for line in file.lines() {
        let values: Vec<String> = line.split_whitespace().map(|v| v.to_string()).collect();
        hosts.push(Host::new(
            &values[0],
            &values[1],
            // Check to see if any aliases exist then handle appropriately.
            match &values[2..] {
                [] => None,
                _ => Some(Vec::from(&values[2..])),
            },
        ))
    }
    return hosts;
}

/// Writes hosts to a hosts file.
///
/// # Example
/// ```
/// let hosts  = vec!(hosts::Host::new("127.0.0.1", "localhost", None));
/// hosts::write("tests/test_hosts", hosts);
///
/// assert_eq!(hosts::read("tests/test_hosts")[0], hosts::Host::new("127.0.0.1", "localhost", None));
///
/// std::fs::remove_file("tests/test_hosts");
/// ```
pub fn write(path: &str, hosts: Vec<Host>) {
    let mut hosts_file = String::new();

    for host in hosts {
        hosts_file.push_str(&format!("{}\n", host))
    }

    fs::write(path, hosts_file).expect("Invalid path");
}
