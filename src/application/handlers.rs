use infrastructure::domain::repositories::InMemoryClientRepository;
use domain::repositories::ClientRepository;
use domain::entities::Client;

pub struct ClientHandler {
    client_repository: Box<dyn ClientRepository>
}

impl ClientHandler{
    pub fn new() -> ClientHandler {
        let mem_repository = InMemoryClientRepository::new();
        return ClientHandler {
            client_repository: Box::new(mem_repository)
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

    pub fn get_all_clients(&self) -> Vec<Client> {
        return self.client_repository.all();
    }
}