extern crate hosts;

use hosts::{parse, Host};

#[test]
fn test_parser() {
    assert_eq!(parse("tests/hosts").len(), 3);
    assert_eq!(
        parse("tests/hosts")[0],
        Host::new("127.0.0.1", "localhost", None)
    );
    assert_eq!(
        parse("tests/hosts")[2],
        Host::new(
            "127.0.1.1",
            "test.localdomain",
            Some(vec!("test".to_string()))
        )
    );
}
