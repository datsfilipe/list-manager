use crate::list;

pub struct Manager {
    pub lists: Vec<list::List>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { lists: Vec::new() }
    }

    pub fn _add_list(&mut self, list: list::List) {
        self.lists.push(list);
    }
}
