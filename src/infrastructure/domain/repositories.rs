use crate::domain::entities::Client;
use crate::domain::repositories::ClientRepository;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct InMemoryClientRepository {
    clients: RefCell<HashMap<String, Client>>,
}

impl InMemoryClientRepository {
    pub fn new() -> Self {
        let clients: HashMap<String, Client> = HashMap::new();

        Self {
            clients: RefCell::new(clients),
        }
    }

    pub fn new_with_samples() -> Self {
        let repository = Self::new();

        let client1 = Client::new("1", "Zack", "London");
        let client2 = Client::new("2", "Manuel", "Madrid");

        repository.save(client1);
        repository.save(client2);

        repository
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: String) -> Result<Client, String> {
        let client = self.clients.borrow().get(&id).cloned();

        match client {
            Some(c) => Ok(c),
            None => Err(String::from("No client found for given ID")),
        }
    }

    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id.clone(), client);
    }

    fn next_identity(&self) -> String {
        let size = self.clients.borrow().len() + 1;

        size.to_string()
    }

    fn all(&self) -> Vec<Client> {
        let mut result = Vec::new();

        for value in self.clients.borrow().values() {
            result.push(value.clone())
        }

        result
    }
}
