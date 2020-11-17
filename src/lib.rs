mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub bread: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(bread: String) -> Breakfast {
            Breakfast {
                bread,
                seasonal_fruit: String::from("Blueberries"),
            }
        }
    }
    #[derive(Debug)]
    pub struct Appetizer {
        pub variety: String,
    }
}

fn eat_at_restaurant() {
    let order = back_of_house::Breakfast::summer(String::from("wheat"));
    println!("Result is {:?}", order);
    impl back_of_house::Appetizer {
        fn name(&self) {
            println!("I am {}", self.variety);
        }
    }
    let appetizer = back_of_house::Appetizer {
        variety: String::from("Vegetarian"),
    };
    appetizer.name();
}
