use fake::{Dummy, Fake};

#[derive(Clone, Dummy, PartialEq, Eq)]
#[readonly::make]
pub struct Client {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl Client {
    pub fn new(id: &str, name: &str, location: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            location: location.to_string(),
        }
    }

    pub fn edit(&mut self, name: &str, location: &str) {
        self.name = name.to_string();
        self.location = location.to_string();
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::Client;
    use fake::{Fake, Faker};

    #[test]
    fn create_client() {
        let id: String = Faker.fake();
        let name: String = Faker.fake();
        let location: String = Faker.fake();

        let client = Client::new(id.as_str(), name.as_str(), location.as_str());

        assert_eq!(client.id, id.as_str());
        assert_eq!(client.name, name.as_str());
        assert_eq!(client.location, location.as_str());
    }

    #[test]
    fn edit_client() {
        let mut client: Client = Faker.fake();
        let id = client.id.clone();

        let new_name: String = Faker.fake();
        let new_location: String = Faker.fake();

        assert_ne!(client.name, new_name);
        assert_ne!(client.location, new_location);

        client.edit(new_name.as_str(), new_location.as_str());

        assert_eq!(client.id, id);
        assert_eq!(client.name, new_name);
        assert_eq!(client.location, new_location);
    }
}
