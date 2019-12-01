use infrastructure::repositories::InMemoryClientRepository;
use domain::clients::{ClientInfo, Client};

pub struct ClientHandler{
    client_agg: Client}

impl ClientHandler{
    pub fn new() -> Self {
        let mem_repository = InMemoryClientRepository::new();
        let client_agg = Client::new(mem_repository);
        return ClientHandler {
            client_agg: client_agg
        }
    }

    pub fn create_client(&self, name: &str) {
        self.client_agg.create_client(name);
    }

    pub fn get_by_id(&self, client_id: &str) -> Result<ClientInfo, String>{
        self.client_agg.get_by_id(client_id)
    }

    pub fn all_clients(&self) -> Vec<ClientInfo> {
        self.client_agg.all_clients()
    }
}
