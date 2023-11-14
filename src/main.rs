mod args;
mod list;
mod manager;
mod commands;
mod database;

use std::env;

fn main() {
    let mut _manager = manager::Manager::new();

    let args: Vec<String> = env::args().collect();
    if !args::validate_input(&args) {
        return;
    }

    println!("args: {:?}", args);

    match &args[1][..] {
        "help" => commands::help(),
        "add" => commands::add(&mut _manager, &args),
        "remove" => commands::remove(&mut _manager, &args),
        "edit" => commands::edit(&mut _manager, &args),
        "list" => commands::list(&mut _manager, &args),
        _ => println!("Invalid command"),
    }
}
