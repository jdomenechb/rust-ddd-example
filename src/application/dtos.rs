use crate::domain::entities::Client;

#[readonly::make]
#[derive(Debug, PartialEq, Eq)]
pub struct ClientDto {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl ClientDto {
    pub fn from_entity(client: &Client) -> Self {
        Self {
            id: client.id.clone(),
            name: client.name.clone(),
            location: client.location.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DtoList<T>(pub Vec<T>);

#[cfg(test)]
mod test {
    use crate::application::dtos::ClientDto;
    use crate::domain::entities::Client;

    #[test]
    fn create_client_dto_from_entity() {
        let client = Client::new("ID", "Name", "Location");
        let client_dto = ClientDto::from_entity(&client);

        assert_eq!(client_dto.id, "ID");
        assert_eq!(client_dto.name, "Name");
        assert_eq!(client_dto.location, "Location");
    }
}
