extern crate hosts;

use hosts::{HostsFile};
use std::fs;

#[test]
fn test_from_file() {
    assert_eq!(HostsFile::from_file("tests/hosts").hosts.len(), 3);
}

#[test]
fn test_save() {
    let hosts_file = HostsFile::from_file("tests/hosts");
    hosts_file.write("tests/test_hosts");

    let hosts_new = HostsFile::from_file("tests/test_hosts").hosts;
    let hosts_old = HostsFile::from_file("tests/hosts").hosts;
    for (i, host) in hosts_old.iter().enumerate() {
        assert_eq!(host, &hosts_new[i])
    }
    fs::remove_file("tests/test_hosts").unwrap();
}
