use crate::application::dtos::ClientDto;
use std::fmt::{Display, Formatter};

impl Display for ClientDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client #{}: {}", self.id, self.name)
    }
}
