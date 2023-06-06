pub fn main() {
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
