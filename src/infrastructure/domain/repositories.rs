use std::collections::HashMap;
use std::cell::RefCell;
use crate::domain::entities::Client;
use crate::domain::repositories::ClientRepository;

pub struct InMemoryClientRepository {
    clients : RefCell<HashMap<String, Client>>
}

impl InMemoryClientRepository {
    pub fn new() -> InMemoryClientRepository {
        let mut clients : HashMap<String, Client> = HashMap::new();

        let client1 = Client::new("1".to_string(), "Client number 1".to_string());
        let client2 = Client::new("2".to_string(), "Client number 2".to_string());

        clients.insert(client1.id().clone(), client1);
        clients.insert(client2.id().clone(), client2 );

        return InMemoryClientRepository {
            clients: RefCell::new(clients)
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: String) -> Result<Client, String> {
        let id_string = id.to_string();

        let client = self.clients.borrow().get(&id_string).cloned();

        match client {
            Some(c) => Ok(c),
            None => Err(String::from("No client found for given ID"))
        }
    }

    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id().clone(), client);
    }

    fn next_identity(&self) -> String {
        let size = self.clients.borrow().len() + 1;

        String::from(size.to_string())
    }

    fn all(&self) -> Vec<Client> {
        let mut result = Vec::new();

        for value in self.clients.borrow().values() {
            result.push(value.clone())
        }

        result
    }
}