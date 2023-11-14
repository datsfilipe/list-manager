mod manager;
mod list;
mod commands;
mod database;

fn main() {
    let mut manager = manager::Manager::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        commands::help();
    }

    let command = &args[1];
    if command == "help" {
        commands::help();
    }

    if command == "add" {
        if args.len() == 3 {
            commands::add(&mut manager, &args[2]);
        }
        if args.len() == 4 {
            commands::add_item(&mut manager, &args[2], &args[3]);
        }
    }

    if command == "remove" {
        if args.len() == 3 {
            commands::remove(&mut manager, &args[2]);
        }
        if args.len() == 4 {
            commands::remove_item(&mut manager, &args[2], &args[3]);
        }
    }

    if command == "list" {
        if args.len() == 2 {
            let lists = commands::list(&manager);
            for list in lists {
                println!("{}", list);
            }
        }
        if args.len() == 3 {
            let list = manager.get_list(&args[2]).unwrap();
            for item in list.get_list() {
                println!("{}", item.get_content());
            }
        }
        if args.len() == 4 {
            let status = list::Status::from_str(&args[3]);
            commands::get_list_by_status(&manager, &args[2], status);
        }
    }

    if command == "edit" {
        if args.len() == 4 {
            commands::edit_list(&mut manager, &args[2], &args[3]);
        }
        if args.len() == 5 {
            commands::edit_item(&mut manager, &args[2], &args[3], &args[4]);
        }
    }  

    if command == "get" {
        if args.len() == 4 {
            let item = commands::get_item(&manager, &args[2], &args[3]).unwrap();
            println!("{}", item);
        }
    }

    let lists = commands::list(&manager);
    for list in lists {
        println!("{}", list);
    }

    commands::help();
}
