use crate::garden::vegetables::Asparagus;

pub fn defining_modules_to_control_scope() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // Works if defined in root crate
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod back_of_house1 {
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
}

pub fn eat_at_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod front_of_house_path_scope {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house_path_scope::hosting;

pub fn eat_at_restaurant_path_scope() {
    hosting::add_to_waitlist();
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::{cmp::Ordering, io};

// use std::io;


// use std::io::Write;
// // equivalent to:
// use std::io::{self, Write};


// the glob operator * imports all names from a module
// use std::collections::*;
