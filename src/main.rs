mod manager;
mod list;
mod commands;
mod database;

fn main() {
    let mut manager = manager::Manager::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        commands::help();
        return;
    }

    let command = &args[1];
    if command == "help" {
        commands::help();
        return;
    }

    if command == "add" {
    }

    commands::help();
}
