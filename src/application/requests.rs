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
pub struct GetClientUseCaseRequest<'a> {
    pub client_id: &'a str,
}

impl<'a> GetClientUseCaseRequest<'a> {
    pub fn new(client_id: &'a str) -> GetClientUseCaseRequest {
        GetClientUseCaseRequest { client_id }
    }
}
