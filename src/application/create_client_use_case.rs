/*
use ::domain::repositories::ClientRepository;
use domain::entities::Client;


// --- REQUEST -------------------------------------------------------------------------------------

pub struct Request {
    name: String
}

impl Request {
    pub fn new(name: String) -> Request {
        return Request {
            name
        };
    }
}


// --- HANDLER -------------------------------------------------------------------------------------

pub struct Handler<'a> {
    client_repository: &'a ClientRepository
}

impl<'a> Handler<'a> {
    pub fn new(client_repository: &ClientRepository) -> Handler {
        return Handler {
            client_repository
        }
    }

    pub fn execute(&self, request: Request) {
        let id = self.client_repository.next_identity();
        let client = Client::new(id, request.name);

        self.client_repository.save(client);
    }
}*/
