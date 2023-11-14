use std::time::SystemTime;

#[derive(PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

pub struct Item {
    pub content: String,
    pub created_at: SystemTime,
    pub status: Status,
}

pub struct List {
    pub items: Vec<Item>,
    pub name: String,
}

impl List {
    pub fn new(name: String, items: Vec<Item>) -> List {
        List { name, items }
    }

    pub fn get_list(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn get_list_by_status(&self, status: Status) -> Vec<&Item> {
        self.items.iter().filter(|item| item.status == status).collect()
    }
}

impl Item {
    pub fn new(content: String, created_at: SystemTime, status: Status) -> Item {
        Item {
            content,
            created_at,
            status,
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}
