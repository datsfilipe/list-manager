use rusqlite::Connection;
use std::fs;

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
                list_id INTEGER NOT NULL,
                FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE
            )",
            (),
        ).unwrap();

        Database { conn }
    }
}
