use rusqlite::Connection;
use std::{env, fs};
use uuid::Uuid;

fn get_path() -> String {
    let xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", env::var("HOME").unwrap()));
    format!("{}/list-manager", xdg_config_home)
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Database {
        fs::create_dir_all(get_path()).unwrap();
        let conn = Connection::open(format!("{}/lists.db", get_path())).unwrap();

        conn.execute("PRAGMA synchronous = NORMAL", ()).unwrap();
        conn.execute("PRAGMA temp_store = MEMORY", ()).unwrap();

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

        conn.execute(
            "CREATE INDEX IF NOT EXISTS item_content ON items (content)",
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

    pub fn list_items(&self, list_name: &str) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT id FROM lists WHERE name = ?1").unwrap();
        let query = stmt.query_map([list_name], |row| {
            let id: [u8; 16] = row.get(0)?;

            Ok(id)
        }).unwrap();

        let list_id: [u8; 16] = query.last().unwrap().unwrap();

        let mut stmt = self.conn.prepare("SELECT content FROM items WHERE list_id = ?1").unwrap();
        let items = stmt.query_map([list_id], |row| {
            let content: String = row.get(0)?;

            Ok(content)
        }).unwrap();

        let result: Vec<String> = items.map(|item| item.unwrap()).collect();
        
        result
    }

    pub fn delete_list(&self, list_name: &str) {
        let mut stmt = self.conn.prepare("SELECT id FROM lists WHERE name = ?1").unwrap();
        let query = stmt.query_map([list_name], |row| {
            let id: [u8; 16] = row.get(0)?;

            Ok(id)
        }).unwrap();

        let list_id: [u8; 16] = query.last().unwrap().unwrap();

        self.conn.execute(
            "DELETE FROM lists WHERE id = ?1",
            &[&list_id],
        ).unwrap();
    }

    pub fn delete_item(&self, list_name: &str, item_name: &str) {
        let mut stmt = self.conn.prepare("SELECT id FROM lists WHERE name = ?1").unwrap();
        let query = stmt.query_map([list_name], |row| {
            let id: [u8; 16] = row.get(0)?;

            Ok(id)
        }).unwrap();

        let list_id: [u8; 16] = query.last().unwrap().unwrap();

        let mut stmt = self.conn.prepare("SELECT id FROM items WHERE list_id = ?1 AND content = ?2").unwrap();
        let query = stmt.query_map((list_id, item_name), |row| {
            let id: [u8; 16] = row.get(0)?;

            Ok(id)
        }).unwrap();

        let item_id: [u8; 16] = query.last().unwrap().unwrap();

        self.conn.execute(
            "DELETE FROM items WHERE id = ?1",
            &[&item_id],
        ).unwrap();
    }

    pub fn get_items(&self, item_name: &str) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT content FROM items WHERE content LIKE '%' || ?1 || '%'").unwrap();
        let items = stmt.query_map([item_name], |row| {
            let content: String = row.get(0)?;

            Ok(content)
        }).unwrap();

        let result: Vec<String> = items.map(|item| item.unwrap()).collect();
        
        result
    }
}
