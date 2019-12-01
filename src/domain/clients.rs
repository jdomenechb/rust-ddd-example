use domain::repositories::ClientRepository;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub id: String,
    pub name: String,
}

impl ClientInfo {
    pub fn new(id: &str, name: &str) -> ClientInfo {
        ClientInfo {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
// Client Aggregate
pub struct Client{
    client_repository: Rc<dyn ClientRepository>
}

impl Client{
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> Client{
        Client {
            client_repository: client_repository
        }
    }

    pub fn create_client(&self, name: &str) {
        let id = self.client_repository.next_identity();
        let client = ClientInfo::new(&id, name);

        self.client_repository.save(client);
    }
}
