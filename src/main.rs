extern crate proc_macro;

use infrastructure::domain::repositories::InMemoryClientRepository;
use application::get_client_use_case::{Handler, Request};

mod domain;
mod application;
mod infrastructure;

fn main() {
    let client_repository : InMemoryClientRepository = InMemoryClientRepository::new();
    let get_client_use_case : Handler = Handler::new(&client_repository);

    let get_client_use_case_req : Request = Request::new("ID0002");

    get_client_use_case.execute(get_client_use_case_req);
}
