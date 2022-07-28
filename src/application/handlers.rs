use crate::application::dtos::ClientDto;
use crate::application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use crate::domain::entities::Client;
use crate::domain::repositories::ClientRepository;

// -------------------------------------------------------------------------------------------------

pub struct CreateClientUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository,
}

impl<'a> CreateClientUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> CreateClientUseCaseHandler {
        CreateClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: CreateClientUseCaseRequest) {
        let id = self.client_repository.next_identity();
        let client = Client::new(id, request.name);

        self.client_repository.save(client);
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetClientUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository,
}

impl<'a> GetClientUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetClientUseCaseHandler {
        GetClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<ClientDto, String> {
        let client = self
            .client_repository
            .by_id(String::from(request.client_id))?;

        Ok(ClientDto::from_entity(&client))
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetAllClientsUseCaseHandler<'a> {
    client_repository: &'a dyn ClientRepository,
}

impl<'a> GetAllClientsUseCaseHandler<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetAllClientsUseCaseHandler {
        GetAllClientsUseCaseHandler { client_repository }
    }

    pub fn execute(&self) -> Vec<ClientDto> {
        return self
            .client_repository
            .all()
            .iter()
            .map(ClientDto::from_entity)
            .collect();
    }
}
