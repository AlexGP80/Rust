

fn main() {
    playing_with_options();
}

#[derive(Debug)]
struct Cell {
    value: String,
}

impl Cell {
    fn new(value: String) -> Cell {
        Cell {
            value,
        }
    }
}

fn playing_with_options() {
    type CellOption = Option<Cell>;

    use rand::Rng;

    let mut rng = rand::thread_rng();

    let mut str_vec: Vec<CellOption> = Vec::new();
    let vowels = [97, 101, 105, 111, 117];

    for _ in 1..10 {
        let mut buffer: [u8; 5] = [0, 0, 0, 0, 0];

        for i in 0..buffer.len() {
            // 97..122
            buffer[i] = rng.gen_range(97..123);
        }

        if vowels.contains(&buffer[0]) {
            str_vec.push(None);
        } else {
            let buffer = std::str::from_utf8(&buffer).unwrap();
            str_vec.push(Some(Cell::new(buffer.to_string())));
        }

    }

    println!("{:?}", str_vec);
}
