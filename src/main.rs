mod manager;
mod list;
mod commands;

fn main() {
    let mut _manager = manager::Manager::new();

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
}
