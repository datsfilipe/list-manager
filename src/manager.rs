use crate::list;

pub struct Manager {
    pub lists: Vec<list::List>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { lists: Vec::new() }
    }

    pub fn add_list(&mut self, name: &str) -> &list::List {
        let list = list::List::new(name.to_string(), Vec::new());
        self.lists.push(list);
        self.get_list(name).unwrap()
    }

    pub fn add_item (&mut self, list_name: &str, item_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        let item = list::Item::new(item_content.to_string(), std::time::SystemTime::now(), list::Status::Todo);
        list.items.push(item);
    }

    pub fn remove_item (&mut self, list_name: &str, item_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        list.items.retain(|item| item.content != item_content);
    }

    pub fn edit_item (&mut self, list_name: &str, item_content: &str, new_content: &str) {
        let list = self.get_mut_list(list_name).unwrap();
        let item = list.items.iter_mut().find(|item| item.content == item_content).unwrap();
        item.content = new_content.to_string();
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
    }
}
