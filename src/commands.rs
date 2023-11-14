use crate::manager;
use crate::list::{Status, Item};

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
    println!("  edit [list] [name] - Edit a list name");
    println!("  edit [list] [item] [content] - Edit an item in a list");
    println!("  edit [list] [item] [status] - Edit an item status in a list");
    println!("  get [list] [item] - Get an item from a list");
    println!("  help - Show this help message");
}

// add
pub fn _add (manager: &mut manager::Manager, name: &str) {
}

// pub fn _add_item (manager: &mut manager::Manager, list_name: &str, item_content: &str) {
// }
//
// pub fn _remove (manager: &mut manager::Manager, name: &str) {
// }
//
// pub fn _list (manager: &manager::Manager) -> Vec<&str> {
// }
//
// pub fn _edit_list (manager: &mut manager::Manager, list_name: &str, new_name: &str) {
// }
//
// pub fn _remove_item (manager: &mut manager::Manager, list_name: &str, item_content: &str) {
// }
//
// pub fn _edit_item (manager: &mut manager::Manager, list_name: &str, item_content: &str, new_content: &str) {
// }
//
// pub fn _get_item<'a> (manager: &'a manager::Manager, list_name: &str, item_content: &str) -> Option<&'a str> {
// }
//
// pub fn _get_list_items (manager: &manager::Manager, list_name: &str) -> Vec<Item> {
// }
//
// pub fn _get_list_by_status (manager: &manager::Manager, list_name: &str, status: Status) -> Vec<Item> {
// }
