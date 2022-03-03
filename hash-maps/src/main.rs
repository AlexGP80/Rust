#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Real Madrid"), 4);
    scores.insert(String::from("Barça"), 1);

    // Constructing a hash map using iterators and the collect method on a vector of tuples (k,v)
    // Qué ponzoña
    let teams = vec![String::from("Real Madrid"), String::from("Farsalona")];
    let initial_scores = vec![5, 0];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and see what compiler
    // error you get
    // println!("{}: {}", field_name, field_value);
}
