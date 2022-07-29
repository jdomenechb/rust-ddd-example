#[readonly::make]
pub struct CreateClientUseCaseRequest {
    pub name: String,
    pub location: String,
}

impl CreateClientUseCaseRequest {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
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
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
        }
    }
}

#[readonly::make]
pub struct EditClientUseCaseRequest {
    pub client_id: String,
    pub name: String,
    pub location: String,
}

impl EditClientUseCaseRequest {
    pub fn new(client_id: &str, name: &str, location: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            name: name.to_string(),
            location: location.to_string(),
        }
    }
}
