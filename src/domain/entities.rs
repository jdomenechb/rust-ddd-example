#[derive(Clone)]
pub struct Client {
    id: String,
    name: String,
}

impl Client {
    pub fn new(id: String, name: String) -> Client {
        Client { id, name }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
