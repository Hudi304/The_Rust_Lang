// see the adder crate in this repository
pub fn tests() {
    //? set up needed data or state
    //? run the code you want to test
    //? assert the results that you expect

    // a lot of types of tests

    // you can filter them
    // you can run specifically some tests
    // there is an unstable feature pfa benchmark test
    // there is also doc testing (keeping the docs and the code in sync)

    // tests fail when they panic
    // each test is performed in a different thread

    // assert! takes a boolean parameter
    // if the expression return true, nothing happens
    // if it returns false, it calls panic!()

    // assert_eq! param order does not matter

    ////////////////////////////////
    // Controlling How The Tests are Run
    // cargo compiles them in test mode and runs the binary

    // the default behavior is to run all tests in parallel
    // you can override this behavior with command line options

    // because the  test are run in parallel
    // you make sure that they do not depend on each other

    // you can pick how many threads you want to allocate
    // to run the tests with --test-threads
    //? cargo test -- --test-threads=1

    ////////////////////////////////
    //  Showing function output
    //  by default you can not see any !print()
    // you can override the output with :
    //? $ cargo test -- --show-output

    ////////////////////////////////
    // Running a subset of tests by name
    //? cargo test exploration
    //?  this does not need to be the full name
    //? you can write part of the name and cargo will fuzzy find

    ////////////////////////////////
    // Ignoring tests unless specifically requested
    // to ignore use :
    //?  #[ignore]
    // to run the ignored tests use:
    //? cargo run -- --ignored // just ignored
    //? cargo run -- --include-ignored // all

    ////////////////////////////////
    // Test organization

    //? unit tests
    //? integration tests -  Integration tests
    // are entirely external to your library and
    // use your code in the same way any other
    // external code would, using only the public
    // interface and potentially exercising multiple
    // modules per test.

    // Testing private functions
    // Rust does not care :))))

    ////////////////////////////////
    // Integration tests

    // these are entirely external to your library
    // and use your code in the same way any other
    // external code would

    // integration tests sit in the test directory top level
    // (./src)

    // each file in the test directory is a separate crate
    // no need for [cfg(test)]
    // cargo treats the /src/test directory specially
    // the files in the /test directory are compiled
    // ony when cargo test is run

    ////////////////////////////////
    //? unit tests
    //? integration tests
    //? documentation tests

    // the tests cascade
    // if one section fails the others will not be run

    ////////////////////////////////
    // Running integration tests individually
    //? cargo test --test integration_test

    ////////////////////////////////
    // Binary crates
    // if the project does not have a src/lib.rs file 
    // we can not create integration tests

}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    // this brings all of the test module's parents into scope

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
