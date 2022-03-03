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

    //Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let dice_roll = 4;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // if we want that nothing happened if a value other than 3 or 7 is rolled, it would be
    // like this
    let dice_roll = 2;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    if_let_vs_match();
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

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

// if let vs match
fn if_let_vs_match() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
