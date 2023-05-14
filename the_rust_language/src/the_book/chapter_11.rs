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
    /// Controlling How The Tests are Run
    /// cargo compiles them in test mode and runs the binary
    
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




}
