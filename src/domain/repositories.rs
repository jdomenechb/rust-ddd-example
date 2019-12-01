use super::clients::ClientInfo;

pub trait ClientRepository {
    fn by_id(&self, id: &str) -> Result<ClientInfo, String>;
    fn save(&self, client: ClientInfo);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<ClientInfo>;
}