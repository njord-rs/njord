use crate::sqlite;

#[test]
fn test_function1() {
    // Call function1 from the sqlite module and verify its output
    assert_eq!(sqlite::function1(), expected_value);
}

#[test]
fn test_function2() {
    // Call function2 from the sqlite module and verify its output
    assert_eq!(sqlite::function2(), expected_value);
}