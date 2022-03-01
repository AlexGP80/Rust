// 4.2. Example of Mutable References
// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// 4.2. Mutable and immutable references cannot exist at the same time
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {} and {}", r1, r2, r3);
}
