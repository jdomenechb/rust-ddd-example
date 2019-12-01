use infrastructure::repositories::InMemoryClientRepository;
use domain::repositories::ClientRepository;
use domain::clients::{ClientInfo, Client};
use std::rc::Rc;

pub enum Commands {
    CreateClient(String)
}

pub struct MockApi{
    pub client_command_handler: ClientCommandHandler,
    pub client_repository: Rc<dyn ClientRepository>
}

impl MockApi{
    pub fn new() -> Self {
        let mem_repository =Rc::new(InMemoryClientRepository::new());
        return MockApi {
            client_command_handler: ClientCommandHandler::new(mem_repository.clone()),
            client_repository: mem_repository.clone(),
        }
    }

    pub fn get_by_id(&self, client_id: &str) -> Result<ClientInfo, String>{
        self.client_repository.as_ref().by_id(client_id)
    }

    pub fn all_clients(&self) -> Vec<ClientInfo> {
        self.client_repository.as_ref().all()
    }

    pub fn create_client(&self, name: &str) {
        self.client_command_handler.handle_commands(Commands::CreateClient(name.to_string()));
    }
}

pub struct ClientCommandHandler{
    client: Box<Client>
}

impl ClientCommandHandler{
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> Self {
        let client = Client::new(client_repository);
        return ClientCommandHandler {
            client: Box::new(client)
        }
    }

    pub fn handle_commands(&self, command: Commands) {
        match command {
            Commands::CreateClient(name) => self.client.as_ref().create_client(&name)
        }
    }
}
