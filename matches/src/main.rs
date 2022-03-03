#![allow(unused)]
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let my_coin = Coin::Penny;
    println!("{}", value_in_cents(&my_coin));
    let my_coin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(&my_coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let another_six = plus_one(five);
    let str_five = Some(String::from("five"));
    let str_none: Option<String> = None;
    display(str_five);
    display(str_none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn display(x: Option<String>) {
    match x {
        None => println!("None"),
        Some(str) => println!("{}", str),
    }
}
