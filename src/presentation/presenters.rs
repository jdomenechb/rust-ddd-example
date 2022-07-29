use crate::application::dtos::{ClientDto, ClientDtoList};
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

impl Display for ClientDtoList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No clients found");
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
