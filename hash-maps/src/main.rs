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

    
}
