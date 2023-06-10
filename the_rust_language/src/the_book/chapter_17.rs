fn ch_17_1() {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    //? Encapsulation
    // the only public methods are add, remove and average
    // this ensures that the we can not change the list without
    // updating the average

    // this is encapsulation
    // because this is encapsulated we can easily change the data stricture
    // for example from a vec to hashSet

    //? Inheritance
    // an object can inherit elements from another object's definition
    // without defining them again
    //* RUST DOES NOT HAVE THIS */
    // there is no way to define

    // there is the default implementation feature though

    //? Polymorphism
    // use a child in the place of it's parent

    // Rust uses generics for this (bounded parametric polymorphism)
}

fn ch_17_2() {
    // trait objects are more like regular objects from OOP languages
    // in the sense that they combine data and behavior

    // they differ from OOP languages objects because they can not add data
    //? that's interesting... le data be data

    // their specific purpose is to allow abstraction over common behavior

    // a trait object points to :
    //      - an instance of the type implementing the (common) trait
    //      - a table used to look up they methods at runtime

    //? these trait objects ar created by specific some sort fo pointer
    //? & or a reference toa Box<T> + dyn keyword
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // aka any type that implements Draw
        pub components: Vec<Box<dyn Draw>>,
    }

    // this works differently from defining a generic struct with trait bounds
    // because it can only be substituted with one concrete type at a time

    // trait object allow multiple concrete types to be filled in at runtime
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // example for a generic struct
    {
        pub struct Screen<T: Draw> {
            pub components: Vec<T>,
        }
        // this restricts us to having a screen that can only draw buttons or inputs

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }

    //? IMPLEMENTING THE TRAIT

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // TODO: draw the button
        }
    }
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // TODO: draw the select box
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    screen.run();

    // in the case of generic types the compiler generates non generic implementation
    // of functions for each each concrete type
    // the code that results from this is called static dispatch
    // this is opposed to dynamic dispatch, we'll figure out at runtime witch method to call

    // so the dynamic dispatch has a runtime cost
    // but because it makes the code much more flexible it is a cost to consider
}

fn ch_17_3_oop_way() {
    //? OOP DESIGN PATTERNS

    //* THE STATE PATTERN */
    //? The final functionality will look like this:
    //      A blog post starts as an empty draft.
    //      When the draft is done, a review of the post is requested.
    //      When the post is approved, it gets published.
    //      Only published blog posts return content to print,
    //          so unapproved posts can’t accidentally be published.

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // we take a post as a parameter and return part of it
        // so their lifetimes are entangled
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    // the rules live in the state object instead of being scattered throughout Post

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn ch_17_3_the_rusty_way() {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        // it takes ownership of the Post they are called on and consume it
        // this way we don't have any lingering DraftPost
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        // it takes ownership of the Post they are called on and consume it
        // this way we don't have any lingering PendingReviewPost
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    
    // this does not quite follow the oop design pattern because the transformations 
    // between states are no longer encapsulated

    // the big + is that invalid states are now impossible
    // this ensures that the display of an unapproved post will be discovered before production
     
   
}

pub fn main() {
    // ch_17_1();
    // ch_17_2();
    ch_17_3_oop_way();
}
