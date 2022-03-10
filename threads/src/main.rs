use std::thread;

fn main() {
    // threads();

    message_passing();
}

fn threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn message_passing() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
}
