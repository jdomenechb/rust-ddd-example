extern crate proc_macro;
extern crate core;

use infrastructure::domain::repositories::InMemoryClientRepository;
use application::get_client_use_case;
use application::create_client_use_case;
use std::io;
use std::io::Write;

mod domain;
mod application;
mod infrastructure;

fn menu() -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Please, select an option:");
    println!("\t 1. Read a client");
    println!("\t 2. Create a client");
    println!("\t 0. Exit");
    print!("\nOption: ");
    io::stdout().flush().expect("Error flushing");

    let mut option :String = String::new();

    io::stdin().read_line(&mut option)
        .expect("Failed to read line");

    let option = option.trim().parse();

    match option {
        Ok(o) => return o,
        Err(_) => {
            eprintln!("ERROR: Please, type a number");
            99
        }
    }
}

fn main() {
    let client_repository : InMemoryClientRepository = InMemoryClientRepository::new();
    let get_client_use_case_handler = get_client_use_case::Handler::new(&client_repository);
    let create_client_use_case_handler = create_client_use_case::Handler::new(&client_repository);

    while {
        let option :u8 = menu();

        match option {
            1 => {
                println!("\nPlease, enter the ID of the client that you want to read:");

                let mut client_id :String = String::new();

                io::stdin().read_line(&mut client_id)
                    .expect("Failed to read line");

                let get_client_use_case_req = get_client_use_case::Request::new(client_id.trim());

                let client = get_client_use_case_handler.execute(get_client_use_case_req);

                match client {
                    Ok(c) => println!("{:#X?}", c),
                    Err(e) => eprintln!("ERROR: {}", e)
                }
            }

            2 => {
                println!("\nPlease, enter the name of the client that you want to create:");

                let mut client_name :String = String::new();

                io::stdin().read_line(&mut client_name)
                    .expect("Failed to read line");

                let create_client_use_case_req  = create_client_use_case::Request::new(String::from(client_name.trim()));

                create_client_use_case_handler.execute(create_client_use_case_req);
            }

            0 => println!("Exiting..."),
            _ => println!("Invalid option")
        };

        option != 0
    } {}
}
