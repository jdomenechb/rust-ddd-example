pub struct CreateClientUseCaseRequest {
    pub name: String
}

impl CreateClientUseCaseRequest {
    pub fn new(name: String) -> CreateClientUseCaseRequest {
        return CreateClientUseCaseRequest {
            name
        };
    }
}

pub struct GetClientUseCaseRequest<'a> {
    pub client_id: &'a str
}

impl<'a> GetClientUseCaseRequest<'a> {
    pub fn new(client_id: &'a str) -> GetClientUseCaseRequest {
        return GetClientUseCaseRequest {
            client_id
        };
    }
}
