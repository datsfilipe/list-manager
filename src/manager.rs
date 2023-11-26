use crate::database;

pub struct Manager {
    pub db: database::Database,
}

impl Manager {
    pub fn new() -> Manager {
        let db = database::Database::new();

        Manager { db }
    }
}
