use crate::domain::entities::Client;

pub trait ClientRepository {
    fn by_id(&self, id: String) -> Result<Client, String>;
    fn save(&self, client: Client);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<Client>;
}