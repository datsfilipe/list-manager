const _PATH: &str = "/home/user/.config/list-manager";
use crate::manager;
use crate::list::Status;


pub fn help () {
    println!("Usage: lm [command] [args]");
    println!("Commands:");
    println!("  add [list] - Add a new list");
    println!("  remove [list] - Remove a list");
    println!("  list - List all lists");
    println!("  list [list] - List all items in a list");
    println!("  list [list] [status] - List all items in a list by status");
    println!("  add [list] [item] - Add an item to a list");
    println!("  remove [list] [item] - Remove an item from a list");
    println!("  edit [list] - Edit a list name");
    println!("  edit [list] [item] [content] - Edit an item in a list");
    println!("  get [list] [item] - Get an item from a list");
    println!("  help - Show this help message");
}

pub fn add (manager: &mut manager::Manager, name: &str) {
    manager.add_list(name);
}

pub fn remove (manager: &mut manager::Manager, name: &str) {
    manager.remove_list(name);
}

pub fn list (manager: &manager::Manager) -> Vec<&str> {
    let mut lists: Vec<&str> = Vec::new();
    for list in manager.get_lists() {
        lists.push(&list.name);
    }
    lists
}

pub fn edit_list (manager: &mut manager::Manager, list_name: &str, new_name: &str) {
    let list = manager.get_mut_list(list_name).unwrap();
    list.name = new_name.to_string();
}

pub fn add_item (manager: &mut manager::Manager, list_name: &str, item_content: &str) {
    manager.add_item(list_name, item_content);
}

pub fn remove_item (manager: &mut manager::Manager, list_name: &str, item_content: &str) {
    manager.remove_item(list_name, item_content);
}

pub fn edit_item (manager: &mut manager::Manager, list_name: &str, item_content: &str, new_content: &str) {
    manager.edit_item(list_name, item_content, new_content);
}

pub fn get_item<'a> (manager: &'a manager::Manager, list_name: &str, item_content: &str) -> Option<&'a str> {
    let list = manager.get_list(list_name).unwrap();
    let item = list.items.iter().find(|item| item.content == item_content).unwrap();
    Some(&item.content)
}

pub fn get_list_by_status<'a> (manager: &'a manager::Manager, list_name: &str, status: Status) -> Vec<&'a str> {
    manager.get_list_by_status(list_name, status).unwrap().iter().map(|item| item.content.as_str()).collect()
}
