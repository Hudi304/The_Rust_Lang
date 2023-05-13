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
}
