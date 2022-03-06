#![allow(unused)]

fn longest(x: &str, y: &str) -> String {
    String::from("Mockup")
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
