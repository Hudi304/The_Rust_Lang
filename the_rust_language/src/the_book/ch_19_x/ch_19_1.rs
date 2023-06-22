use std::slice;

fn dereferencing_a_raw_pointer() {
    // there are 2 more pointer types in Rust
    // Raw pointers
    // a.k.a. *const T and *mut T

    // * is not a the dereference operator it's part of the type name

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // you can create raw pointers in safe code
    // you just can't dereference it

    let address = 0x012345usize;
    let r = address as *const i32;

    // *  the dereference operator

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // one is immutable
        // the other one is mutable
        // this would not be possible
        // in safe rust

        // this means that you can create a data race

        // especially useful when interfacing with C
    }
}

pub fn main() {
    fn calling_an_unsafe_function() {
        // the unsafe ward means that the Function
        // has some requirements and that we read the
        // documentation

        unsafe fn dangerous() {
            println!("DANGEROUS")
        }

        unsafe {
            dangerous();
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        // this can be called from safe rust
        // it is not an unsafe function
        // it's a function that has a unsafe block
        // so it's a safe abstraction over unsafe code
        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();
            // if we pass an index that is greater than the length
            // the function will panic
            assert!(mid <= len);
            // quite an elegant way :))

            // we are basically borrowing twice from the same slice
            // this is ok as long as the slices do not overlap
            // (&mut values[..mid], &mut values[mid..])

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
    }

    extern "C" {
        fn abs(input: i32) -> i32;
        // ok this is cool
    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // this is accessible from C LOL this is cool
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // THIS_IS_CALLED_SCREAMING_SNAKE_CASE LOL :))
    static HELLO_WORLD: &str = "Hello, world!";

    // they have a static lifetime
    // they are fixed values in memory
    // (constants are allowed to be duplicated)

    fn main2() {
        println!("name is: {}", HELLO_WORLD);
    }

    // changing these values is unsafe

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    fn main3() {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    fn implementing_an_unsafe_trait() {
        unsafe trait Foo {
            // methods go here
        }
        // the implementation is unsafe as well
        unsafe impl Foo for i32 {
            // method implementations go here
        }

        // you can do this if you want to implement Send or Sync
        // on a type that contains a type that is not
        // Send or Sync
        // this is unsafe
    }

    // accessing fields of an union is also unsafe

    calling_an_unsafe_function();
    main();
    main2();
    main3();
}
