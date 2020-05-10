// add test files to the tests directory to run integration tests (not that the below test is an integration test)
// each file in the tests directory is compiled as a separate crate
// we can't call functions from the binary because only libraries expose functions like that
// most rust projects have a minimal src/main.rs that calls functions from src/lib.rs 
#[test]
fn do_something() {
    assert_eq!(5-2, 3);
}
