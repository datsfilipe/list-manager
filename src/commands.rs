use crate::{manager, list::Status};
use textwrap::{fill,Options};
use colored::Colorize;

fn print_commands() {
    let commands = vec![
        ("add", "Add a list or an item"),
        ("remove", "Remove a list or an item"),
        ("edit", "Edit a list or an item"),
        ("list", "List lists or items"),
        ("help", "Show this help message")
    ];
    for (command, description) in commands {
        println!("  {}: {}", command, fill(description, Options::new(40)));
    }
}

fn print_params(command: &str) {
    match command {
        "add" => {
            println!("  add [list_name] [item_content]");
        },
        "remove" => {
            println!("  remove [list_name] [item_content]");
        },
        "edit" => {
            println!("  edit [list_name] [new_name]");
            println!("  edit [list_name] [item_content] [new_content]");
            println!("  edit [list_name] [item_content] [new_status] (todo, doing, done)");
        },
        "list" => {
            println!("  list");
            println!("  list [list_name]");
            println!("  list [list_name] [status] (list items by status)");
        },
        _ => println!("Invalid command"),
    }
}

pub fn help() {
    println!("{}", "Usage: list [command] [params]".bold().red());
    println!("");
    println!("{}", "Commands:".bold().red());
    print_commands();
    println!("");
    println!("{}", "Params:".bold().red());
    print_params("add");
    print_params("remove");
    print_params("edit");
    print_params("list");
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
