#[derive(PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl Status {
    pub fn from_str(status: &str) -> Status {
        match status {
            "todo" => Status::Todo,
            "doing" => Status::Doing,
            "done" => Status::Done,
            _ => panic!("Invalid status"),
        }
    }

    pub fn _fmt(&self) -> String {
        match self {
            Status::Todo => "todo".to_string(),
            Status::Doing => "doing".to_string(),
            Status::Done => "done".to_string(),
        }
    }
}

pub struct Item {
    pub content: String,
    pub created_at: String,
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
    pub fn new(content: String, created_at: String, status: Status) -> Item {
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
