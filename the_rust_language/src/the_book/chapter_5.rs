#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structs() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    //? only if they are mutable

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user("dsadas".to_string(), "Hudi".to_string());

    println!("build user  =  {:?}", user2);

    {
        fn build_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 1,
            }
        }
    }

    println!(" --- using the shorthand same as in JS");

    println!(" --- update syntax same sa in JS");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let user2 = User {
    //     // active: user1.active,
    //     //? this is a move and it can not be used later
    //     //? the value itself is not here anymore
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("you can use Tuples Structs to create types without names");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //? unit like struct

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // {
    //     struct User {
    //         active: bool,
    //         username: &str,
    //         email: &str,
    //         sign_in_count: u64,
    //     }

    //     let user1 = User {
    //         email: "someone@example.com",
    //         username: "someusername123",
    //         active: true,
    //         sign_in_count: 1,
    //     };
    // }

    //? this code needs lifetimes
}

pub fn program_with_structs() {
    {
        let width1 = 30;
        let height1 = 50;

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        println!("rct = {:#?}", rect1);

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let scale = 2;
        let rect1 = Rectangle {
            // !  dbg!() return ownership
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}

pub fn methods() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        //? self can be borrowed like any other variable, mutably or immutably
        fn area(&self) -> u32 {
            return self.width * self.height;
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("there is not -> operator like in C");
    println!("p1.distance(&p2);");
    println!("(&p1).distance(&p2);");
    println!("these are equivalent, because rust automatic referencing and dereferencing ");

    println!();
    println!("  --   More parameters!");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // okay you can redeclare implementations to a struct

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let square = Rectangle::square(3);

    println!("square = {:#?}", square);


    // ? do multiple implementation blocks mean that you can extend 
    // ? functionality of the types within libraries?


}
