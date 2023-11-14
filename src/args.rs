enum Message {
    ValidArguments,
    InvalidArguments,
    TooManyArguments,
    NoArguments,
}

pub fn validate_input(args: &Vec<String>) -> bool {
    let message = verify_args(args);
    match message {
        Message::ValidArguments => return true,
        Message::NoArguments => return true,
        Message::InvalidArguments => return false,
        Message::TooManyArguments => return false,
    }
}

fn verify_args(args: &Vec<String>) -> Message {
    match args.len() {
        1 => Message::NoArguments,
        2 => {
            if args[1] == "help" || args[1] == "list" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        3 => {
            if args[1] == "add" || args[1] == "remove" || args[1] == "edit" || args[1] == "list" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        4 => {
            if args[1] == "add" || args[1] == "remove" || args[1] == "edit" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        5 => {
            if args[1] == "edit" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        _ => Message::TooManyArguments,
    }
}
