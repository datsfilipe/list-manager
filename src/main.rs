mod args;
mod manager;
mod commands;
mod database;

use std::env;

fn main() {
    let mut manager = manager::Manager::new();

    let args: Vec<String> = env::args().collect();
    if !args::validate_input(&args) {
        return;
    }

    match &args[1][..] {
        "help" => commands::help(),
        "show" => commands::show(&manager),
        "add" => commands::add(&mut manager, &args[2..]),
        "delete" => commands::delete(&mut manager, &args[2..]),
        "list" => commands::list(&mut manager, &args[2..]),
        "get" => commands::get(&mut manager, &args[2..]),
        _ => println!("Urecognized argument: {}", &args[1]),
    }
}
