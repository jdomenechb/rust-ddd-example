use domain::repositories::ClientRepository;
use domain::clients::{ClientInfo};
use std::collections::HashMap;
use std::cell::RefCell;

pub struct InMemoryClientRepository {
    clients : RefCell<HashMap<String, ClientInfo>>
}

impl InMemoryClientRepository {
    pub fn new() -> InMemoryClientRepository {
        let mut clients : HashMap<String, ClientInfo> = HashMap::new();

        let client1 = ClientInfo::new("1", "Client number 1");
        let client2 = ClientInfo::new("2", "Client number 2");

        clients.insert(client1.id.clone(), client1);
        clients.insert(client2.id.clone(), client2 );

        return InMemoryClientRepository {
            clients: RefCell::new(clients)
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: &str) -> Result<ClientInfo, String> {
        let id_string = id.to_string();

        let client = self.clients.borrow().get(&id_string).cloned();

        match client {
            Some(c) => Ok(c),
            None => Err(String::from("No client found for given ID"))
        }
    }

    fn save(&self, client: ClientInfo) {
        self.clients.borrow_mut().insert(client.id.clone(), client);
    }

    fn next_identity(&self) -> String {
        let size = self.clients.borrow().len() + 1;

        String::from(size.to_string())
    }

    fn all(&self) -> Vec<ClientInfo> {
        let mut result = Vec::new();

        for value in self.clients.borrow().values() {
            result.push(value.clone())
        }

        result
    }
}