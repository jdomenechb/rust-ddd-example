#[derive(Debug)]
#[derive(Clone)]
pub struct Client {
    id: String,
    name: String
}

impl Client {
    pub fn new(id: &str, name: &str) -> Client {
        Client {
            id: id.to_string(),
            name: name.to_string()
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}
