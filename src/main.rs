extern crate proc_macro;
extern crate core;

use infrastructure::domain::repositories::InMemoryClientRepository;
use std::io;
use std::io::Write;
use application::handlers::{GetClientUseCaseHandler, CreateClientUseCaseHandler, GetAllClientsUseCaseHandler};
use application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};

mod domain;
mod application;
mod infrastructure;

fn menu() -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Please, select an option:");
    println!("\t 1. List all clients");
    println!("\t 2. Read a client");
    println!("\t 3. Create a client");
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

    let get_all_clients_use_case_handler = GetAllClientsUseCaseHandler::new(&client_repository);
    let get_client_use_case_handler = GetClientUseCaseHandler::new(&client_repository);
    let create_client_use_case_handler = CreateClientUseCaseHandler::new(&client_repository);

    while {
        let option :u8 = menu();

        match option {
            1 => {
                println!();

                let clients=  get_all_clients_use_case_handler.execute();

                if clients.is_empty() {
                    println!("No clients found");
                } else {
                    println!("Client list");
                    println!("----------------------------------------\n");

                    for client in clients {
                        println!("{:#X?}", client);
                    }
                }
            }

            2 => {
                println!("\nPlease, enter the ID of the client that you want to read:");

                let mut client_id :String = String::new();

                io::stdin().read_line(&mut client_id)
                    .expect("Failed to read line");

                let get_client_use_case_req = GetClientUseCaseRequest::new(client_id.trim());

                let client = get_client_use_case_handler.execute(get_client_use_case_req);

                match client {
                    Ok(c) => println!("{:#X?}", c),
                    Err(e) => eprintln!("ERROR: {}", e)
                }
            }

            3 => {
                println!("\nPlease, enter the name of the client that you want to create:");

                let mut client_name :String = String::new();

                io::stdin().read_line(&mut client_name)
                    .expect("Failed to read line");

                let create_client_use_case_req  = CreateClientUseCaseRequest::new(String::from(client_name.trim()));

                create_client_use_case_handler.execute(create_client_use_case_req);
            }

            0 => println!("Exiting..."),
            _ => println!("Invalid option")
        };

        option != 0
    } {}
}
