const _PATH: &str = "/home/user/.config/list-manager";

pub fn help () {
    println!("Usage: lm [command] [args]");
    println!("Commands:");
    println!("  add [list] - Add a new list");
    println!("  remove [list] - Remove a list");
    println!("  list - List all lists");
    println!("  list [list] - List all items in a list");
    println!("  add [list] [item] - Add an item to a list");
    println!("  remove [list] [item] - Remove an item from a list");
    println!("  edit [list] [item] [content] - Edit an item in a list");
    println!("  get [list] [item] - Get an item from a list");
    println!("  help - Show this help message");
}
