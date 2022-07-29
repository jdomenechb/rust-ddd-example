#[readonly::make]
pub struct CreateClientUseCaseRequest {
    pub name: String,
    pub location: String,
}

impl CreateClientUseCaseRequest {
    pub fn new(name: &str, location: &str) -> CreateClientUseCaseRequest {
        CreateClientUseCaseRequest {
            name: name.to_string(),
            location: location.to_string(),
        }
    }
}

#[readonly::make]
pub struct GetClientUseCaseRequest {
    pub client_id: String,
}

impl GetClientUseCaseRequest {
    pub fn new(client_id: &str) -> GetClientUseCaseRequest {
        GetClientUseCaseRequest {
            client_id: client_id.to_string(),
        }
    }
}
