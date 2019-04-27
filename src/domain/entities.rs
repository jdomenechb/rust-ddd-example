#[derive(Debug)]
pub struct Client {
    name: String
}

impl Client {
    pub fn new(name: String) -> Client {
        Client {
            name
        }
    }
}
