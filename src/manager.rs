use crate::list;
use crate::database;

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

    pub fn add_list(&mut self, name: &str) -> &list::List {
    }

    // pub fn _add_item (&mut self, list_name: &str, item_content: &str) {
    // }
    //
    // pub fn _remove_item (&mut self, list_name: &str, index: usize) {
    // }
    //
    // pub fn _edit_item (&mut self, index: usize, new_content: &str, list_name: &str) {
    // }
    //
    // pub fn _get_list_items(&self, name: &str) -> Option<Vec<list::Item>> {
    // }
    //
    // pub fn _get_mut_list(&mut self, name: &str) -> Option<&mut list::List> {
    // }
    //
    // pub fn _get_list_by_status(&self, name: &str, status: list::Status) -> Option<Vec<list::Item>> {
    // }
    //
    // pub fn _get_lists(&self) -> &Vec<list::List> {
    // }
    //
    // pub fn _remove_list(&mut self, index: usize) {
    // }
}
