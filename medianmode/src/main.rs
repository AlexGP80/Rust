#![allow(unused)]
use rand::Rng;
use std::collections::HashMap;

const NUMBER_OF_NUMBERS: u8 = 255;

fn main() {
    let mut numbers: Vec<u8> = Vec::new();
    for i in (0..NUMBER_OF_NUMBERS) {
        let number = rand::thread_rng().gen_range(0..101);
        numbers.push(number);
    }
    println!("{:?}", numbers);
    numbers.sort_unstable();

    let mut sum: u32 = 0;
    for number in &numbers {
        sum += (*number as u32);
    }
    let mean: f64 = (sum / (NUMBER_OF_NUMBERS as u32)).into();
    println!("The mean is: {}", mean);

    if let Some(median) = numbers.get(5) {
        println!("The median is: {}", median);
    }

    let mut map = HashMap::new();
    for number in &numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut max_val = 0;
    let mut max_key = 0;
    for (k, v) in map.iter() {
        if *v > max_val {
            max_key = **k;
            max_val = *v;
        }
    }
    let mode = max_key;
    println!("The mode is {}", mode);
}
