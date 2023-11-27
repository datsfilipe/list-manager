#[allow(dead_code, unused_imports)]
use crate::{args, manager, commands, database};

#[test]
fn test_validate_input() {
    let args: Vec<String> = vec!["".to_string(), "help".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "show".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "add".to_string(), "test".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "delete".to_string(), "test".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "list".to_string(), "test".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "get".to_string(), "test".to_string()];
    assert!(args::validate_input(&args));

    let args: Vec<String> = vec!["".to_string(), "test".to_string()];
    assert!(!args::validate_input(&args));
}

#[test]
fn test_help() {
    commands::help();
}

#[test]
fn test_show() {
    let manager = manager::Manager::new();
    commands::show(&manager);
}

#[test]
fn test_add() {
    let mut manager = manager::Manager::new();
    let args: Vec<String> = vec!["".to_string(), "add".to_string(), "test".to_string()];
    commands::add(&mut manager, &args[2..]);
}

#[test]
fn test_list() {
    let mut manager = manager::Manager::new();
    let args: Vec<String> = vec!["".to_string(), "list".to_string(), "test".to_string()];
    commands::list(&mut manager, &args[2..]);
}

#[test]
fn test_get() {
    let mut manager = manager::Manager::new();
    let args: Vec<String> = vec!["".to_string(), "get".to_string(), "test".to_string()];
    commands::get(&mut manager, &args[2..]);
}

#[test]
fn test_delete() {
    let mut manager = manager::Manager::new();
    let args: Vec<String> = vec!["".to_string(), "delete".to_string(), "test".to_string()];
    commands::delete(&mut manager, &args[2..]);
}

#[test]
fn test_database() {
    let db = database::Database::new();
    db.show_lists();
}

#[test]
fn test_manager() {
    let manager = manager::Manager::new();
    manager.db.show_lists();
}
