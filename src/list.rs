use std::time::SystemTime;

pub struct Item {
    _content: String,
    _created_at: SystemTime,
}

pub struct List {
    _items: Vec<Item>,
    _name: String,
}

impl List {
    pub fn _new(name: String, items: Vec<Item>) -> List {
        List { _name: name, _items: items }
    }
}
