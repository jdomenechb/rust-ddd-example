use std::io;
use std::io::Write;

pub fn read_input() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

pub fn ask_question(question: &str) -> String {
    println!("{}", question);

    read_input()
}

pub fn menu(options: Vec<&str>) -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Please, select an option:");

    let mut i = 1;

    for option in options {
        println!("\t {}. {}", i, option);
        i += 1;
    }

    println!("\t 0. Exit");
    print!("\nOption: ");

    io::stdout().flush().expect("Error flushing");

    let option = read_input().trim().parse();

    println!("\n");

    match option {
        Ok(o) => o,
        Err(_) => {
            eprintln!("ERROR: Please, type a number");
            u8::MAX
        }
    }
}
