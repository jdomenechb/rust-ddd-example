use super::entities::Client;

pub trait ClientRepository {
    fn by_id(&self, id: &str) -> Result<&Client, String>;
    fn save(&self, client: Client);
    fn next_identity(&self) -> String;
}