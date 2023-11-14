use crate::{manager, list::Status};

pub fn help () {
    println!("Usage: lm [command] [args]");
    println!("Commands:");
    println!("  add [list] - Add a new list");
    println!("  add [list] [item] - Add an item to a list");
    println!("  remove [list] - Remove a list");
    println!("  remove [list] [item] - Remove an item from a list");
    println!("  edit [list] [name] - Edit a list name");
    println!("  edit [list] [item] [content] - Edit an item in a list");
    println!("  edit [list] [item] [status] - Edit an item status in a list");
    println!("  list - List all lists");
    println!("  list [list] - List all items in a list");
    println!("  list [list] [status] - List all items in a list by status");
    println!("  help - Show this help message");
}

pub fn add(manager: &mut manager::Manager, args: &Vec<String>) {
    match args.len() {
        3 => {
            let list_name = &args[2];
            manager.add_list(list_name);
        },
        4 => {
            let list_name = &args[2];
            let item_content = &args[3];
            manager.add_item(list_name, item_content);
        },
        _ => println!("Invalid arguments"),
    }
}

pub fn remove (manager: &mut manager::Manager, args: &Vec<String>) {
    match args.len() {
        3 => {
            let list_name = &args[2];
            manager.remove_list(list_name);
        },
        4 => {
            let list_name = &args[2];
            let item_content = &args[3];
            manager.remove_item(list_name, item_content);
        },
        _ => println!("Invalid arguments"),
    }
}

pub fn edit (manager: &mut manager::Manager, args: &Vec<String>) {
    match args.len() {
        4 => {
            let list_name = &args[2];
            let new_name = &args[3];
            manager.edit_list(list_name, new_name);
        },
        5 => {
            let list_name = &args[2];
            let item_content = &args[3];
            let data = &args[4];

            let list_index = manager.lists.iter().position(|list| &list.name == list_name).unwrap();
            let item = manager.get_item(item_content, list_index);

            if data == "todo" || data == "doing" || data == "done" {
                manager.edit_item(list_name, data, item_content);
            } else {
                manager.edit_item(list_name, &item.status.fmt(), data);
            }
        },
        _ => println!("Invalid arguments"),
    }
}

pub fn list (manager: &manager::Manager, args: &Vec<String>) {
    match args.len() {
        2 => {
            let lists = manager.list_lists();
            for list in lists {
                println!("{}", list);
            }
        },
        3 => {
            let list_name = &args[2];
            let items = manager.list_items(list_name);
            for item in items {
                println!("{}", item.content);
            }
        },
        4 => {
            let list_name = &args[2];
            let status = &args[3];
            let status = Status::from_str(status);
            let items = manager.list_list_by_status(list_name, status);
            for item in items {
                println!("{}", item.content);
            }
        },
        _ => println!("Invalid arguments"),
    }
}
