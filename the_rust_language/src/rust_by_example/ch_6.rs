pub fn from_into() {
    println!("From : ");

    let my_str = "hello";
    let my_string = String::from(my_str);

    {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let num = Number::from(30);
        println!("My number is {:?}", num);

        let int = 5;
        // Try removing the type declaration
        let num: Number = int.into();
        println!("My number is {:?}", num);

        // let str_array = ["dsa", "dsadsa"];

        // let string_array : [&String,2] = str_array.fr
    }
}



