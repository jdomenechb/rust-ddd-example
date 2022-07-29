use crate::application::dtos::ClientDto;
use crate::application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use crate::domain::entities::Client;
use crate::domain::repositories::ClientRepository;
use std::rc::Rc;

// -------------------------------------------------------------------------------------------------

pub struct CreateClientUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl CreateClientUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> CreateClientUseCaseHandler {
        CreateClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: CreateClientUseCaseRequest) {
        let id = self.client_repository.next_identity();
        let client = Client::new(
            id.as_str(),
            request.name.as_str(),
            request.location.as_str(),
        );

        self.client_repository.save(client);
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetClientUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl GetClientUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> GetClientUseCaseHandler {
        GetClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<ClientDto, String> {
        let client = self.client_repository.by_id(request.client_id.as_str())?;

        Ok(ClientDto::from_entity(&client))
    }
}

// -------------------------------------------------------------------------------------------------

pub struct GetAllClientsUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl GetAllClientsUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> GetAllClientsUseCaseHandler {
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
