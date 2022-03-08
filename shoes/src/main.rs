fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4, 5];

    for (a, b) in v1.iter().zip(v2.iter().skip(1)) {
        println!("{} * {} = {} % 3 == 0?: {}", a, b, a * b, (a * b) % 3 == 0);
    }
}
