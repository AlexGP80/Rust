// If we use pub before a struct definition, we make the struct public, but the struct’s
// fields will still be private.
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

// In contrast, if we make an enum public, all of its variants are then public. We only need
// the pub before the enum keyword
pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {
    println!("cook_order()");
}
