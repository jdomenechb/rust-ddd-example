use crate::application::dtos::{ClientDto, DtoList};
use std::fmt::{Display, Formatter};

impl Display for ClientDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client #{}: {}, from {}",
            self.id, self.name, self.location
        )
    }
}

impl Display for DtoList<ClientDto> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No clients");
        }

        let mut to_display = String::new();
        to_display.push_str("Client list\n");
        to_display.push_str("----------------------------------------\n\n");

        for client in &self.0 {
            to_display.push_str(format!("{}\n", client).as_str());
        }

        write!(f, "{}", to_display)
    }
}

#[cfg(test)]
mod test {
    use crate::application::dtos::{ClientDto, DtoList};
    use crate::domain::entities::Client;
    use fake::{Fake, Faker};

    #[test]
    fn client_dto_display() {
        let client = Client::new("ID", "Name", "Location");
        let client_dto = ClientDto::from_entity(&client);

        assert_eq!(client_dto.to_string(), "Client #ID: Name, from Location")
    }

    #[test]
    fn client_dto_list_empty_display() {
        let client_dto_list: DtoList<ClientDto> = DtoList(Vec::new());

        assert_eq!(client_dto_list.to_string(), "No clients")
    }

    #[test]
    fn client_dto_list_with_items_display() {
        let client1: Client = Faker.fake();
        let client2: Client = Faker.fake();

        let dto1 = ClientDto::from_entity(&client1);
        let dto2 = ClientDto::from_entity(&client2);

        let client_dto_list: DtoList<ClientDto> = DtoList(vec![dto1, dto2]);

        let mut expected = "Client list
----------------------------------------

"
        .to_string();

        for dto in &client_dto_list.0 {
            expected.push_str(dto.to_string().as_str());
            expected.push('\n');
        }

        assert_eq!(client_dto_list.to_string(), expected)
    }
}
