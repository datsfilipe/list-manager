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
        let list = list::List::new(name.to_string(), Vec::new());
        self.lists.push(list);

        let mut stmt = self.db.conn.prepare("INSERT INTO lists (name) VALUES (?)").unwrap();
        stmt.execute([name]).unwrap();
        stmt.finalize().unwrap();

        self.get_list(name).unwrap()
    }

    pub fn add_item (&mut self, list_name: &str, item_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        let item = list::Item::new(item_content.to_string(), "tmp".to_string(), list::Status::Todo);
        list.items.push(item);

        let index = self.lists.iter().position(|list| list.name == list_name).unwrap();

        let mut stmt = self.db.conn.prepare("INSERT INTO items (content, created_at, status, list_id) VALUES (?, ?, ?, ?)").unwrap();
        stmt.execute([item_content, "tmp", "todo", format!("{}", index).as_str()]).unwrap();
        stmt.finalize().unwrap();
    }

    pub fn remove_item (&mut self, list_name: &str, item_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        list.items.retain(|item| item.content != item_content);

        let mut stmt = self.db.conn.prepare("DELETE FROM items WHERE content = ?").unwrap();
        stmt.execute([item_content]).unwrap();
        stmt.finalize().unwrap();
    }

    pub fn edit_item (&mut self, list_name: &str, item_content: &str, new_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        let item = list.items.iter_mut().find(|item| item.content == item_content).unwrap();
        item.content = new_content.to_string();

        let mut stmt = self.db.conn.prepare("UPDATE items SET content = ? WHERE content = ?").unwrap();
        stmt.execute([new_content, item_content]).unwrap();
        stmt.finalize().unwrap();
    }

    pub fn get_list(&self, name: &str) -> Option<&list::List> {
        self.lists.iter().find(|list| list.name == name)
    }

    pub fn get_mut_list(&mut self, name: &str) -> Option<&mut list::List> {
        self.lists.iter_mut().find(|list| list.name == name)
    }

    pub fn get_list_by_status(&self, name: &str, status: list::Status) -> Option<Vec<&list::Item>> {
        self.get_list(name).map(|list| list.get_list_by_status(status))
    }

    pub fn get_lists(&self) -> &Vec<list::List> {
        &self.lists
    }

    pub fn remove_list(&mut self, name: &str) {
        self.lists.retain(|list| list.name != name);

        let mut stmt = self.db.conn.prepare("DELETE FROM lists WHERE name = ?").unwrap();
        stmt.execute([name]).unwrap();
        stmt.finalize().unwrap();
    }
}
