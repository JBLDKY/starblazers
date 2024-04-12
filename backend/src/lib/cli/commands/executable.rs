use crate::database::db::DatabaseClient;

pub trait Executable {
    fn execute(&self, db: &mut DatabaseClient);
    fn query(&self) -> &str;
}
