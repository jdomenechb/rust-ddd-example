use domain::repositories::ClientRepository;
use domain::entities::Client;
use std::collections::HashMap;

pub struct InMemoryClientRepository {
    clients : HashMap<String, Client>
}

impl InMemoryClientRepository {
    pub fn new() -> InMemoryClientRepository {
        let mut clients : HashMap<String, Client> = HashMap::new();

        clients.insert(String::from("ID0001"), Client::new(String::from("Client number 1")));
        clients.insert(String::from("ID0002"), Client::new(String::from("Client number 2")));

        return InMemoryClientRepository {
            clients
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: &str) -> Option<&Client> {
        let id_string = String::from(id);

        return self.clients.get(&id_string);
    }
}