use ::domain::repositories::ClientRepository;
use domain::entities::Client;


// --- REQUEST -------------------------------------------------------------------------------------

pub struct Request<'a> {
    client_id: &'a str
}

impl<'a> Request<'a> {
    pub fn new(client_id: &'a str) -> Request {
        return Request {
            client_id
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

    pub fn execute(&self, request: Request) -> Result<Client, String>{
        return self.client_repository.by_id(String::from(request.client_id));
    }
}