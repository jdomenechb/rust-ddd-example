use crate::domain::repositories::ClientRepository;
use crate::application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use crate::domain::entities::Client;

// -------------------------------------------------------------------------------------------------

pub struct CreateClientUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository
}

impl<'a> CreateClientUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> CreateClientUseCaseHandler {
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


// -------------------------------------------------------------------------------------------------

pub struct GetClientUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository
}

impl<'a> GetClientUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetClientUseCaseHandler {
        return GetClientUseCaseHandler {
            client_repository
        }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<Client, String>{
        return self.client_repository.by_id(String::from(request.client_id));
    }
}


// -------------------------------------------------------------------------------------------------

pub struct GetAllClientsUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository
}

impl<'a> GetAllClientsUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetAllClientsUseCaseHandler {
        return GetAllClientsUseCaseHandler {
            client_repository
        }
    }

    pub fn execute(&self) -> Vec<Client> {
        return self.client_repository.all();
    }
}