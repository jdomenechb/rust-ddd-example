use fake::{Dummy, Fake};

#[derive(Clone, Dummy)]
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
}
