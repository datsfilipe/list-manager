use rusqlite::Connection;
use std::fs;

const PATH: &str = "/home/dtsf/.config/list-manager/lists.db";

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Database {
        fs::create_dir_all("/home/dtsf/.config/list-manager").unwrap();
        let conn = Connection::open(PATH).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS lists (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            (),
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                status TEXT NOT NULL,
                list_id INTEGER NOT NULL,
                FOREIGN KEY (list_id) REFERENCES lists (id)
            )",
            (),
        ).unwrap();

        Database { conn }
    }
}
