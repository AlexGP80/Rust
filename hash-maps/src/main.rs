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

    // Accessing Values in a Hash Map
    let team_name = String::from("Real Madrid");
    if let Some(score) = scores.get(&team_name) {
        println!("{}: {:?}", team_name, score);
    }

    // Iterating over each key/value pair in a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Phoenix Suns"), 128);
    scores.insert(String::from("Phoenix Suns"), 118);
    println!("{:?}", scores);
    // Only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Phoenix Suns"), 118);
    scores.entry(String::from("New York Knicks")).or_insert(97);
    scores.entry(String::from("Phoenix Suns")).or_insert(92);
    println!("{:?}", scores);
    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // Ojo ponzoña
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
