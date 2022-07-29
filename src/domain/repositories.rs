use crate::domain::entities::Client;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait ClientRepository {
    fn by_id(&self, id: &str) -> Result<Client, String>;
    fn save(&self, client: Client);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<Client>;
}
