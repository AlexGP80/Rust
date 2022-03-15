fn main() {
    fuckit(1);
}

fn fuckit(n: u32) {
    println!("{}", n);
    let n = n + 1;
    fuckit(n);
}
