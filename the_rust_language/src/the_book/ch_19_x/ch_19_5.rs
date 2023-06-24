fn marcos_vs_functions() {
    // macro = code that write other code
    // a.k.a. metaprogamming

    // the derive attribute generates implementations for you
    //? COOL

    // the println! macro and the vec! macro expand to produce more code

    // metaprogamming is useful because it is reducing the code that you need
    // to write and maintain
    // kinda like function but different

    // functions in rust can take a defined number of arguments
    //  macros can take any number of arguments
    // they also get called at compile time
    // functions get called at runtime
}

fn declarative_marcos() {
    let v: Vec<u32> = vec![1, 2, 3];

    #[macro_export]
    // this makes the marco available whenever the crate is imported
    // pub for macros if you will
    macro_rules! vec {
        // the body of the macro starts here
    ( $( $x:expr ),* ) => { // this is similar to a match expression with one arm
        // if the pattern matched then the code will be emitted
        // $( $x:expr ) reads as match any rust expression and name it x
        // ',' means that there might actually be a ','
        // '*' match everything else
        // in vec![1,2,3] ,  x= 1 | 2 | 3
        // it matches 3 times

        //?  this looks a lot like lists in lips
        {
            let mut temp_vec = Vec::new();
            $( // this code is generated every time the pattern matches
                temp_vec.push($x);
            )*
            return temp_vec
        }

        // the expansion of this macro is :
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        // temp_vec
        // there is a book for this as well
        // THE LITTLE BOOK OF RUST MACROS
        // https://veykril.github.io/tlborm/

    };
    }
}

fn procedural_macros() {
    // more like a function
    // it is a type of procedure

    // they accept some code as input
    // operate on it
    // and produce some code as output

    // custom derive
    // attribute-like
    // function-like

    //  these macros must reside in their own crate
    // with a special crate type

    // use proc_macro;

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {}

    // ! when you define a crate you need to add it to Cargo.toml
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    // normal trait implementation
    // impl HelloMacro for Pancakes {
    //     fn hello_macro() {
    //         println!("Hello, Macro! My name is Pancakes!");
    //     }
    // }
    // Rust DOES NOT have REFLECTION at all

    //? at the moment the derive macro for a crate must be in a separate crate
    // for crate foo th derive crate should be named foo_derive

    // normally the derive crate should be a dependency
    // of the crate that exports the trait
    // a.k.a. they should be in the same folder
    // and the normal care should reexport the derive crate

    Pancakes::hello_macro();
}

fn attribute_like_macros() {
    // #[route(GET, "/")]
    // fn index() {}

    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
}

fn function_like_macros() {
    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    //     #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {}
}

pub fn main() {
    // MACROS

    // there are more then one kind of macro

    // declarative macro
    // 3 kinds of procedural macros
    //     1. Custom #[derive] macros that specify code
    //        added with the derive attribute on structs and enums
    //     2. Attribute like marcos that define a custom attribute
    //        usable on any item
    //     3. Function like marcos that look like function calls
    //        but operate on the tokens specified as their argument

    println!("Ch 19.5");
    marcos_vs_functions();
    declarative_marcos();
    procedural_macros();
}
