mod front_of_house;
// extract to sperated file
// {
    
    // pub mod hosting {
    //     pub fn add_to_waitlist() {}

    //     fn seat_at_table() {}
    // }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
// }

mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting; // Re-exporting
// external can use restaurant::hosting::add_to_waitlist() 
// instead of restaurant::front_of_house::hosting::add_to_waitlist()

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        // The next line won't compile if we uncomment it; use statement bring 
        // hosting in outer scope not in customer scope
        // hosting::add_to_waitlist();   -> super::hosting::add_to_waitlist();
        super::hosting::add_to_waitlist(); // this line work hence we use super to link with outer use statement
    }
}


use std::fmt::Result;
// use std::io::Result; // Rust not allow to bring the same name in same scope
use std::io::Result as IoResult;


// using nested path
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};


// use std::fmt;
// use std::fmt::format;
use std::fmt::{self, format};

// (*) : glob operators
use std::collections::*;


