#![allow(unused)]
use rand::Rng;

const NUMBER_OF_NUMBERS: u8 = 10;

fn main() {
    let mut numbers: Vec<u8> = Vec::new();
    for i in (0..NUMBER_OF_NUMBERS) {
        let number = rand::thread_rng().gen_range(0..101);
        numbers.push(number);
    }
    println!("{:?}", numbers);
}
