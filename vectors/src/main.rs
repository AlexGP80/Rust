#![allow(unused)]
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100]; // This line causes de program to panic
    let does_not_exist = v.get(100); // This line is fine. does_not_exist takes the None value.

    // v.push(6); // Won't compile because we already have a immutable reference (third)
    println!("The third element is still {}", third);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
    for i in &v2 {
        println!("{}", i);
    }
}
