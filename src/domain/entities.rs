#[derive(Clone)]
pub struct Client {
    id: String,
    name: String,
    location: String,
}

impl Client {
    pub fn new(id: &str, name: &str, location: &str) -> Client {
        Client {
            id: id.to_string(),
            name: name.to_string(),
            location: location.to_string(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn location(&self) -> String {
        self.location.clone()
    }
}
