use domain::repositories::ClientRepository;
use domain::entities::Client;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct InMemoryClientRepository {
    clients : RefCell<HashMap<String, Client>>
}

impl InMemoryClientRepository {
    pub fn new() -> InMemoryClientRepository {
        let mut clients : HashMap<String, Client> = HashMap::new();

        let client1 = Client::new(String::from("1"), String::from("Client number 1"));
        let client2 = Client::new(String::from("2"), String::from("Client number 2"));

        clients.insert(client1.id(), client1);
        clients.insert(client2.id(), client2 );

        return InMemoryClientRepository {
            clients: RefCell::new(clients)
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: &str) -> Result<&Client, String> {
        let id_string = String::from(id);

        let client = self.clients.borrow().get(&id_string);

        match client {
            Some(c) => Ok(c),
            None => Err(String::from("No client found for given ID"))
        }
    }

    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id(), client);
    }

    fn next_identity(&self) -> String {
        let size = self.clients.borrow().len();

        String::from(size.to_string())
    }
}