use domain::repositories::ClientRepository;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub id: String,
    pub name: String,
}

impl ClientInfo {
    pub fn new(id: &str, name: &str) -> ClientInfo {
        ClientInfo {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
// ClientAggregate
pub struct Client{
    client_repository: Box<dyn ClientRepository> 
}

impl Client{
    pub fn new(client_repository: impl ClientRepository + 'static) -> Client{
        Client {
            client_repository: Box::new(client_repository)
        }
    }

    pub fn create_client(&self, name: &str) {
        let id = self.client_repository.next_identity();
        let client = ClientInfo::new(&id, name);

        self.client_repository.save(client);
    }

    pub fn get_by_id(&self, client_id: &str) -> Result<ClientInfo, String>{
        return self.client_repository.by_id(client_id);
    }

    pub fn all_clients(&self) -> Vec<ClientInfo> {
        return self.client_repository.all();
    }
}
