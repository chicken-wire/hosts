extern crate hosts;

use hosts::{read, write, Host};
use std::fs;

#[test]
fn test_read() {
    assert_eq!(read("tests/hosts").len(), 3);
    assert_eq!(
        read("tests/hosts")[0],
        Host::new("127.0.0.1", "localhost", None)
    );
    assert_eq!(
        read("tests/hosts")[2],
        Host::new(
            "127.0.1.1",
            "test.localdomain",
            Some(vec!("test".to_string()))
        )
    );
}

#[test]
fn test_write() {
    write("tests/test_hosts", read("tests/hosts"));

    let hosts = read("tests/hosts");
    for (i, host) in hosts.iter().enumerate() {
        assert_eq!(&read("tests/test_hosts")[i], host)
    }

    fs::remove_file("tests/test_hosts").unwrap();
}
