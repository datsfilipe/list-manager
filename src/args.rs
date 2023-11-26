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
        Message::NoArguments => return false,
        Message::InvalidArguments => return false,
        Message::TooManyArguments => return false,
    }
}

fn verify_args(args: &Vec<String>) -> Message {
    match args.len() {
        1 => Message::NoArguments,
        2 => {
            if args[1] == "help" || args[1] == "show" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        3 => {
            if args[1] == "add" || args[1] == "delete" || args[1] == "list" || args[1] == "get" {
                Message::ValidArguments
            } else {
                Message::InvalidArguments
            }
        },
        4 => {
            if args[1] == "add" || args[1] == "delete" {
                if args[2] == "--list" || args[2] == "--item" {
                    Message::ValidArguments
                } else {
                    Message::InvalidArguments
                }
            } else {
                Message::InvalidArguments
            }
        },
        _ => Message::TooManyArguments,
    }
}
