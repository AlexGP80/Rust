#![allow(unused)]
use std::collections::HashMap;
use std::io;
use std::io::Write; // bring flush() into scope

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    println!("ACME Corp. Employee Directory");

    let mut user_input = String::new();
    let mut employees: HashMap<String, String> = HashMap::new();

    loop {
        // Command prompt
        print!("\n> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        trim_newline(&mut user_input);

        let mut uppercase_user_input = &user_input.to_uppercase();

        match &uppercase_user_input[..] {
            "EXIT" => break,
            "LIST ALL" => {
                println!("EMPLOYEES:");
                println!("==========");
                for (employee, department) in &employees {
                    println!("{} - {}", employee, department);
                }
            }
            other => {
                let splitted_command: Vec<&str> = user_input.split(" ").collect();
                if splitted_command.len() == 4
                    && &splitted_command[0].to_uppercase() == "ADD"
                    && &splitted_command[2].to_uppercase() == "TO"
                {
                    let employee = splitted_command[1];
                    let department = splitted_command[3];
                    println!("{} added to {} department", &employee, &department);
                    employees.insert(String::from(employee), String::from(department));
                } else if splitted_command.len() == 3
                    && &splitted_command[0].to_uppercase() == "LIST"
                    && &splitted_command[1].to_uppercase() == "FROM"
                {
                    println!("{}:", &splitted_command[2].to_uppercase());
                    let mut underscore = String::new();
                    for i in (0..splitted_command[2].len()+1) {
                        underscore.push('=');
                    }
                    println!("{}", underscore);
                    for (employee, department) in &employees {
                        if &department.to_uppercase() == &splitted_command[2].to_uppercase() {
                            println!("{}", employee);
                        }
                    }
                } else {
                    println!("Invalid command: '{}'", user_input);
                }
            }
        }

        user_input.clear();
    }
}
