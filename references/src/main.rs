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

// // 4.2. Non-Lexical Lifetimes (NLL)
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//
//     println!("{}", r3);
// }

// // 4.2 Dangling References
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String { // dangle returns a refrence to a String
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// 4.2 Dangling References Solution
fn main() {
    let str = no_dangle();
    println!("{}", str);
}

fn no_dangle() -> String {
    let s = String::from("no dangling!");

    s
}
