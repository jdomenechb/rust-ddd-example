use domain::repositories::ClientRepository;

#[derive(Clone, Debug)]
pub struct Client {
    pub id: String,
    pub name: String,
}

impl Client {
    pub fn new(id: &str, name: &str) -> Client {
        Client {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct ClientAggregate{
    client_repository: Box<dyn ClientRepository> 
}

impl ClientAggregate{
    pub fn new(client_repository: impl ClientRepository + 'static) -> ClientAggregate {
        ClientAggregate {
            client_repository: Box::new(client_repository)
        }
    }

    pub fn create_client(&self, name: &str) {
        let id = self.client_repository.next_identity();
        let client = Client::new(&id, name);

        self.client_repository.save(client);
    }

    pub fn get_by_id(&self, client_id: &str) -> Result<Client, String>{
        return self.client_repository.by_id(client_id);
    }

    pub fn all_clients(&self) -> Vec<Client> {
        return self.client_repository.all();
    }
}
