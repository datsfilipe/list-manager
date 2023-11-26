use rusqlite::Connection;
use std::fs;
use uuid::Uuid;

fn get_path() -> String {
    let user = "dtsf";
    format!("/home/{}/.config/list-manager", user)
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Database {
        fs::create_dir_all(get_path()).unwrap();
        let conn = Connection::open(format!("{}/lists.db", get_path())).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS lists (
                id BLOB PRIMARY KEY,
                name TEXT NOT NULL
            )",
            (),
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id BLOB PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                list_id INTEGER NOT NULL,
                FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE
            )",
            (),
        ).unwrap();

        Database { conn }
    }

    fn generate_uuid() -> [u8; 16] {
        let uuid = Uuid::new_v4();
        *uuid.as_bytes()
    }

    pub fn show_lists(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT * FROM lists").unwrap();
        let lists = stmt.query_map([], |row| {
            let id: [u8; 16] = row.get(0)?;
            let name: String = row.get(1)?;

            Ok((id, name))
        }).unwrap();

        let mut result = Vec::new();

        for list in lists {
            let (_, name) = list.unwrap();
            result.push(name);
        }

        result
    }

    pub fn add_list(&self, name: &str) {
        let uuid = Database::generate_uuid();

        self.conn.execute(
            "INSERT INTO lists (id, name) VALUES (?1, ?2)",
            (&uuid, &name),
        ).unwrap();
    }

    pub fn add_item(&self, list_name: &str, content: &str) {
        let uuid = Database::generate_uuid();

        let mut stmt = self.conn.prepare("SELECT id FROM lists WHERE name = ?1").unwrap();
        let query = stmt.query_map([list_name], |row| {
            let id: [u8; 16] = row.get(0)?;

            Ok(id)
        }).unwrap();

        let list_id: [u8; 16] = query.last().unwrap().unwrap();

        self.conn.execute(
            "INSERT INTO items (id, content, list_id) VALUES (?1, ?2, ?3)",
            (&uuid, &content, &list_id),
        ).unwrap();
    }

}
