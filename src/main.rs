extern crate core;
extern crate proc_macro;

use crate::presentation::prompt::{ask_question, menu};
use application::handlers::{
    CreateClientUseCaseHandler, GetAllClientsUseCaseHandler, GetClientUseCaseHandler,
};
use application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use infrastructure::domain::repositories::InMemoryClientRepository;
use std::rc::Rc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

fn main() {
    let client_repository = Rc::new(InMemoryClientRepository::new_with_samples());

    let get_all_clients_use_case_handler =
        GetAllClientsUseCaseHandler::new(client_repository.clone());
    let get_client_use_case_handler = GetClientUseCaseHandler::new(client_repository.clone());
    let create_client_use_case_handler = CreateClientUseCaseHandler::new(client_repository);

    while {
        let option: u8 = menu(vec!["List all clients", "Read a client", "Create a client"]);

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
