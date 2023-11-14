use crate::list;
use crate::database;
use chrono::prelude::*;

pub struct Manager {
    pub lists: Vec<list::List>,
    pub db: database::Database,
}

impl Manager {
    pub fn new() -> Manager {
        let db = database::Database::new();

        let mut lists: Vec<list::List> = Vec::new();
        let mut stmt = db.conn.prepare("SELECT * FROM lists").unwrap();
        let list_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let mut stmt = db.conn.prepare("SELECT * FROM items WHERE list_id = ?").unwrap();
            let item_iter = stmt.query_map([id], |row| {
                let content: String = row.get(1).unwrap();
                let created_at: String = row.get(2).unwrap();
                let status: list::Status = list::Status::from_str(&row.get::<_, String>(3).unwrap());
                Ok(list::Item::new(content, created_at, status))
            }).unwrap();
            let items: Vec<list::Item> = item_iter.map(|item| item.unwrap()).collect();
            Ok(list::List::new(name, items))
        }).unwrap();

        for list in list_iter {
            lists.push(list.unwrap());
        }

        stmt.finalize().unwrap();

        Manager {
            lists,
            db,
        }
    }

    pub fn add_list(&mut self, name: &str) {
        self.db.add_list(name);
    }

    pub fn add_item (&mut self, list_name: &str, item_content: &str) {
        let lists = &self.lists;
        let index = lists.iter().position(|list| list.name == list_name).unwrap();
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let item = list::Item::new(item_content.to_string(), created_at, list::Status::Todo);

        println!("item: {:?}", item);

        self.db.add_item(&item.content, &item.created_at, &item.status.fmt(), index.try_into().unwrap());
    }

    pub fn remove_list(&mut self, name: &str) {
        self.db.remove_list(name);
    }

    pub fn remove_item (&mut self, list_name: &str, content: &str) {
        let lists = &self.lists;
        let list_index = lists.iter().position(|list| list.name == list_name).unwrap();
        self.db.remove_item(content, list_index.try_into().unwrap());
    }

    pub fn edit_list(&mut self, old_name: &str, new_name: &str) {
        let index = self.lists.iter().position(|list| list.name == old_name).unwrap();
        self.db.edit_list(index.try_into().unwrap(), new_name);
    }

    pub fn edit_item(&mut self, list_name: &str, new_status: &str, new_content: &str) {
        let lists = &self.lists;
        let list_index = lists.iter().position(|list| list.name == list_name).unwrap();
        self.db.edit_item(list_index.try_into().unwrap(), new_content, new_status);
    }

    pub fn list_items(&self, name: &str) -> Vec<list::Item> {
        self.db.get_list_items(name)
    }

    pub fn list_lists(&self) -> Vec<String> {
        self.db.get_lists()
    }

    pub fn list_list_by_status(&self, name: &str, status: list::Status) -> Vec<list::Item> {
        self.db.get_list_by_status(name, status)
    }

    pub fn get_item(&self, item_content: &str, list_index: usize) -> list::Item {
        self.db.get_item(item_content, list_index.try_into().unwrap()).unwrap()
    }
}
