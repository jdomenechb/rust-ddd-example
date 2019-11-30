extern crate proc_macro;
extern crate core;

use std::io;
use std::io::Write;
use application::handlers::ClientHandler;

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
    let client_handler = ClientHandler::new();

    while {
        let option :u8 = menu();

        match option {
            1 => {
                println!();

                let clients=  client_handler.all_clients() ;

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


                let client = client_handler.get_by_id(client_id.trim());

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

                client_handler.create_client(client_name.trim());
            }

            0 => println!("Exiting..."),
            _ => println!("Invalid option")
        };

        option != 0
    } {}
}
