extern crate hosts;

use hosts::{read, Host};

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
