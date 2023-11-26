use crate::manager;
use termion::{color, style};

fn print_status(status: bool, message: &str) {
    if status {
        println!("");
        println!("{}{}{}{}", color::Bg(color::Green), color::Fg(color::Black), message, style::Reset);
        println!("");
    } else {
        println!("");
        println!("{}{}{}{}", color::Bg(color::Red), color::Fg(color::Black), message, style::Reset);
        println!("");
    }
}

fn print_commands() {
    let commands = vec![
        ("help", "Show this help message"),
        ("add", "Add a list or an item"),
        ("show", "Show lists"),
        ("delete", "Delete a list or an item"),
        ("get", "Get an item"),
        ("list", "List items"),
    ];

    for (command, description) in commands {
        println!("  {}{}{} - {}", color::Fg(color::Green), command, style::Reset, description);
    }
}

fn print_params(command: &str) {
    match command {
        "add" => {
            let params = vec![
                ("[add] <list> <item>", "Add an item to a list"),
                ("[add] <list>", "Add a list"),
            ];

            for (param, description) in params {
                println!("  {}{}{} - {}", color::Fg(color::Cyan), param, style::Reset, description);
            }
        },
        "delete" => {
            let params = vec![
                ("[delete] <list> <item>", "Delete an item from a list"),
                ("[delete] <list>", "Delete a list"),
            ];

            for (param, description) in params {
                println!("  {}{}{} - {}", color::Fg(color::Cyan), param, style::Reset, description);
            }
        },
        "get" => {
            let params = vec![
                ("[get] <list> <item>", "Get an item from a list"),
            ];

            for (param, description) in params {
                println!("  {}{}{} - {}", color::Fg(color::Cyan), param, style::Reset, description);
            }
        },
        "list" => {
            let params = vec![
                ("[list] <list>", "List items from a list"),
            ];

            for (param, description) in params {
                println!("  {}{}{} - {}", color::Fg(color::Cyan), param, style::Reset, description);
            }
        },
        _ => {},
    }
}

pub fn help() {
    println!("Save and manage lists from the command line");
    println!("");
    println!("{}{}{}{}", color::Fg(color::Green), style::Bold, "COMMANDS:", style::Reset);
    print_commands();
    println!("");
    println!("{}{}{}{}", color::Fg(color::Green), style::Bold, "PARAMS:", style::Reset);
    print_params("add");
    print_params("delete");
    print_params("get");
    print_params("list");
    println!("");
    println!("{}~The other commands don't have any params{}", style::Italic, style::Reset);
    println!("");
}

pub fn add(manager: &mut manager::Manager, args: &[String]) {
    match args.len() {
        1 => {
            manager.db.add_list(&args[0]);

            print_status(true, " List added ");

            let lists = manager.db.show_lists();

            for index in 0..lists.len() {
                println!("  {}{}{}{}", color::Fg(color::Cyan), (index + 1).to_string() + ". ", style::Reset, lists[index]);
            }
        },
        2 => {

        },
        _ => {},
    }
}

pub fn delete(_manager: &mut manager::Manager, args: &[String]) {
    println!("{}", args[0]);
}

pub fn list(_manager: &manager::Manager, args: &[String]) {
    println!("{}", args[0]);
}

pub fn get(_manager: &manager::Manager, args: &[String]) {
    println!("{}", args[0]);
}

pub fn show(_manager: &manager::Manager) {
    println!("");
}
