use std::io;

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
