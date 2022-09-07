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

use std::convert::TryFrom;
use std::convert::TryInto;

pub fn try_from_try_into() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

pub fn string_parsing() {
    println!("Turbo fish");
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
