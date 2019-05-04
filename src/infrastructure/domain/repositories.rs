use domain::repositories::ClientRepository;
use domain::entities::Client;
use std::collections::HashMap;

pub struct InMemoryClientRepository {
    clients : HashMap<String, Client>
}

impl InMemoryClientRepository {
    pub fn new() -> InMemoryClientRepository {
        let mut clients : HashMap<String, Client> = HashMap::new();

        let client1 = Client::new("1".to_string(), "Client number 1".to_string());
        let client2 = Client::new("2".to_string(), "Client number 2".to_string());

        clients.insert(client1.id().clone(), client1);
        clients.insert(client2.id().clone(), client2 );

        return InMemoryClientRepository {
            clients: clients
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: String) -> Result<Client, String> {
        let id_string = String::from(id);

        let client = self.clients.get(&id_string).cloned();

        match client {
            Some(c) => Ok(c),
            None => Err(String::from("No client found for given ID"))
        }
    }

    fn save(&self, client: Client) {
        //self.clients.borrow_mut().insert(client.id().clone(), client);
    }

    fn next_identity(&self) -> String {
        let size = self.clients.len();

        String::from(size.to_string())
    }
}