#[derive(Clone)]
#[readonly::make]
pub struct Client {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl Client {
    pub fn new(id: &str, name: &str, location: &str) -> Client {
        Client {
            id: id.to_string(),
            name: name.to_string(),
            location: location.to_string(),
        }
    }
}
