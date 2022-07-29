use crate::domain::entities::Client;

#[readonly::make]
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
