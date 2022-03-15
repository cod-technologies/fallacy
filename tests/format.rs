//! format test case

use fallacy::try_format;

#[test]
fn test_try_format() {
    assert_eq!(try_format!("{}", 123).unwrap(), "123");
    assert_eq!(try_format!("{}", "abc").unwrap(), "abc");
}
