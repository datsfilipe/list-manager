use rusqlite::Connection;
use crate::list::{Status, Item};
use users::{get_user_by_uid, get_current_uid};
use std::fs;

fn get_path() -> String {
    let uid = get_current_uid();
    let user = get_user_by_uid(uid).unwrap();
    format!("/home/{}/.config/list-manager", user.name().to_str().unwrap())
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
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            (),
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                status TEXT NOT NULL,
                list_id INTEGER NOT NULL,
                FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE
            )",
            (),
        ).unwrap();

        Database { conn }
    }

    pub fn add_list(&self, name: &str) {
        self.conn.execute(
            "INSERT INTO lists (name) VALUES (?1)",
            &[name],
        ).unwrap();
    }

    pub fn add_item(&self, content: &str, created_at: &str, status: &str, list_id: i32) {
        let id = list_id + 1;
        self.conn.execute(
            "INSERT INTO items (content, created_at, status, list_id) VALUES (?1, ?2, ?3, ?4)",
            &[content, created_at, status, &id.to_string()],
        ).unwrap();
    }

    pub fn remove_list(&self, name: &str) {
        self.conn.execute(
            "DELETE FROM lists WHERE name = ?1",
            &[name],
        ).unwrap();
    }

    pub fn remove_item(&self, content: &str, list_id: i32) {
        let id = list_id + 1;
        self.conn.execute(
            "DELETE FROM items WHERE content = ?1 AND list_id = ?2",
            &[content, &id.to_string()],
        ).unwrap();
    }

    pub fn edit_list(&self, index: usize, new_name: &str) {
        let id = index + 1;
        self.conn.execute(
            "UPDATE lists SET name = ?1 WHERE id = ?2",
            &[new_name, &id.to_string()],
        ).unwrap();
    }

    pub fn edit_item(&self, index: usize, new_content: &str, new_status: &str) {
        let id = index + 1;
        self.conn.execute(
            "UPDATE items SET content = ?1, status = ?2 WHERE id = ?3",
            &[new_content, new_status, &id.to_string()],
        ).unwrap();
    }

    pub fn get_list_items(&self, name: &str) -> Vec<Item> {
        let mut stmt = self.conn.prepare("SELECT * FROM lists WHERE name = ?").unwrap();
        let list_iter = stmt.query_map([name], |row| {
            let id: i32 = row.get(0).unwrap();
            let mut stmt = self.conn.prepare("SELECT * FROM items WHERE list_id = ?").unwrap();
            let item_iter = stmt.query_map([id], |row| {
                let content: String = row.get(1).unwrap();
                let created_at: String = row.get(2).unwrap();
                let status: Status = Status::from_str(&row.get::<_, String>(3).unwrap());
                Ok(Item::new(content, created_at, status))
            }).unwrap();
            let items: Vec<Item> = item_iter.map(|item| item.unwrap()).collect();
            Ok(items)
        }).unwrap();

        let mut items: Vec<Item> = Vec::new();
        for list in list_iter {
            items = list.unwrap();
        }

        stmt.finalize().unwrap();

        items
    }

    pub fn get_list_by_status(&self, name: &str, status: Status) -> Vec<Item> {
        let mut stmt = self.conn.prepare("SELECT * FROM lists WHERE name = ?").unwrap();
        let list_iter = stmt.query_map([name], |row| {
            let id: i32 = row.get(0).unwrap();
            let mut stmt = self.conn.prepare("SELECT * FROM items WHERE list_id = ? AND status = ?").unwrap();
            let item_iter = stmt.query_map((id, &status.fmt()), |row| {
                let content: String = row.get(1).unwrap();
                let created_at: String = row.get(2).unwrap();
                let status: Status = Status::from_str(&row.get::<_, String>(3).unwrap());
                Ok(Item::new(content, created_at, status))
            }).unwrap();
            let items: Vec<Item> = item_iter.map(|item| item.unwrap()).collect();
            Ok(items)
        }).unwrap();

        let mut items: Vec<Item> = Vec::new();
        for list in list_iter {
            items = list.unwrap();
        }

        stmt.finalize().unwrap();

        items
    }

    pub fn get_lists(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT * FROM lists").unwrap();
        let list_iter = stmt.query_map([], |row| {
            let name: String = row.get(1).unwrap();
            Ok(name)
        }).unwrap();

        let mut lists: Vec<String> = Vec::new();
        for list in list_iter {
            lists.push(list.unwrap());
        }

        stmt.finalize().unwrap();

        lists
    }

    pub fn get_item(&self, content: &str, list_id: i32) -> Option<Item> {
        let id = list_id + 1;
        let mut stmt = self.conn.prepare("SELECT * FROM items WHERE content = ? AND list_id = ?").unwrap();
        let item_iter = stmt.query_map([content, &id.to_string()], |row| {
            let content: String = row.get(1).unwrap();
            let created_at: String = row.get(2).unwrap();
            let status: Status = Status::from_str(&row.get::<_, String>(3).unwrap());
            Ok(Item::new(content, created_at, status))
        }).unwrap();

        let mut item: Option<Item> = None;
        for i in item_iter {
            item = Some(i.unwrap());
        }

        stmt.finalize().unwrap();

        item
    }
}
