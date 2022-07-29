extern crate core;
extern crate proc_macro;

use crate::presentation::prompt::{ask_question, read_input};
use application::handlers::{
    CreateClientUseCaseHandler, GetAllClientsUseCaseHandler, GetClientUseCaseHandler,
};
use application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use infrastructure::domain::repositories::InMemoryClientRepository;
use std::io;
use std::io::Write;
use std::rc::Rc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

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

fn main() {
    let client_repository = Rc::new(InMemoryClientRepository::new_with_samples());

    let get_all_clients_use_case_handler =
        GetAllClientsUseCaseHandler::new(client_repository.clone());
    let get_client_use_case_handler = GetClientUseCaseHandler::new(client_repository.clone());
    let create_client_use_case_handler = CreateClientUseCaseHandler::new(client_repository);

    while {
        let option: u8 = menu();

        match option {
            1 => {
                let clients = get_all_clients_use_case_handler.execute();

                print!("{}", clients);
            }

            2 => {
                let client_id =
                    ask_question("Please, enter the ID of the client that you want to read:");

                println!();

                let get_client_use_case_req = GetClientUseCaseRequest::new(client_id.trim());

                let client = get_client_use_case_handler.execute(get_client_use_case_req);

                match client {
                    Ok(c) => println!("{}", c),
                    Err(e) => eprintln!("ERROR: {}", e),
                }
            }

            3 => {
                let client_name =
                    ask_question("Please, enter the name of the client that you want to create:");
                let client_location =
                    ask_question("\nEnter the location of the client that you want to create:");

                let create_client_use_case_req =
                    CreateClientUseCaseRequest::new(client_name.trim(), client_location.trim());

                create_client_use_case_handler.execute(create_client_use_case_req);

                println!("\nClient created!");
            }

            0 => println!("Exiting..."),
            _ => println!("Invalid option"),
        };

        println!();

        option != 0
    } {}
}
