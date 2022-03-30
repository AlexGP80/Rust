

fn main() {
    playing_with_options();
    playing_with_options_2();
}




fn playing_with_options_2() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    type IntOption = Option<i32>;

    let mut int_opt_vec: Vec<IntOption> = Vec::new();

    for _ in 1..1000 {
        int_opt_vec.push(match rng.gen_range(0..2) {
            1 => Some(rng.gen_range(0..101)),
            _ => None,
        });
    }

    for i in 0..int_opt_vec.len() {
        if let Some(number) = int_opt_vec[i] {
            println!("{}", number)
        } else {
            println!("NADA");
        }
    }
}


fn playing_with_options() {
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

    type CellOption = Option<Cell>;

    use rand::Rng;

    let mut rng = rand::thread_rng();

    let mut str_vec: Vec<CellOption> = Vec::new();
    let vowels = [97, 101, 105, 111, 117];

    for _ in 1..1000 {
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

    for elem in str_vec {
        match elem {
            Some(cell) => {
                println!("{}", cell.value)
            },
            None => println!("CACA"),
        }
    }
}
