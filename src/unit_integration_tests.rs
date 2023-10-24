// integration tests:
// In Rust, integration tests are entirely external to your library. 
// They use your library in the same way any other code would, which 
// means they can only call functions that are part of your library’s public API. 
// Their purpose is to test whether many parts of your library work together correctly.
// https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests

#[test]
fn test_add_one() {
    assert_eq!(2, crate::unit_test::add_one(1));
}
