use domain::repositories::ClientRepository;
use application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use domain::entities::Client;

pub struct CreateClientUseCaseHandler<'a> {
    client_repository: &'a ClientRepository
}

impl<'a> CreateClientUseCaseHandler<'a> {
    pub fn new(client_repository: &ClientRepository) -> CreateClientUseCaseHandler {
        return CreateClientUseCaseHandler {
            client_repository
        }
    }

    pub fn execute(&self, request: CreateClientUseCaseRequest) {
        let id = self.client_repository.next_identity();
        let client = Client::new(id, request.name);

        self.client_repository.save(client);
    }
}

pub struct GetClientUseCaseHandler<'a> {
    client_repository: &'a ClientRepository
}

impl<'a> GetClientUseCaseHandler<'a> {
    pub fn new(client_repository: &ClientRepository) -> GetClientUseCaseHandler {
        return GetClientUseCaseHandler {
            client_repository
        }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<Client, String>{
        return self.client_repository.by_id(String::from(request.client_id));
    }
}