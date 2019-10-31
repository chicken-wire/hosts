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
            // FIXME: don't repeat yourself!
            match &values[2..] {
                [] => None,
                _ => Some(Vec::from(&values[2..])),
            },
        ))
    }
    return hosts;
}
